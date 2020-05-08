use super::{GameEvent, Game, Model, Spatial, Lighting};

use events_handling::DevicesState;

use graphics::
{
    glium::glutin::event_loop::EventLoopProxy,
    Similarity,
    Camera,
    Scene,
    Graphical,
    Frame,
    RessourcesHolder
};

use imgui::{Ui, Context};
use imgui_glium_renderer::Renderer;

use base::EngineError;

use std::collections::HashMap;

use rayon::iter::
{
    ParallelIterator,
    IntoParallelIterator
};

use specs::
{
    World,
    Dispatcher,
    WorldExt,
    join::Join,
    join::ParJoin
};

/**
A GameState contains a SPECS world, and can be stacked and popped out of the GameStateStack
GameStates, in the game, have a logic to be run and their rendering to be done.
*/
pub struct GameState
{
    pub name: String,
    pub scene: Scene, // to be removed
    gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    render_behavior: RenderBehavior,
    logic_behavior: LogicBehavior,
    proxy: EventLoopProxy<GameEvent>, // to be removed (maybe (maybe not))

    pub world: World,
    dispatcher: Dispatcher<'static, 'static> // game states never die
}



impl GameState
{

    /**
    Returns a GameState, with some ressources registered in its SPECS World
    Ressources registered for now:
    - EventSender
     */
    fn new(name: String,
           scene: Scene,
           render_behavior: RenderBehavior,
           logic_behavior: LogicBehavior,
           gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
           proxy: EventLoopProxy<GameEvent>,
	   init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>),
	   ressources: &mut RessourcesHolder
    ) -> Self
    {
	let mut world = World::new();

	world.insert(EventSender::new());


	
	let (world, dispatcher) = init(world, ressources);
        Self
        {
            name: name,
            scene: scene,
            render_behavior: render_behavior,
            gui: gui,
            logic_behavior: logic_behavior,
            proxy: proxy,
	    world: world,
	    dispatcher: dispatcher
        }
    }

    /// Build a GameState from a ProtoState
    pub fn from_proto(
        game: &mut Game,
        proto: &ProtoState) -> Result<Self, EngineError>
    {
        Ok(Self::new(proto.name.clone(),
                     (proto.scene_builder)(game)?,
                     proto.render_behavior,
                     proto.logic_behavior,
                     proto.run_gui,
                     game.event_loop_proxy.clone(),
		     proto.init,
		     &mut game.ressources))
    }

    /// Used to send a GameEvent to the EventLoop through the proxy
    pub fn send_event(&self, user_event: GameEvent)
    {
        match self.proxy.send_event(user_event)
        {
            Err(_) => panic!("Cannot send user event: Event Loop terminated"),
            _ => ()
        }
    }

    /// Call send() on the EventSender stored in the World, sending all the collected event to the EventLoop
    pub fn send_events(&self)
    {
	let mut sender = self.world.write_resource::<EventSender>();
	sender.send(&self.proxy);
    }

    /// Builds the graphical scene of the GameState (used before rendering)
    pub fn update_scene(&mut self)
    {
	let models_storage = self.world.read_storage::<Model>();
	let light_storage = self.world.read_storage::<Lighting>();
	let spatial_storage = self.world.read_storage::<Spatial>();

	// we regroup the objects of the scene into instances of the same graphical object
	// it is done in parallel to improve the performances
	let instances = (&models_storage, &spatial_storage).par_join()
	    .fold(|| HashMap::new(), |mut instances, (Model(obj_handle), Spatial{pos, rot, scale})|
		  {
		      let similarity = Similarity::new(*pos, *rot, *scale);
		      match instances.get_mut(obj_handle)
		      {
			  None =>
			  {
			      instances.insert(obj_handle, vec![similarity]);
			  },
			  Some(v) =>
			  {
			      v.push(similarity);
			  }
		      };
		      instances
		  })
	    .reduce( // the part where we regroup the results of the differents threadsx
		|| HashMap::new(),
		|mut total, part|
		{
		    part.into_iter()
			.for_each(
			    |(obj_handle, mut vect)|
			    {
				match total.get_mut(obj_handle)
				{
				    None =>
				    {
					total.insert(obj_handle, vect);
				    },
				    Some(v) =>
				    {
					v.append(&mut vect);
				    }
				};
			    });
			total
		}
	    );
	let objects: Vec<_> = instances.into_par_iter()
	    .map(|(model, inst)| (vec![*model], inst))
	    .collect();
	self.scene.objects = objects;


	// the lights aren't worth parallelizing
	self.scene.lights.clear();

	for (Lighting(light), maybe_spatial) in (&light_storage, spatial_storage.maybe()).join()
	{
	    self.scene.lights.push(*light, maybe_spatial
				   .map(|spatial| ([spatial.pos[0],
						   spatial.pos[1],
						    spatial.pos[2], 0.],
						   [spatial.rot[0],
						    spatial.rot[1],
						    spatial.rot[2], 0.],)
				   ))
	}


	let camera = self.world.read_resource::<Camera>();
	self.scene.camera = *camera;
	
    }
    
}

/// Defines the render behavior of the GameState
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RenderBehavior
{
    /// This GameState never render
    NoRender,
    /// The GameState let the other states under it to be rendered
    Superpose,
    /// The GameState prevent the states under it to be rendered
    Blocking
}

/// Defines the logic behavior of the GameState
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicBehavior
{
    /// The GameState let the other states under it have their logic run
    Superpose,
    /// The GameState prevent the states under it to have their logic run
    Blocking
}

/// The GameStateStack contains a stack of the different active GameState, but also the inactives states and the registered ProtoGameState s
pub struct GameStateStack
{
    stack: Vec<GameState>,
    register: HashMap<String, ProtoState>,
    pub loaded: HashMap<String, GameState>
}


impl GameStateStack
{
    /// Returns an empty GameStateStack
    pub fn new() -> Self
    {
        Self
        {
            stack: Vec::new(),
            register: HashMap::new(),
            loaded: HashMap::new()
        }
    }

    /// Register a new state from a ProtoState, which will then be able to be loaded and added to the stack
    pub fn register_proto(
        &mut self,
        name: &str,
        proto: ProtoState      
    )
    {
        self.register.insert(name.to_string(), proto);
    }

    /// Returns the ProtoState registered by the given name
    pub fn get_proto(&self, name: String) -> Result<ProtoState, EngineError>
    {
        match self.register.get(&name)
        {
            Some(proto) => Ok(proto.clone()),
            None => EngineError::new(&format!("Game State {} not registered", name))
        }
    }

    /// Register a new state, which will then be able to be loaded and added to the stack
    pub fn register(
        &mut self,
        name: &str,
        scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
        run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
        render_behavior: RenderBehavior,
        logic_behavior: LogicBehavior,
	init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>)
    
    )
    {
        let name = name.to_string();
        self.register.insert(
            name.clone(),
            ProtoState
            {
                name: name,
                scene_builder: scene_builder,
                run_gui: run_gui,
                render_behavior: render_behavior,
                logic_behavior: logic_behavior,
		init: init
            });
    }

    /// Push a state to the Stack
    pub fn push(&mut self, state: GameState)
    {
        self.stack.push(state);
    }

    /// Pop a state out of the stack
    pub fn pop(&mut self)
    {
        if let Some(state) = self.stack.pop()
        {
            let name = state.name.clone();
            println!("Storing state '{}'", name);
            self.loaded.insert(name, state);
        };
    }

    /// Returns an iterator of the stack
    pub fn iter(&self) -> std::slice::Iter<GameState>
    {
        self.stack.iter()
    }
    
    /// Returns an mutable iterator of the stack
    pub fn iter_mut(&mut self) -> std::slice::IterMut<GameState>
    {
        self.stack.iter_mut()
    }
    
    /// Renders the logics of the non-blocked GameStates
    pub fn render(&mut self,
                  gr: &Graphical,
		  ressources: &RessourcesHolder,
                  gui_renderer: &mut Renderer,
                  frame: &mut Frame,
                  gui_context: &mut Context)
    {

	
        let first_block = self.iter()
            .rposition(|state| state.render_behavior == RenderBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };


	
        for state in self.iter_mut().skip(to_skip)
            .filter(|state| state.render_behavior != RenderBehavior::NoRender)
        {
	    state.update_scene();

	    // about 90% of the time spent
            state.scene.render(gr, ressources, frame);

            // gui
            if let Some(gui) = state.gui
            {
                let mut ui = gui_context.frame();
                
                (gui)(&mut ui, &state.proxy);
                
                let draw_data = ui.render();
                gui_renderer
                    .render(&mut frame.frame, draw_data)
                    .expect("Rendering failed GUI on frame");
            }

        }
    }

    /// Runs the logics of the non-blocked GameStates
    pub fn logic(&mut self, devices: &DevicesState)
    {
        let first_block = self.iter()
            .rposition(|state| state.logic_behavior == LogicBehavior::Blocking);
        let to_skip = match first_block
        {
            None => 0,
            Some(pos) => pos
        };
        for state in self.iter_mut().skip(to_skip)
        {
	    state.world.insert((*devices).clone());
	    state.dispatcher.dispatch(&mut state.world);

	    state.send_events();
	    
        }
    }
}

/// A "bluprint" to build a GameState
#[derive(Clone)]
pub struct ProtoState
{
    name: String,
    
    scene_builder: fn(&mut Game) -> Result<Scene, EngineError>,
    run_gui: Option<fn(&mut Ui, &EventLoopProxy<GameEvent>)>,
    init: fn(World, &mut RessourcesHolder) -> (World, Dispatcher<'static, 'static>),

    render_behavior: RenderBehavior, // can make a trait instead
    logic_behavior: LogicBehavior, // can make a trait instead

   
}


/// a ressource to store GameEvent s and send them
#[derive(Debug, Default)]
pub struct EventSender(Vec<GameEvent>);

impl EventSender
{
    /// Constructor
    fn new() -> Self
    {
	Self(Vec::new())
    }

    /// add an event to the EventSender
    pub fn push(&mut self, event: GameEvent)
    {
	self.0.push(event);
    }

    /// use the EventLoopProxy to send the stored event to the EventLoop, emptying the EventSender
    fn send(&mut self, proxy: &EventLoopProxy<GameEvent>)
    {
	self.0.drain(..).for_each(
	    |event|
	    match proxy.send_event(event)
	    {
		Err(_) => panic!("Cannot send user event: Event Loop terminated"),
		_ => ()		
	    }
	)
    }
}

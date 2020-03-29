extern crate nalgebra as na;
extern crate physics;
extern crate graphics;
extern crate rand;

use physics::physics::*;
use physics::shapes::*; 
use physics::misc::*;

use na::Vector3;
use na::geometry::Point3; 

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::object::{ActivationStatus, BodyStatus, DefaultBodySet, DefaultColliderSet}; 

use std::f32::consts::PI; 
use std::f32::INFINITY; 

use base::{Base, EngineError};

use events_handling::{Key, Event, DevicesState};

use graphics::engine::*;
use graphics::misc::*;
use graphics::ressource_handling::*;

use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4}; //, normalize, look_at};

use graphics::glium::glutin::event_loop::{EventLoop, ControlFlow};


/*impl Game
{
    /// Initialize and runs the game
    fn run(&mut self) -> Result<(), base::EngineError>
    {
        self.init()?;
        while !self.exit
        {
            self.handle_events();
            self.game_logic();
            self.render();
            self.physics.run();
        }
        Ok(())
    }
}*/

/**
The Game structure
It owns everything
*/
struct Game
{
    scene: Scene,
    graphic_engine: Graphical,
    ressources: RessourcesHolder,
    base: Base,
    devices: DevicesState,
    exit: bool,
    game_logic: fn(&mut Self),
    physics: Physics,
}

impl Game
{
    fn new(logic: fn(&mut Self), event_loop: &EventLoop<()>) -> Self
    {
        let base = Base::new();
        let mut holder = RessourcesHolder::new();
        let gr = Graphical::new(event_loop, &base, &mut holder);

        // #################################################################################
        // MechanicalWorld with a gravity vector
        let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));
        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();
        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::<f32>::new();
        let col_tab = Vec::new();
        let physics = Physics::new(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators, col_tab);

        Self
        {
            scene: Scene::new(),
            ressources: holder,
            graphic_engine: gr,
            base: base,
            devices: DevicesState::new(),
            exit: false,
            game_logic: logic,
            physics: physics,
        }

    }

    fn set_scene(&mut self, scene: Scene)
    {
        self.scene = scene;
    }

    fn set_physics(&mut self, physics: Physics)
    {
        self.physics = physics;
    }

    /// renders the stored scene
    fn render(&mut self)
    {
        self.graphic_engine.update_dimensions();
        let mut frame = self.graphic_engine.frame();

        frame.clear();
        self.scene.render(&self.graphic_engine, &mut frame);
        
        frame.swap();
        
    }

    /// useless for now
    fn init(&mut self) -> Result<(), base::EngineError>
    {
        let scene = make_scene(
            &self.graphic_engine.display,
            &mut self.ressources,
            &self.base
        )?;

        // #################################################################################
        // MechanicalWorld with a gravity vector
        let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

        let geometrical_world = DefaultGeometricalWorld::<f32>::new();
        let joint_constraints = DefaultJointConstraintSet::<f32>::new();
        let force_generators = DefaultForceGeneratorSet::<f32>::new();

        let obj_set = make_objects(&scene);

        // (bodies, colliders, coll_tab)
        let three_uplet = build_rb_col(obj_set);

        // Where we store all the RigidBody object
        let bodies = three_uplet.0;

        // Where we store all the Collider object
        let colliders = three_uplet.1;

        // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
        let col_tab = three_uplet.2;

        let physics = Physics::new(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators, col_tab);
        // #################################################################################

        self.set_physics(physics);
        self.set_scene(scene);
        Ok(())
    }
    

    // maybe user defined
    fn handle_event(&mut self, event: Event)
    {
        match event {
            Event::KeyPressed(key) => {self.devices.keyboard_state.insert(key);},
            Event::KeyReleased(key) => {self.devices.keyboard_state.remove(&key);},
            Event::ButtonPressed(button) => {self.devices.mouse_state.insert(button);},
            Event::ButtonReleased(button) => {self.devices.mouse_state.remove(&button);},
            Event::MouseMove(x, y) => {self.devices.mouse_move = (self.devices.mouse_move.0+x, self.devices.mouse_move.1+y);}
            Event::ScrollMouse(x, y) => {self.devices.mouse_scroll = (self.devices.mouse_scroll.0+x, self.devices.mouse_scroll.1+y);},
            _ => ()
        }

    }

    /// Initialize and runs the game
    fn run(mut self, event_loop: EventLoop<()>) -> Result<(), base::EngineError>
    {
        self.init()?;

        let mut now = std::time::Instant::now();
        let mut render_date = std::time::Instant::now();
        // 30 fps
        let delay = std::time::Duration::from_millis(1000/30);
        
        event_loop
            .run(move |event, _, control_flow|
                 {

                     // inputs
                     if let Some(ev) = Event::parse_relevant(event)
                     {
                         self.handle_event(ev);
                     } 
                     if self.exit
                     {
                         *control_flow = ControlFlow::Exit
                     }

                     // game logic
                    (self.game_logic)(&mut self);


                     // render
                     now = std::time::Instant::now();
                     if render_date < now
                     {
                         let _delta = (now-render_date+delay).as_nanos();
                         //println!("{} fps ({} ns)", 1_000_000_000/(delta+1), delta);
                         self.render();
                         render_date = now + delay;
                     }

                 });
    }
    
}

fn game_logic(game: &mut Game)
{

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);
    let sensibility = 0.005;
    let speed = 0.001; // parce que pourquoi pas.

    let (mouse_x, mouse_y) = game.devices.mouse_motion();
    camera_rot[1] -= (mouse_x as f32) * sensibility;
    camera_rot[0] -= (mouse_y as f32) * sensibility;

    if game.devices.key_pressed(Key::Q) {
        camera_pos[2] = camera_pos[2] - speed;
    }
    if game.devices.key_pressed(Key::D) {
        camera_pos[2] = camera_pos[2] + speed;
    }
    if game.devices.key_pressed(Key::Z) {
        camera_pos[0] = camera_pos[0] + speed;
    }
    if game.devices.key_pressed(Key::S) {
        camera_pos[0] = camera_pos[0] - speed;
    }
    if game.devices.key_pressed(Key::Escape) {
        game.exit = true;
    }
    game.graphic_engine.camera.relative_move(camera_pos);
    game.graphic_engine.camera.rotation(camera_rot.clone());

    // #################################################################################
    game.physics.run();
    let i = 0;
    for object in game.scene.objects.iter() {
        for similarity in object.1.iter() {
            let translation = game.physics.colliders.get(game.physics.col_tab[i]).unwrap().position(); /*C'est de type Isometry -> ?????????????????????*/
            let rotation = /*?????????????????????*/;
            let scale = similarity.deconstruct().2;
            similarity.set_pos(translation, rotation, scale);
            i += 1;
        }
    }
    // #################################################################################
}

fn new_transformation((tx, ty, tz): (f32, f32, f32),
                      (rx, ry, rz): (f32, f32, f32), scale: f32) -> [[f32; 4]; 4]
{
    let rot =
        rotation(rx, &vec3(1., 0., 0.)) *
        rotation(ry, &vec3(0., 1., 0.)) *
        rotation(rz, &vec3(0., 0., 1.));
    let trans = translation(&vec3(tx, ty, tz));
    let resize = TMat4::from_diagonal(&vec4(scale, scale, scale, 1.));
    *(trans*rot*resize).as_ref()
}


// the holder outlives the scene
fn make_scene(
    disp: &Display,
    holder: & mut RessourcesHolder,
    _base: &Base
) -> Result<Scene, EngineError>
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "textured_cube.obj", &ressources_path)?;
    holder.load_wavefront(disp, "terrain.obj", &ressources_path)?;

    let zeldo = holder.get_object("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_content("terrain").unwrap();
    // le buffer d'instanciation pour la map
    let map_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];


    holder.add_parameters(Params::new().polygon_line(), "wireframe");

    

    // le buffer d'instanciation pour les cubes
    let instances = (0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>();

    
    let mut scene = Scene::new();

    scene.add(vec![zeldo], instances);
    scene.add(vec![map_elements], map_position);

    Ok(scene)
}



fn make_objects(scene: &Scene) -> ObjSet{
    let mut obj_set = ObjSet::new();

    for object in scene.objects.iter() {
        for similarity in object.1.iter() {
            let trs = similarity.deconstruct();
            let translation = trs.0;
            let rotation = trs.1;
            let scale = trs.2;

            let rb_data = RbData::new(
                translation,                            // translation
                rotation,                               // rotation
                true,                                   // gravity_enabled
                BodyStatus::Dynamic,                    // bodystatus
                Vector3::new(0.0, 0.0, 0.0),            // linear_velocity
                Vector3::new(0.0, 0.0, 0.0),            // angular_velocity
                0.0,                                    // linear_damping
                0.0,                                    // angular_damping
                INFINITY,                               // max_linear_velocity
                INFINITY,                               // max_angular_velocity
                0.0,                                    // angular_inertia
                2000.0,                                 // mass
                Point3::new(0.0, 0.0, 0.0),             // local_center_of_mass
                ActivationStatus::default_threshold(),  // sleep_threshold
                Vector3::new(false, false, false),      // kinematic_translations
                Vector3::new(false, false, false),      // kinematic_rotations
                0,                                      // user_data
                true                                    // enable_linear_motion_interpolation
            );

            let col_data = ColData::new(
                Vector3::new(0.0, 0.0, 0.0),            // translation
                Vector3::new(0.0, 0.0, 0.0),            // rotation
                0.0,                                    // density
                0.5,                                    // restitution
                0.2,                                    // friction
                0.01,                                   // margin
                0.002,                                  // linear_prediction
                PI / 180.0 * 5.0,                       // angular_prediction
                false,                                  // sensor
                0                                       // user_data
            );

            let shape = ShapeType::Ball(Ball::new(scale)); // NOT THE RIGHT SHAPE FOR NOW
            let handle = physics::misc::Object::new(shape, rb_data, col_data); // CHANGER LE NOM DE 'Object' (optionnel mais préférable)
            obj_set.push(handle);
        }
    }
    return obj_set;
}



fn main() -> Result<(), EngineError> {

    let event_loop = EventLoop::new();
    let game = Game::new(game_logic, &event_loop);
    game.run(event_loop)

}
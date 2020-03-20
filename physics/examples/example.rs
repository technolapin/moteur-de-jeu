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
use nphysics3d::object::{ActivationStatus, BodyStatus}; 

use std::f32::consts::PI; 
use std::f32::INFINITY; 


use base::Base;
use events_handling::{EventsHandler, Key};

use graphics::engine::*;
use graphics::misc::*;
use graphics::processing::*;

use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4}; //, normalize, look_at};



// #################################################################################s

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
    //*(look_at(&vec3(0., 0., 0.), &vec3(tx, ty, tz), &normalize(&vec3(rx, ry, rz)))*scale).as_ref()
}


// the holder outlives the scene
fn make_scene<'a, 'b>(
    graphics: &Graphical,
    holder: &'b mut RessourcesHolder,
) -> Result<Scene<'a>, &'static str>
where
    'b: 'a,
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(&graphics, "reds.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "terrain.obj", &ressources_path)?;

    let red = holder.get("reds", "Cube_translaté_Cube.002").unwrap();
    let map_elements = holder.get_whole_file("terrain").unwrap();



    // le buffer d'instanciation pour la map
    let map_position = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &vec![Similarity {
            world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
        }],
    ).unwrap();

    
    // le buffer d'instanciation pour les cubes
    let instances = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &(0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>(),
    )
    .unwrap();


    let mut scene = Scene::new();

    scene.add(vec![red], instances);
    scene.add(map_elements, map_position);

    Ok(scene)
}

fn main() -> Result<(), &'static str> {

    // #################################################################################


    // MechanicalWorld with a gravity vector
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

    let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let force_generators = DefaultForceGeneratorSet::<f32>::new();

    // We create the Obj_set
    let mut obj_set = ObjSet::new(); 

    // (bodies, colliders, coll_tab)
    let three_uplet = build_rb_col(obj_set);

    // Where we store all the RigidBody object
    let bodies = three_uplet.0;

    // Where we store all the Collider object
    let colliders = three_uplet.1;

    // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
    let coll_tab = three_uplet.2;

    let mut physics = Physics::new(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators);


    // #################################################################################



    let mut base = Base::new();
    let mut holder = RessourcesHolder::new();
    let mut graphics = Graphical::new(&base.get_events_loop());
    let scene = make_scene(&graphics, &mut holder)?;

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);

    let sensibility = 0.0005;
    let speed = 0.1; // parce que pourquoi pas.

    // la boucle principale
    // pour l'instant on y récupère les évènements en plus de dessiner

    let mut events_handler = EventsHandler::new(base.get_events_loop_mut());

    loop {
    
        // #################################################################################
        physics.run();
        // #################################################################################
        
        
        ///////////////////////////////////////////
        graphics.camera.relative_move(camera_pos);
        graphics.camera.rotation(camera_rot.clone());
        graphics.update_dimensions();


	
        let mut frame = graphics.frame();
        frame.clear();//(0., 0.2, 0.5, 0.));
        
        scene.objects.iter().for_each(|(objects, instances)| {
            objects
                .iter()
                .for_each(|ob| frame.draw(&graphics, &ob, &instances))
        });
	    //frame.draw_2D(&graphics, (0., 0., 10., 10.), 0.);

        frame.show();

        ///////////////////////////////////////////

        camera_pos = Vector3::new(0., 0., 0.);

	/*
        let mut frame = graphics.frame();
	frame.draw_2D(&graphics, (0., 0., 10., 10.), 0.);
        frame.show();
*/


        
        events_handler.update();
        let devices = events_handler.state();

        let (mouse_x, mouse_y) = devices.mouse_motion();
        camera_rot[1] -= (mouse_x as f32) * sensibility;
        camera_rot[0] -= (mouse_y as f32) * sensibility;

        if devices.key_pressed(Key::Q) {
            camera_pos[2] = camera_pos[2] - speed;
        }
        if devices.key_pressed(Key::D) {
            camera_pos[2] = camera_pos[2] + speed;
        }
        if devices.key_pressed(Key::Z) {
            camera_pos[0] = camera_pos[0] + speed;
        }
        if devices.key_pressed(Key::S) {
            camera_pos[0] = camera_pos[0] - speed;
        }
        if devices.key_pressed(Key::Escape) {
            break;
        }
    }
    Ok(())
}
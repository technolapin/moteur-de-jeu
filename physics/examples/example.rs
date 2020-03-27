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
use graphics::ressource_handling::*;
use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4}; //, normalize, look_at};



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
fn make_scene<'a, 'b>(
    disp: & Display,
    holder: &'b mut RessourcesHolder,
) -> Result<Scene<'a>, &'static str>
where
    'b: 'a,
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "reds.obj", &ressources_path)?;
    holder.load_wavefront(disp, "terrain.obj", &ressources_path)?;

    let red = holder.get_object("reds", "Cube_translaté_Cube.002").unwrap();
    let map_elements = holder.get_whole_content("terrain").unwrap();



    // le buffer d'instanciation pour la map
    let map_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];

    

    // le buffer d'instanciation pour les cubes
    let instances = (0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>();

    
    let mut scene = Scene::new();

    scene.add(vec![red], instances);
    scene.add(vec![map_elements], map_position);

    Ok(scene)
}



fn make_objects(scene: Scene) -> ObjSet{
    let mut obj_set = ObjSet::new();

    for object in scene.objects {
        for similarity in object.1 {
            let tx = similarity.world_transformation[3][0];
            let ty = similarity.world_transformation[3][1];
            let tz = similarity.world_transformation[3][2];
            let rx = similarity.world_transformation[0][0];
            let ry = similarity.world_transformation[1][1];
            let rz = similarity.world_transformation[2][2];

            let rb_data = RbData::new(
                Vector3::new(tx,ty,tz),                 // translation
                Vector3::new(rx,ry,rz),                 // rotation
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
            
            //let shape = ShapeType::Cuboid(Cuboid::new(Vector3::new(30.0, 2.0, 30.0)));
            let shape = ShapeType::Ball(Ball::new(0.1));
            let handle = physics::misc::Object::new(shape, rb_data, col_data); // CHANGER LE NOM DE 'Object'
            obj_set.push(handle);
        }
    }
    return obj_set;
}



fn main() -> Result<(), &'static str> {

    // #################################################################################
    // MechanicalWorld with a gravity vector
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

    let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let force_generators = DefaultForceGeneratorSet::<f32>::new();

    let mut base = Base::new();
    let mut holder = RessourcesHolder::new();
    let mut gr = Graphical::new(&base.get_events_loop(), &base, &mut holder);
    // #################################################################################

    let scene = make_scene(&gr.display, &mut holder)?;

    // #################################################################################
    let obj_set = make_objects(scene); // MOVE OCCURS HERE

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

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);


    // Random things
    let sensibility = 0.0005;
    let speed = 0.1; // Because why not
    
    // creating the event handler
    // warning: it takes a mutable reference to the event loop
    let mut events_handler = EventsHandler::new(base.get_events_loop_mut());

    
    // Main loop
    // For now we draw and get the events
    loop {
    
        // #################################################################################
        physics.run();
        // #################################################################################
        
        
        ///////////////////////////////////////////
        gr.camera.relative_move(camera_pos);
        gr.camera.rotation(camera_rot.clone());
        gr.update_dimensions();

        let mut frame = gr.frame();
        frame.clear();//(0., 0.2, 0.5, 0.));

        scene.render(&gr, &mut frame); // PB BECAUSE LINE 164

        frame.show();

        ///////////////////////////////////////////

        camera_pos = Vector3::new(0., 0., 0.);

        
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
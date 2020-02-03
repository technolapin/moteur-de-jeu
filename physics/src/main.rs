/* 
########################################################################
 EVERYTHING BETWEEN "### FOR TESTING PURPOSE ONLY ###" and 
 "### END OF TESTING ###" must be utlimately removed
########################################################################
*/
extern crate nalgebra as na;
extern crate generational_arena;

pub mod shapes;
pub mod misc;
use shapes::*;
use misc::*;

use na::Vector3;

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};



/// Creates the world and ticks it 
pub fn main() {

    // MechanicalWorld with a gravity vector
    let mut mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

    let mut geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let mut joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let mut force_generators = DefaultForceGeneratorSet::<f32>::new();

    // We create the tab of the Obj_set
    let mut obj_tab = ObjSet::build_object_table();



    // ### FOR TESTING PURPOSE ONLY ###
    // BALL
    let coords_ball = Coordinates{
        x: 0 as f32,
        y: 500 as f32,
        z: 0 as f32
    };

    let shape_ball = ShapeType::Ball(Ball{ radius: 1.0 as f32});

    let ball = Object {
        position: coords_ball,
        shape: shape_ball,
        density: 1.0 as f32,
        restitution: 0.8 as f32,
        friction: 0.8 as f32,
        margin: 0.02 as f32,
        linear_prediction: 0.01,
        angular_prediction: 0.1,
        sensor: false,
        user_data: 10,
    };

    obj_tab.push(ball);

    // GROUND

    let coords_ground = Coordinates{
        x: 0 as f32,
        y: 0 as f32,
        z: 0 as f32
    };

    let vec_ground = Vector3::new(3.0, 0.2, 3.0);

    let shape_ground = ShapeType::Cuboid(Cuboid{vector: vec_ground});

    let ground = Object {
        position: coords_ground,
        shape: shape_ground,
        density: 1.0 as f32,
        restitution: 0.0 as f32,
        friction: 0.5 as f32,
        margin: 0.02 as f32,
        linear_prediction: 0.01,
        angular_prediction: 0.1,
        sensor: false,
        user_data: 10,
    };

    obj_tab.push(ground);
    // ### END OF TESTING ###



    // We create the Obj_set
    let obj_set = ObjSet::build_obj_set(obj_tab); 

    let three_uplet = build_colliders(obj_set);

    // Where we store all the RigidBody object
    let mut bodies = three_uplet.0;

    // Where we store all the Collider object
    let mut colliders = three_uplet.1;
    
    // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
    let coll_tab = three_uplet.2;

    loop {
        // The universe is now running and ticking 60 times per second
        mechanical_world.step(
            &mut geometrical_world,
            &mut bodies,
            &mut colliders,
            &mut joint_constraints,
            &mut force_generators
        );
        // ### FOR TESTING PURPOSE ONLY ###
        // Prints the object's coordinates (Ball)
        println!("{}", colliders.get(coll_tab[0]).unwrap().position());
        // ### END OF TESTING ###
    }
}
extern crate nalgebra as na;
extern crate generational_arena;

pub mod shapes;
use shapes::*;

pub mod misc;
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

    // We create the Obj_set
    let obj_set = ObjSet::build_obj_set(obj_tab); 

    let three_uplet = build_rb_col(obj_set);

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
    }
}
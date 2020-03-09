extern crate nalgebra as na;
extern crate physics;

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



/// Creates the world and ticks it 
fn main(){

    // MechanicalWorld with a gravity vector
    let mechanical_world = DefaultMechanicalWorld::new(Vector3::new(0.0, -9.81, 0.0));

    let geometrical_world = DefaultGeometricalWorld::<f32>::new();
    let joint_constraints = DefaultJointConstraintSet::<f32>::new();
    let force_generators = DefaultForceGeneratorSet::<f32>::new();

    // We create the Obj_set
    let mut obj_set = ObjSet::new(); 









    let ground_rbdata = RbData::new(
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 0.0), 
        false, 
        BodyStatus::Static, 
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 0.0), 
        0.0, 
        0.0, 
        0.0, 
        0.0, 
        0.0, 
        0.0, 
        Point3::new(0.0, 0.0, 0.0), 
        ActivationStatus::default_threshold(), 
        Vector3::new(true, true, true), 
        Vector3::new(true, true, true), 
        0, 
        true 
    );

    let ball_rbdata = RbData::new(
        Vector3::new(0.0, 1000.0, 0.0), 
        Vector3::new(0.0, 0.0, 0.0), 
        true, 
        BodyStatus::Dynamic, 
        Vector3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 0.0, 0.0), 
        0.0, 
        0.0, 
        INFINITY, 
        INFINITY, 
        0.0, 
        2000.0, 
        Point3::new(0.0, 0.0, 0.0), 
        ActivationStatus::default_threshold(), 
        Vector3::new(false, false, false), 
        Vector3::new(false, false, false), 
        0, 
        true 
    );

    let ground_coldata = ColData::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0,
        0.5,
        0.01,
        0.002,
        PI / 180.0 * 5.0,
        false,
        0
    );

    let ball_coldata = ColData::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.5,
        0.2,
        0.01,
        0.002,
        PI / 180.0 * 5.0,
        false,
        0
    );

    let ground_shape = ShapeType::Cuboid(Cuboid::new(Vector3::new(30.0, 2.0, 30.0)));
    let ball_shape = ShapeType::Ball(Ball::new(0.1));

    let ground = Object::new(ground_shape, ground_rbdata, ground_coldata);
    let ball = Object::new(ball_shape, ball_rbdata, ball_coldata);
    obj_set.push(ground);
    obj_set.push(ball);







    

    // (bodies, colliders, coll_tab)
    let three_uplet = build_rb_col(obj_set);

    // Where we store all the RigidBody object
    let bodies = three_uplet.0;

    // Where we store all the Collider object
    let colliders = three_uplet.1;
    
    // Where we store the handle of every collider so we can get their position and material later (used for testing only at the moment)
    let coll_tab = three_uplet.2;

    let mut physics = Physics::new(mechanical_world, geometrical_world, bodies, colliders, joint_constraints, force_generators);

    loop {
        physics.run();
        println!("{}", physics.colliders.get(coll_tab[1]).unwrap().position());
    }
}
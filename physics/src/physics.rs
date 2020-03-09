extern crate nalgebra as na;
extern crate generational_arena;

use nphysics3d::force_generator::DefaultForceGeneratorSet;
use nphysics3d::joint::DefaultJointConstraintSet;
use nphysics3d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics3d::object::{DefaultBodySet, DefaultColliderSet};



pub struct Physics{
    pub mechanical_world: DefaultMechanicalWorld<f32>,
    pub geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    pub joint_constraints: DefaultJointConstraintSet<f32>,
    pub force_generators: DefaultForceGeneratorSet<f32>
}

impl Physics
{
    pub fn new(mechanical_world: DefaultMechanicalWorld<f32>,
        geometrical_world: DefaultGeometricalWorld<f32>,
        bodies: DefaultBodySet<f32>,
        colliders: DefaultColliderSet<f32>,
        joint_constraints: DefaultJointConstraintSet<f32>,
        force_generators: DefaultForceGeneratorSet<f32>) -> Physics{

        let physics = Physics{mechanical_world: mechanical_world,
            geometrical_world: geometrical_world,
            bodies: bodies,
            colliders: colliders,
            joint_constraints: joint_constraints,
            force_generators: force_generators};

        return physics;
    }



    /// Runs the universe and ticks it 60 times per second
    pub fn run(&mut self){
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators
        );
        
    }
}
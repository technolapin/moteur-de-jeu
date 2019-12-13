/*
 * Juste un test
 *
 */



use specs::prelude::*;
use std::thread::sleep;
use std::time::Duration;

struct Position(f64, f64);
struct Velocity(f64, f64);

#[derive(Default)]
struct Displayable;

impl Component for Position
{
    type Storage = DenseVecStorage<Self>;
}
impl Component for Velocity
{
    type Storage = DenseVecStorage<Self>;
}

impl Component for Displayable
{
    type Storage = NullStorage<Self>;
}


#[derive(Default)]
struct Space(f64, f64, f64, f64);



struct DisplacementSystem;

impl<'a> System<'a> for DisplacementSystem
{
    type SystemData = (WriteStorage<'a, Position>,
                       ReadStorage<'a, Velocity>);
    fn run(&mut self, (mut positions, velocities): Self::SystemData)
    {
        for (pos, vel) in (&mut positions, &velocities).join()
        {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
    }
}

struct BoundarySystem;

impl<'a> System<'a> for BoundarySystem
{
    type SystemData = (WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       Read<'a, Space>);
    fn run(&mut self, (mut positions, mut velocities, space): Self::SystemData)
    {
        let friction = 0.7;
        for (mut pos, mut vel) in (&mut positions, &mut velocities).join()
        {
            if pos.0 < space.0
            {
                pos.0 = space.0;
                vel.0 = -vel.0*friction;
            }
            if pos.1 < space.1
            {
                pos.1 = space.1;
                vel.1 = -vel.1*friction;
            }
            if pos.0 > space.2
            {
                pos.0 = space.2;
                vel.0 = -vel.0*friction;
            }
            if pos.1 > space.3
            {
                pos.1 = space.3;
                vel.1 = -vel.1*friction;
            }
        }
    }
}


struct DisplaySystem;


impl<'a> System<'a> for DisplaySystem
{
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Displayable>);

    fn run(&mut self, (positions, displayable): Self::SystemData)
    {
        for (pos, _) in (& positions, &displayable).join()
        {
            println!("Some entity is at ({}, {}) !", pos.0, pos.1);
        }
    }  
}




fn main() {

    let mut world = World::new();

    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Displayable>();

    world.insert(Space(0., 0., 10., 10.));
    
    world.create_entity()
        .with(Position(0., 0.))
        .with(Velocity(33., 0.))
        .with(Displayable)
        .build();

    
    let mut dispatcher =
        DispatcherBuilder::new()
        .with(DisplacementSystem, "DisplacementSystem", &[])
        .with(BoundarySystem, "BoundarySystem", &[])
        .with(DisplaySystem, "DisplaySystem", &[])
        .build();

    loop
    {
        dispatcher.dispatch(&mut world);
        sleep(Duration::from_millis(10));
    }
    println!("Hello, world!");
    
}


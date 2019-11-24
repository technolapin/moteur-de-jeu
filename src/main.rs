mod ecs;

use crate::ecs::entity::{Entity, EntityAllocator};
use crate::ecs::world::World;
use crate::ecs::component::{Storage, Component};
use crate::ecs::archetype::*;

// exemple de storage
struct VecStorage<T>
{
    free_space: Vec<usize>,
    data: Vec<Option<T>>
}


impl<Compo> Storage for VecStorage<Compo>
where
    Compo: Component
{
    type Component = Compo;
    
    fn get(&self, index: usize) -> Option<&Compo>
    {
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data.get(index).unwrap().as_ref()
    }
    
    fn get_mut(&mut self, index: usize) -> Option<&mut Compo>
    {
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data.get_mut(index).unwrap().as_mut()
    }
    

    fn insert(&mut self, index: usize, comp: Self::Component)
    {
        if self.data.len() <= index
        {
            for i in self.data.len()..index
            {
                self.data.push(None)
            }
            self.data.push(Some(comp));
        }
    }
    fn delete(&mut self, index: usize)
    {
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data[index] = None;

    }
    
    fn new() -> Self
    {
        Self
        {
            free_space: vec![],
            data: vec![]
        }
    }
    fn len(&self) -> usize
    {
        self.data.len()
    }
}

struct NotAStorage<Compo>(Vec<Compo>);
impl<Compo> Storage for NotAStorage<Compo>
where
     Compo: Component + 'static
{
    type Component = Compo;
    fn get(&self, index: usize) -> Option<&Compo>
    {
        None
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut Compo>
    {
        None
    }
    fn insert(&mut self, index: usize, comp: Compo){}
    fn new() -> Self
    {
        Self(vec![])
    }
    fn delete(&mut self, index: usize)
    {}
    fn len(&self) -> usize
    {
        0
    }
}


#[derive(Debug)]
struct Position
{
    x: i32,
    y: i32
}

impl Component for Position
{
    type Storage = VecStorage<Self>;
}




struct IsFalling;
impl Component for IsFalling
{
    type Storage = VecStorage<Self>;
}


enum Shape
{
    Square(i32, i32, u32, u32),
    Point(i32, i32),
}

impl Component for Shape
{
    type Storage = VecStorage<Self>;
}






fn main() {

    let mut world = World::new();

    
    world
        .add_component::<Position>()
        .add_component::<IsFalling>();


    let mut storage_a = VecStorage::<Shape>::new();
    let mut storage_b = VecStorage::<Position>::new();

//    let join = (&mut storage_a, &mut storage_b);

  //  let it = join.iter();
    


    /*
    {
        let st_ref = world.get_storage_mut::<Position>().unwrap();
        println!("lol {:?}", st_ref.data);
    }
    {
        let entity = world.entity_allocator.new_entity();
        world.add_entity_component::<Position>(0, Position{x: 64, y: 32});
    }
    {
        let st_ref = world.get_storage_mut::<Position>().unwrap();
        println!("lol {:?}", st_ref.data);
    }
    {
        let entity = world.entity_allocator.new_entity();
        world.add_entity_component::<Position>(1, Position{x: 64, y: 32});

    }
    {
        let st_ref = world.get_storage_mut::<Position>().unwrap();
        println!("lol {:?}", st_ref.data);
    }
    
    println!("{:?}", world.components);

    {
        let st_ref = world.get_storage_mut::<Position>()
            .unwrap().data
            .iter_mut()
            .map(|y| match y.as_mut() {Some(x) => x, _ => unreachable!()});

        println!("lol {:?}", st_ref);
        for position in st_ref
        {
            position.x = 1;
            println!("Position {:?}", position);
        }
    }
    {
        let st_ref = world.get_storage_mut::<Position>()
            .unwrap().data
            .iter_mut()
            .map(|y| match y.as_mut() {Some(x) => x, _ => unreachable!()});

        println!("lol {:?}", st_ref);
        for position in st_ref
        {
            println!("Position {:?}", position);
        }
    }
    {
        let st_ref = world.get_storage_mut::<Position>()
            .unwrap().data
            .iter_mut()
            .map(|y| match y.as_mut() {Some(x) => x, _ => unreachable!()});

        println!("lol {:?}", st_ref);
        for position in st_ref
        {
            println!("Position {:?}", position);
        }
    }
    let st_ref = world.get_storage_mut::<Position>()
        .unwrap();

    let compo = st_ref.get_mut(0).unwrap();
    
    compo.x = 10;
    {
        let st_ref = world.get_storage_mut::<Position>()
            .unwrap().data
            .iter_mut()
            .map(|y| match y.as_mut() {Some(x) => x, _ => unreachable!()});

        println!("lol {:?}", st_ref);
        for position in st_ref
        {
            println!("Position {:?}", position);
        }
    }
     */

    use anymap::AnyMap;
    let tuple = (1usize, 2u32, 3i64);

    let mut anymap = AnyMap::new();
    tuple.anymap(&mut anymap);


    println!("{:?}", anymap);
    println!("{:?}", tuple);

    
    let moo = <(usize, u32) as Untuplable>::has_elements(&anymap);

    println!("{:?}", moo);


    let mut arch = Archetype::new();
    arch.add_component::<Position>();
    arch.add_component::<Shape>();

    println!("has {}", arch.has_components::<(Position, Shape,)>());

    arch.add_entity((Position{x: 0, y: 0},
                     Shape::Square(0,0,0,0)));
    
    let poses = arch.get_storage_mut::<Position>();
    
}

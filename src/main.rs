extern crate sdl2;
use anymap::AnyMap;
use std::collections::HashMap;


struct Entity
{
    id: u64,
    index: usize
}

trait Component
{
    type Storage: Storage;
}


trait Storage
{
    type Component: Component;
    fn get(&self, entity: Entity) -> Option<&Self::Component>;
    fn insert(&mut self, entity: Entity, comp: Self::Component); // should add an Ok(()) return type later
    fn new() -> Self;
}

struct VecStorage<T>    
{
    free_space: Vec<usize>,
    data: Vec<T>,
    
}


impl<Compo> Storage for VecStorage<Compo>
where
    Compo: Component
{
    type Component = Compo;
    fn get(&self, entity: Entity) -> Option<&Self::Component>
    {
        let index = entity.index;
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data.get(index)
    }
    fn insert(&mut self, entity: Entity, comp: Self::Component){}
    fn new() -> Self
    {
        Self
        {
            free_space: vec![],
            data: vec![]
        }
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

use std::any::{TypeId, Any};



struct World
{
    entities: Vec<Entity>,
    components: AnyMap,
    components_registry: Vec<TypeId>
}

impl World
{
    fn add_component<Comp>(&mut self) -> &mut Self
    where
        Comp: Component + 'static
    {
        self.components_registry.push(TypeId::of::<Comp::Storage>());
        self.components.insert(Comp::Storage::new());
        self
    }

    fn get_storage<Comp>(&self) -> Option<&Comp::Storage>
    where
        Comp: Component + 'static
    {
        self.components.get::<Comp::Storage>()
    }
}

struct IsFalling;
impl Component for IsFalling
{
    type Storage = VecStorage<Self>;
}


fn main() {

    let mut world = World
    {
        entities: vec![],
        components: AnyMap::new(),
        components_registry: vec![]
    };

    
    world
        .add_component::<Position>()
        .add_component::<IsFalling>();
   
    println!("{:?}", world.components);

    let st_ref = world.get_storage::<Position>().unwrap();

    for position in st_ref.data.iter()
    {
        println!("Position {:?}", position);
    }
    

}


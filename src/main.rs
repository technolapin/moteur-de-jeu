extern crate sdl2;
use anymap::AnyMap;
use std::collections::HashMap;
use std::any::{TypeId, Any};


/**
   Used to manage the entities generations and indexes
**/
struct EntityAllocator
{
    free_indexes: Vec<usize>,
    max_index: usize,
    generation: u64
}

impl EntityAllocator
{
    fn new() -> Self
    {
        Self
        {
            free_indexes: vec![],
            max_index: 0,
            generation: 0
        }
    }
    
    fn new_entity(&mut self) -> Entity
    {
        self.generation += 1;
        match self.free_indexes.pop()
        {
            None =>
            {
                self.max_index += 1;
                Entity
                {
                    generation: self.generation-1,
                    index: self.max_index-1
                }
            },
            Some(index) =>
            {
                Entity
                {
                    generation: self.generation-1,
                    index: index
                }
            }
        }
    }
}


struct StorageIndex
{
    index: usize
}


struct Entity
{
    /// the "identity card" of the entity
    generation: u64,
    
    /// the index the components are stored at
    index: usize
}


/// A component is something you store in a storage
trait Component
{
    type Storage: Storage;
}

/// A storage is something you store components into
trait Storage
{
    type Component: Component;
    fn get(&self, entity: Entity) -> Option<&Self::Component>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Component>;
    fn insert(&mut self, entity: &Entity, comp: Self::Component); // should add an Ok(()) return type later
    fn new() -> Self;
}

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
    
    fn get(&self, entity: Entity) -> Option<&Compo>
    {
        let index = entity.index;
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data.get(index).unwrap().as_ref()
    }
    
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Compo>
    {
        let index = entity.index;
        if index >= self.data.len() || self.data.get(index).is_none()
        {
            panic!("Tried to access a non-existing component");
        }
        
        self.data.get_mut(index).unwrap().as_mut()
    }
    

    fn insert(&mut self, entity: &Entity, comp: Self::Component)
    {
        if self.data.len() <= entity.index
        {
            for i in self.data.len()..(entity.index)
            {
                self.data.push(None)
            }
            self.data.push(Some(comp));
        }
    }
    
    fn new() -> Self
    {
        Self
        {
            free_space: vec![],
            data: vec![]
        }
    }
}

struct NotAStorage<Compo>(Vec<Compo>);
impl<Compo> Storage for NotAStorage<Compo>
where
     Compo: Component + 'static
{
    type Component = Compo;
    fn get(&self, entity: Entity) -> Option<&Compo>
    {
        None
    }
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Compo>
    {
        None
    }
    fn insert(&mut self, entity: &Entity, comp: Compo){}
    fn new() -> Self
    {
        Self(vec![])
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



/**
 The structure owning the entities, the storages and everything else
**/
struct World
{
    entity_allocator: EntityAllocator,
    entities: Vec<Entity>,
    components: AnyMap,
}

impl World
{
    fn new() -> Self
    {
        Self
        {
            entity_allocator: EntityAllocator::new(),
            entities: vec![],
            components: AnyMap::new(),
        }
        
    }
    
    fn add_component<Comp>(&mut self) -> &mut Self
    where
        Comp: Component + 'static
    {
        self.components.insert(Comp::Storage::new());
        self
    }

    fn add_entity_component<Comp>(
        &mut self,
        entity: &Entity,
        component: <<Comp as Component>::Storage as Storage>::Component) -> &mut Self
    where
        Comp: Component + 'static
    {
        self.get_storage_mut::<Comp>().unwrap().insert(entity, component);
        self
    }


    fn get_storage<Comp>(&self) -> Option<&Comp::Storage>
    where
        Comp: Component + 'static
    {
        self.components.get::<Comp::Storage>()
    }
    
    fn get_storage_mut<Comp>(&mut self) -> Option<&mut Comp::Storage>
    where
        Comp: Component + 'static
    {
        self.components.get_mut::<Comp::Storage>()
    }

    fn new_entity(&mut self) -> &mut Self
    {
        self.entities.push(self.entity_allocator.new_entity());
        self
        
    }
    
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

    {
        let st_ref = world.get_storage_mut::<Position>().unwrap();
        println!("lol {:?}", st_ref.data);
    }
    {
        let entity = world.entity_allocator.new_entity();
        world.add_entity_component::<Position>(&entity, Position{x: 64, y: 32});
    }
    {
        let st_ref = world.get_storage_mut::<Position>().unwrap();
        println!("lol {:?}", st_ref.data);
    }
    {
        let entity = world.entity_allocator.new_entity();
        world.add_entity_component::<Position>(&entity, Position{x: 64, y: 32});

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

}


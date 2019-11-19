use crate::ecs::entity::{EntityAllocator, Entity};
use anymap::AnyMap;
use crate::ecs::component::{Component, Storage};

/**
 The structure owning the entities, the storages and everything else
**/
pub struct World
{
    entity_allocator: EntityAllocator,
    components: AnyMap,
}

impl World
{
    pub fn new() -> Self
    {
        Self
        {
            entity_allocator: EntityAllocator::new(),
            components: AnyMap::new(),
        }
        
    }
    
    pub fn add_component<Comp>(&mut self) -> &mut Self
    where
        Comp: Component + 'static
    {
        self.components.insert(Comp::Storage::new());
        self
    }

    pub fn add_entity_component<Comp>(
        &mut self,
        entity_index: usize,
        component: <<Comp as Component>::Storage as Storage>::Component) -> &mut Self
    where
        Comp: Component + 'static
    {
        self.get_storage_mut::<Comp>().unwrap().insert(entity_index, component);
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
        self.entity_allocator.new_entity();
        self
        
    }

    fn delete_entity(&mut self, index: usize)
    {
        self.entity_allocator.delete(index);
    }
    
}

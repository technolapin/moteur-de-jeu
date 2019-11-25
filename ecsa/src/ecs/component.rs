use std::slice::Iter;

use std::cell::RefMut;

/// A component is something an entity can possess
pub trait Component
{
}


struct Storage<Comp>
{
    free_space: Vec<usize>,
    components: Vec<Comp>,
}


impl<Comp> Storage<Comp>
{
    fn new() -> Self
    {
        Self
        {
            free_space: vec![],
            components: vec![]
        }
    }
    
    /// mutable access to component at index
    fn get_mut(&mut self, index: usize) -> RefMut<Comp>
    {
        assert!(index < self.components.len());
        RefMut::new<self.components[index]>()
    }
    
}

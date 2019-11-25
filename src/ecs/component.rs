use std::slice::Iter;

/// A component is something you store in a storage
pub trait Component
{
    type Storage: Storage;
}

/// A storage is something you store components into
pub trait Storage
{
    type Component: Component;
    fn get(&self, index: usize) -> Option<&Self::Component>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Component>;
    fn insert(&mut self, index: usize, comp: Self::Component); // should add an Ok(()) return type later
    fn delete(&mut self, index: usize);
    fn new() -> Self;
    fn len(&self) -> usize;
//    fn iter(&mut self) -> Iter<Option<Component>>;
}




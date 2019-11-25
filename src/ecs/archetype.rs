/***********************
 *
 *  We're using archetypes now (WIP)
 *
 ***********************/
use anymap::AnyMap;
use crate::ecs::component::{Component, Storage};
use tuple_utils::{Split};
use std::cell::{RefCell, RefMut, Ref};
use std::slice::Iter;

// 4 G entities is faaaar enought
// one entities and its components is at least a few bytes, and we don't have the ram for that much
// but we'll use usize anyway because OVERKILL!!!
/// the types of the indexes within the storages
type Index = usize;


#[derive(Debug)]
struct StoragesMap
{
    anymap: AnyMap
}


impl StoragesMap
{
    fn new() -> Self
    {
        Self
        {
            anymap: AnyMap::new()
        }
    }

    fn insert<S: Storage + 'static>(&mut self, sto: S)
    {
        self.anymap.insert(RefCell::new(sto));
    }

    fn insert_component<C: Component>(&mut self,
                                      index: usize,
                                      comp: <<C as Component>::Storage as Storage>::Component)
    where
        <C as Component>::Storage : 'static
    {
        let mut storage_ref = self.get_mut::<<C as Component>::Storage>();
        (*storage_ref).insert(index, comp);
    }

    

    fn get<S: Storage + 'static>(&self) -> Ref<S>
    {
        self.anymap.get::<RefCell<S>>().unwrap().borrow()
    }
    fn get_mut<S: Storage + 'static>(&self) -> RefMut<S>
    {
        self.anymap.get::<RefCell<S>>().unwrap().borrow_mut()
    }

    fn contains<S: Storage + 'static>(&self) -> bool
    {
        self.anymap.contains::<S>()
    }
}



#[derive(Debug)]
pub struct Archetype
{
    storages: StoragesMap,
    last_entity: usize
        
}


impl<'a> Archetype
{
    pub fn new() -> Self
    {
        Self
        {
            storages: StoragesMap::new(),
            last_entity: 0
                
        }
    }

    pub fn add_component<C>(&mut self) -> &mut Self
    where
        C: Component + 'static
    {
        self.storages.insert(C::Storage::new());
        self
    }

    
    fn get_storage<C>(&self) -> Ref<C::Storage>
    where
        C: Component + 'static
    {
        self.storages.get::<C::Storage>()
    }

    pub fn get_storage_mut<C>(&self) -> RefMut<C::Storage>
    where
        C: Component + 'static
    {
        self.storages.get_mut::<C::Storage>()
    }

    fn has_component<C>(&self) -> bool
    where
        C: Component + 'static
    {
        self.storages.contains::<C::Storage>()
    }

    
    pub fn has_components<Comps: ComponentsTuple>(&self) -> bool
    where
        <Comps as ComponentsTuple>::Storages: Untuplable
    {
        <Comps::Storages as Untuplable>::has_elements(&self.storages.anymap)
    }

    pub fn add_entity<Comps: ComponentsTuple + ComponentsTupleFetchable<'a>>(&'a mut self, compos: <Comps as ComponentsTupleFetchable<'a>>::WeirdComponentTypes)
    where
        <Comps as ComponentsTuple>::Storages: Untuplable + MutRefTuplable<'a>
    {
        assert!(self.has_components::<Comps>());

        <Comps as ComponentsTupleFetchable<'a>>::insert(compos, self.last_entity, &mut self.storages);
        self.last_entity +=1;
    }
    
    
    fn get_storages<Compos: ComponentsTupleFetchable<'a>+ ComponentsTuple>(&'a self) ->
        <Compos as ComponentsTupleFetchable>::RefMutStorages
    where
        Compos::Storages: MutRefTuplable<'a>
    {
        <Compos as ComponentsTupleFetchable<'a>>::fetch(&self.storages)
    }

    fn is_alive(&self, index: usize) -> bool
    {
        index < self.last_entity
    }
}






pub trait Untuplable
{
    type Whole;
    type Untupled;
    type Head;
    type Tail;
    fn untuple(self) -> Self::Untupled;
    fn pop(self) -> Self::Head;
    fn anymap(self, anymap: &mut AnyMap);
    fn has_elements(anymap: &AnyMap) -> bool;
}

impl<A: 'static> Untuplable for (A,)
{
    type Whole = (A,);
    type Head = A;
    type Tail = ();
    type Untupled = (A,);
    fn untuple(self) -> Self::Untupled
    {
        self
    }
    fn pop(self) -> Self::Head
    {
        self.0
    }
    fn anymap(self, mut anymap: &mut AnyMap)
    {
        anymap.insert(self.0);
    }
    fn has_elements(anymap: &AnyMap) -> bool
    {
        anymap.contains::<RefCell<A>>()
    }
}


macro_rules! implement_untuple {
    ($head: ident, $($tail:ident),*) => {
        impl<$head: 'static, $($tail: 'static),*> Untuplable for ($head, $($tail),*)
        {
            type Whole = ($head, $($tail),*);
            type Untupled =  ($head, ($($tail),*,));
            type Head = $head;
            type Tail = ($($tail),*,);
            fn untuple(self) -> Self::Untupled
            {
                let ($head, $($tail),*,) = self;
                ($head, ($($tail),*,))
            }

            fn pop(self) -> Self::Head
            {
                self.0
            }
            fn anymap(self, mut anymap: &mut AnyMap)
            {
                let (head, tail) = self.untuple();
                anymap.insert(head);
                tail.anymap(anymap);
            }
            
            fn has_elements(anymap: &AnyMap) -> bool
            {
                anymap.contains::<RefCell<Self::Head>>()
                    && <Self::Tail as Untuplable>::has_elements(anymap)
            }

        }
    }
}





pub trait ComponentsTuple
{
    type Components;
    type Storages;
    
}

pub trait ComponentsTupleFetchable<'a>: ComponentsTuple
where
    Self::Storages: MutRefTuplable<'a>
{
    type RefMutStorages;
    type WeirdComponentTypes;
    fn fetch(storage_cells: &'a StoragesMap) -> Self::RefMutStorages;
    fn insert(components: Self::WeirdComponentTypes, index: usize, storage_cells: &'a mut StoragesMap);
}




macro_rules! implement_ComponentsTupleFetchable {
    ($($comp:ident),*) => {
        impl<'a, $($comp: Component),*,> ComponentsTupleFetchable<'a> for ($($comp),*,)
        where
            $($comp::Storage: 'static),*,

{
            type RefMutStorages =  ($(RefMut<'a, $comp::Storage>),*,);
            type WeirdComponentTypes = ($(<<$comp as Component>::Storage as Storage>::Component),*,);
            fn fetch(storage_cells: &'a StoragesMap) -> Self::RefMutStorages
            {
                ($(storage_cells.get_mut::<<$comp as Component>::Storage>()),*,)
            }
            
            fn insert(components: Self::WeirdComponentTypes, index: usize, storage_cells: &'a mut StoragesMap)
            {
                match components
                {
                    ($($comp),*,) =>
                    {
                        $(storage_cells.insert_component::<$comp>(index, $comp));*
                    }
                }
            }

            
        }
        
    }
}


pub trait RefTuplable<'a>{
    type RefTuple;
    fn to_ref(&'a self) -> Self::RefTuple;
}

pub trait MutRefTuplable<'a>{
    type MutRefTuple;
    fn to_mut_ref(&'a mut self) -> Self::MutRefTuple;
}



macro_rules! implement_componentstuples {
    ($($compo:ident),*) => {
        impl<$($compo: Component),*> ComponentsTuple for ($($compo),*,)
        {
            type Components = ($($compo),*,);
            type Storages = ($($compo::Storage),*,);
        }
    }
}


macro_rules! implement_reftuplable {
    ($($element:ident),*) => {

        impl<'a, $($element: 'a),*,> RefTuplable<'a> for ($($element),*,)
        {
            type RefTuple = ($(&'a $element),*,);
            fn to_ref(&'a self) -> Self::RefTuple
            {
                match self
                {
                    ($($element),*,) => ($(& $element),*,)
                }
            }        
        }

        impl<'a, $($element: 'a),*,> MutRefTuplable<'a> for ($($element),*,)
        {
            type MutRefTuple = ($(&'a mut $element),*,);
            fn to_mut_ref(&'a mut self) -> Self::MutRefTuple
            {
                match self
                {
                    ($($element),*,) => ($($element),*,)
                }
            }        
        }
    }
}



implement_untuple!(A, B);
implement_untuple!(A, B, C);

implement_componentstuples!(A);
implement_componentstuples!(A, B);
implement_componentstuples!(A, B, C);


implement_reftuplable!(A);
implement_reftuplable!(A, B);
implement_reftuplable!(A, B, C);

implement_ComponentsTupleFetchable!(A);
implement_ComponentsTupleFetchable!(A, B);
implement_ComponentsTupleFetchable!(A, B, C);

/***********************
 *
 *  We're using archetypes now (WIP)
 *
 ***********************/
use anymap::AnyMap;
use crate::ecs::component::{Component, Storage};
use tuple_utils::{Split};
use std::cell::{RefCell, RefMut, Ref};

// 4 G entities is faaaar enought
// one entities and its components is at least a few bytes, and we don't have the ram for that much
// but we'll use usize anyway because OVERKILL!!!
/// the types of the indexes within the storages
type Index = usize;


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



pub struct Archetype
{
    storages: StoragesMap
}


impl<'a> Archetype
{
    pub fn new() -> Self
    {
        Self
        {
            storages: StoragesMap::new()
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

    pub fn add_entity<Comps: ComponentsTuple>(&self, compos: Comps)
    where
        <Comps as ComponentsTuple>::Storages: Untuplable
    {
        
        assert!(self.has_components::<Comps>());
        
    }
    
    
    fn get_storages<Compos: ComponentsTupleFetchable<'a>+ ComponentsTuple>(&'a self) ->
        <Compos as ComponentsTupleFetchable>::RefMutStorages
    where
        Compos::Storages: MutRefTuplable<'a>
    {
        <Compos as ComponentsTupleFetchable<'a>>::fetch(&self.storages)
    }
    
    /*
    fn iter<C>(&mut self) -> ()
    where
        C: Component + 'static
    {
        let st = self.storages.get::<C::Storage>().unwrap();
        st.iter()
    }
    */
    
    
}



/*
pub trait Join: Sized
{
    type Components;
    type Storages;
    type ComponentsRef;
    type StoragesRef;
    fn iter(self) -> JoinIterator<Self>;
    fn get(stores: Self, index: usize) -> Option<Self::ComponentsRef>;

}

pub struct JoinIterator<J: Join>
{
    keys: std::ops::Range<usize>,
    join: J
}

impl<'a, A> Join for &'a mut A
where
    A: Storage + 'static
{
    type Components = A::Component;
    type Storages = A;
    type StoragesRef = &'a mut A;
    type ComponentsRef = &'a mut A::Component;
    fn iter(self) -> JoinIterator<Self>
    {
        JoinIterator
        {
            keys: (0..self.len()),
            join: self
        }
    }
    fn get(stores: Self, index: usize) -> Option<Self::ComponentsRef>
    {
        stores.get_mut(index)
    }
}

/// Joins will be tuples of mutables references of containers
macro_rules! implement_join {
    ($($from:ident),*) => {
        impl<'a, $($from: Storage /*+Join*/,)*> Join for ($(&'a mut $from),*,)
        {
            type Components = ($($from::Component),*,);
            type Storages = ($($from),*,);
            type StoragesRef = ($(&'a mut $from),*,);
            type ComponentsRef = ($(&'a mut <$from as Storage>::Component),*,);
            
            fn iter(self) -> JoinIterator<Self>
            {
                JoinIterator
                {
                    keys: (0..self.0.len()), // all elements are supposed to be of the same length
                    join: self
                }
            }
            // Here we go deep in the dark art, by using the types names A, B, ...
            // as variables name (we don't have much choice tho).
            // This requires to ignore the sacro-saint standart formating
            // just for this function.
            #[allow(non_snake_case)]
            fn get(stores: Self, i: usize) -> Option<Self::ComponentsRef>

            {
//                let st = *stores;
                let ($($from,)*): Self::StoragesRef = stores;
                let comp: Self::ComponentsRef = ($($from.get_mut(i)
                                                   .unwrap(),)*);
                Some(comp)
            }
        }

        impl<'a, $($from: Storage + 'static),*> Iterator for JoinIterator<($(&'a mut $from),*,)>
        where
            ($(&'a mut $from),*,): Join
        {
            type Item = ($(&'a mut $from::Component),*,);
            fn next(&mut self) -> Option<Self::Item>
            {
                match self.keys.next()
                {
                    None => None,
                    Some(k) =>
                    {
                        //<($(&'a mut $from),*) as Join>::get( self.join, k)
                        //self.join.get_self(k)
                        let ($($from,)*)  = & self.join;
                        let comp = ($($from.get_mut(k)
                                      .unwrap(),)*);
                        Some(comp)
                    }
                    
                }
            }
        }

        
    }
}



implement_join!(A);
implement_join!(A, B);
implement_join!(A, B, C);
implement_join!(A, B, C, D);
implement_join!(A, B, C, D, E);
implement_join!(A, B, C, D, E, F);

/*
impl<J: Join> Iterator for JoinIterator<J>
{
    type Item = J::ComponentsRef;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.keys.next()
        {
            None => None,
            Some(k) =>
            {
                J::get( self.join, k)
                //self.join.get_self(k)
            }
                
        }
    }
}
*/





 */

/*

trait Join
{
    type Storages;
    type StoragesRef;
    type Components;
    type ComponentsRef;

    fn get(&mut self, index: usize) -> Self::ComponentsRef;
}


impl<'a, A: Storage, B: Storage> Join for (&'a mut A, &'a mut B)
{
    type Storages = (A, B);
    type StoragesRef = (&'a mut A, &'a mut B);
    type Components = (A::Component, B::Component);
    type ComponentsRef = (&'a mut A::Component, &'a mut B::Component);
    fn get(&'a mut self, index: usize) -> Self::ComponentsRef
    {
        let (A, B) = self;
        (A.get_mut(index).unwrap(), B.get_mut(index).unwrap())
        
    }
}
*/








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
            /*
            fn anymap_contains_head(anymap: AnyMap) -> bool
            {
                anymap.contains::<>
            }*/
            
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

trait ComponentsTupleFetchable<'a>: ComponentsTuple
where
    Self::Storages: MutRefTuplable<'a>
{
    type RefMutStorages;
    type WeirdComponentTypes;
    fn fetch(storage_cells: &'a StoragesMap) -> Self::RefMutStorages;
    fn insert(components: Self::WeirdComponentTypes, index: usize, storage_cells: &'a mut StoragesMap);
}


/*

        impl<'a, A: Component, B: Component> ComponentsTupleFetchable<'a> for (A, B)
        where
            <A as Component>::Storage: 'static,
        <B as Component>::Storage: 'static

{
            type RefMutStorages =  (RefMut<'a, A::Storage>, RefMut<'a, B::Storage>);
            fn fetch(storage_cells: &'a StoragesMap) -> Self::RefMutStorages
            {
                (storage_cells.get_mut::<<A as Component>::Storage>(),
                 storage_cells.get_mut::<<B as Component>::Storage>())
            }
        }
*/


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


trait RefTuplable<'a>{
    type RefTuple;
    fn to_ref(&'a self) -> Self::RefTuple;
}

trait MutRefTuplable<'a>{
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


/*
trait StorageAccess
{
    type Storage;
    type StorageRef;
    type ComponentRef;
}

struct WriteAccess<'a, S>
{
    storage: &'a mut S
}


trait StoragesTuple
{
    type Components;
    type Storages;
    fn fetch(anymap: &mut AnyMap) -> Self::StorageRefs;

}

macro_rules! implement_componentstuples {
    ($($sto:ident),*) => {
        impl<'a, $($sto: Storage + 'a),*> StoragesTuple<'a> for ($($sto),*,)
        where
            ($($sto),*,): StoragesTuple<'a>
        {
            type Components = ($($sto::Component),*,);
            type Storages = ($($sto),*,);
            type SoragesRef = ($(&'a mut $sto),*,);
        }

    }
}



impl<'a, S: Storage + 'static> StorageAccess for WriteAccess<'a, S>
{
    type Storage = S;
    type StorageRef = &'a mut S;
    type ComponentRef = &'a mut S::Component;
}

*/


/*
macro_rules! tuples_macro {
    ($($tuple_element:ident),*) => {
        implement_untuple!($($tuple_element),*);
        implement_componentstuples!($($tuple_element),*);
        
    }
}

tuples_macro!(A);
 */


implement_untuple!(A, B);
implement_untuple!(A, B, C);

implement_componentstuples!(A);
implement_componentstuples!(A, B);
implement_componentstuples!(A, B, C);
implement_componentstuples!(A, B, C, D);


implement_reftuplable!(A);
implement_reftuplable!(A, B);
implement_reftuplable!(A, B, C);

implement_ComponentsTupleFetchable!(A);
implement_ComponentsTupleFetchable!(A, B);
implement_ComponentsTupleFetchable!(A, B, C);

/***********************
 *
 *  We're using archetypes now (WIP)
 *
 ***********************/
use anymap::AnyMap;
use std::any::TypeId;
use std::collections::HashMap;
use crate::ecs::component::{Component, Storage};


// 4 G entities is faaaar enought
// one entities and its components is at least a few bytes, and we don't have the ram for that much
// but we'll use usize anyway because OVERKILL!!!
/// the types of the indexes within the storages
type Index = usize;




struct Archetype
{
    storages: AnyMap
}


impl Archetype
{
    fn new() -> Self
    {
        Self
        {
            storages: AnyMap::new()
        }
    }

    fn add_component<C>(&mut self) -> &mut Self
    where
        C: Component + 'static
    {
        self.storages.insert(C::Storage::new());
        self
    }

    fn get_storage<C>(&self) -> Option<&C::Storage>
    where
        C: Component + 'static
    {
        self.storages.get::<C::Storage>()
    }

    fn get_storage_mut<C>(&mut self) -> Option<&mut C::Storage>
    where
        C: Component + 'static
    {
        self.storages.get_mut::<C::Storage>()
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


trait Join: Sized
{
    type Components;
    type Storages;
    type ComponentsRef;
    type StoragesRef;
    fn iter(self) -> JoinIterator<Self>;
    fn get(stores: Self::StoragesRef, index: usize) -> Option<Self::ComponentsRef>;

}

struct JoinIterator<J: Join>
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
    fn get(stores: Self::StoragesRef, index: usize) -> Option<Self::ComponentsRef>
    {
        stores.get_mut(index)
    }
    /*
    fn get(&mut self, index: usize) -> Option<Self::ComponentsRef>
    {
        self.get_mut()
    }
*/
}

/// un Join est un tuple de storages
macro_rules! implement_join {
    ($($from:ident),*) => {
        impl<'a, $($from: Storage + Join,)*> Join for ($(&'a mut $from),*,)
        where
            $(&'a mut $from: Join,)*
        {
            type Components = ($($from::Component),*,);
            type Storages = ($($from),*,);
            type StoragesRef = ($(&'a mut $from),*,);
            //type ComponentsRef = ($(<$from as Join>::ComponentsRef),*,);
            type ComponentsRef = ($(&'a mut <$from as Storage>::Component),*,);
            
            fn iter(self) -> JoinIterator<Self>
            {
                JoinIterator
                {
                    keys: (0..self.0.len()), // all elements are supposed to be of the same length
                    join: self
                }
            }

            fn get(stores: Self::StoragesRef, i: usize) -> Option<Self::ComponentsRef>
            //                   -> Option<($(&'a mut $from::Component),*)>
//                -> Option<($(<$from as Storage>::Component,))>

            {
                let ($($from,)*): Self::StoragesRef = stores;
                //unreachable!();
                let comp: Self::ComponentsRef = ($($from.get_mut(i)
                                                   .unwrap(),)*);
                Some(comp)
            }
        }
    }
}


implement_join!(A);
implement_join!(A, B);
implement_join!(A, B, C);

macro_rules! implement_joiniter {
    ($($from:ident),*) => {
//        impl<$($from,)*> Join for ($($from),*,)
        impl<$($from,)*> Iterator for JoinIterator<($($from,)*)>
        where
            ($($from),*,): Join,
            $($from: Storage + 'static),*,
        {
            //type Item = ($(&'static mut <$from as Storage>::Component),*,);
            type Item = <($($from,)*) as Join>::ComponentsRef;
            fn next(&mut self) -> std::option::Option<Self::Item>
            {
                match self.keys.next()
                {
                    None => None,
                    Some(k) =>
                    {
                        unreachable!();
  //                      Some(
//                        //                        < ($($from),*,) as Join >::get(self.join, k).unwrap()
//                        self.join.get(k).unwrap()
    //                    )
                    }
                }
            }
            
        }
    }
}



implement_joiniter!(A);
implement_joiniter!(A, B);
implement_joiniter!(A, B, C);





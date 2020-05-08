use std::marker::PhantomData;
/// Used to designate an Item stored in a Registerx
pub struct Handle<T>
{
    /// The generation of the associated object
    id: u64,

    /// the index of the associated Item in the Register
    index: usize,
    
    /// zero-size, used to add the type parameter
    phantom: PhantomData<T> // for genericity and security
}



////////////////////////////////////////////////////////////
// a few implementations ///////////////////////////////////


use std::fmt;
use std::hash::{Hash, Hasher};

impl<T> fmt::Debug for Handle<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Handle{{id: {}, index: {}}}", self.id, self.index)
    }
}

impl<T> PartialEq for Handle<T>
{
    fn eq(&self, other: &Self) -> bool
    {
	self.index == other.index
    }
}

impl<T> Eq for Handle<T>{}

impl<T> Hash for Handle<T>
{
    fn hash<H: Hasher>(&self, hasher: &mut H)
    {
	self.index.hash(hasher);
    }
}

impl<T> Clone for Handle<T>
{
    fn clone(&self) -> Self
    {
	Self
	{
	    id: self.id,
	    index: self.index,
	    phantom: PhantomData
	}
    }
}
impl<T> Copy for Handle<T>{}

// we have to precise it is safe to share between threads (because of the PhantomData)
unsafe impl<T> Sync for Handle<T>{}
unsafe impl<T> Send for Handle<T>{}


////////////////////////////////////////////////////////////



/**
Storage for any type T.
Permit constant-time push, deletion and random-access.
Inserting an item creates a Handle<T> to permit the random access.
*/
#[derive(Debug)]
pub struct Register<T>
{
    /// The generation of objects stored so far (always increasing)
    generation: u64,

    /// The elements stored
    storage: Vec<T>,

    /// To keep track of the object "alive"
    alive: Vec<bool>,

    /// The free indexes of storage
    free: Vec<usize>
}

impl<T> Register<T>
{
    /// Constructor
    pub fn new() -> Self
    {
	Self
	{
	    generation: 0,
	    storage: Vec::new(),
	    alive: Vec::new(),
	    free: Vec::new()
	}
    }

    /// adds an element
    pub fn add(&mut self, thing: T) -> Handle<T>
    {
	let handle = match self.free.pop()
	{
	    None =>
	    {
		self.storage.push(thing);
		self.alive.push(true);
		Handle
		{
		    id: self.generation,
		    index: self.storage.len()-1,
		    phantom: PhantomData
		}
	    },
	    Some(index) =>
	    {
		self.storage[index] = thing;
		self.alive[index] = true;
		Handle
		{
		    id: self.generation,
		    index: index,
		    phantom: PhantomData
		}
	    }
	};
	self.generation+=1;
	handle
    }

    /// removes an element
    pub fn remove(&mut self, handle: Handle<T>)
    {
	let index = handle.index;
	if self.alive[index]
	{
	    self.alive[index] = false;
	    self.free.push(index)
	}
    }

    /// returns a reference to a stored element
    pub fn get(&self, handle: Handle<T>) -> &T
    {
	let index = handle.index;
	unsafe{self.storage.get_unchecked(index)}
    }

    /// returns a mutable reference to a stored element
    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T
    {
	let index = handle.index;
	unsafe{self.storage.get_unchecked_mut(index)}
    }

    
}


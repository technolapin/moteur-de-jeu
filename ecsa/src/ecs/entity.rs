type Generation = u64;
type Index = usize;



/**
Represents an object.
An entity isn't made to be stored, but can sometimes be instancied for various reasons.
**/
pub struct Entity(Generation, Index);

impl Entity
{
    /// The index of the entity is the index of all of its components
    fn index(self) -> Index
    {
        self.1
    }
    /// The generation of the entity is a number who caracterise it. A generation cannot be the wame for two differents living entities.
    fn generation(self) -> Generation
    {
        self.0
    }

    /// A dead entity is caracterised by a generation of 0
    fn is_alive(self) -> bool
    {
        self.0 > 0
    }
    
}



/**
   Used to manage the entities generations and indexes
**/
pub struct EntityAllocator
{
    generations: Vec<Generation>,
    free_indexes: Vec<usize>,
    vec_length: usize,
    last_generation: u64
}



impl EntityAllocator
{
    pub fn new() -> Self
    {
        Self
        {
            generations: vec![],
            free_indexes: vec![],
            vec_length: 0,
            last_generation: 0
        }
    }

    fn entity(&self, index: Index) -> Entity
    {
        Entity
        (
            self.generations[index],
            index
        )
    }
    
    pub fn new_entity(&mut self) -> Entity
    {
        self.last_generation += 1;
        match self.free_indexes.pop()
        {
            None =>
            {
                self.vec_length += 1;
                self.generations.push(self.last_generation);
                self.entity(self.vec_length-1)
            },
            Some(index) =>
            {
                self.generations[index] = self.last_generation;
                self.entity(index)
            }
        }
    }
    
    pub fn is_alive(&self, index: Index) -> bool
    {
        self.vec_length > index && self.generations[index] > 0
    }
    
    pub fn delete(&mut self, index: Index)
    {
        assert!(!self.is_alive(index));
        self.generations[index] = 0;
        self.free_indexes.push(index);
        
    }
}




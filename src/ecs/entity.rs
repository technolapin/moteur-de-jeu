type Generation = u64;
type Index = usize;


pub struct Entity(Generation, Index);

impl Entity
{
    fn index(self) -> Index
    {
        self.1
    }
    fn generation(self) -> Generation
    {
        self.0
    }

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
    max_index: usize,
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
            max_index: 0,
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
                self.max_index += 1;
                self.generations.push(self.last_generation);
                self.entity(self.max_index-1)
            },
            Some(index) =>
            {
                self.generations[index] = self.last_generation;
                self.entity(index)
            }
        }
    }
}




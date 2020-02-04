use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write, BufReader};

use super::errors::{EngineError, option_unwrap };

/// Separate the engine from the files & os
pub struct Base
{
    events_loop: glutin::EventsLoop,
    ressources_folder_path: PathBuf
}




impl Base
{
    pub fn new() -> Self
    {
        let events_loop = glutin::EventsLoop::new();
//        let mut holder = ModelsHolder::new();
        Self
        {
            events_loop: events_loop,
            ressources_folder_path: Self::get_ressources_path()
         }
    }

    pub fn get_event_loop(&self) -> &glutin::EventsLoop
    {
        &self.events_loop
    }

    
    
    /// Finds the path of the ressources folder
    fn get_ressources_path() -> PathBuf {
        let args: Vec<String> = std::env::args().collect();
        //the only relevant path we can get is the executable's since the execution dir could be anywhere
        let executable_path = Path::new(&args[0]);
        for path in executable_path.ancestors()
        {
        let ressources_path = path.join(Path::new("ressources"));
            if ressources_path.exists()
            {
                return ressources_path
            }
        }
        panic!("Can't find any 'ressources' folder while going up the path");
    }
    
    /// Reads a file and returns its content
    pub fn read_ressource(&self, ressource_path: PathBuf) -> Result<Vec<u8>, EngineError>
    {
        let path = self.ressources_folder_path.join(ressource_path);
	let f = File::open(path)?;
        let mut reader = BufReader::new(f);
	let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        
	Ok(buffer)
    }

    pub fn write_ressource(&self, ressource_path: PathBuf, content: Vec<u8>) -> Result<(), EngineError>
    {
        let path = self.ressources_folder_path.join(ressource_path);
        
        // checks if this is a valid location
        let _parent = option_unwrap(path.ancestors().nth(1))?;
        
        let mut f = File::create(path)?;
        f.write_all(&content)?;
        Ok(())
    }

}




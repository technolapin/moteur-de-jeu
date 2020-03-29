use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write, BufReader};

use glium::glutin::event_loop::EventLoop;

use super::errors::{EngineError};


/// Separate the engine from the files & os
pub struct Base
{
    events_loop: EventLoop<()>,
    ressources_folder_path: PathBuf
}




impl Base
{
    pub fn new() -> Self
    {
        let events_loop = EventLoop::new();
//        let mut holder = ModelsHolder::new();
        Self
        {
            events_loop: events_loop,
            ressources_folder_path: Self::get_ressources_path()
         }
    }

    pub fn get_events_loop(&self) -> &EventLoop<()>
    {
        &self.events_loop
    }
    pub fn move_events_loop(self) -> EventLoop<()>
    {
        self.events_loop
    }
    
    pub fn get_events_loop_mut(&mut self) -> &mut EventLoop<()>
    {
        &mut self.events_loop
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

    pub fn open_image(&self, ressource_path: PathBuf) -> Result<image::DynamicImage, EngineError>
    {
        Ok(image::open(self.ressources_folder_path.join(Path::new(&ressource_path)))?)

    }

    pub fn open_folder(&self, folder_path: PathBuf) -> Result<Vec<PathBuf>, EngineError>
    {
	let path = self.ressources_folder_path.join(folder_path);
	Ok(path.read_dir()?
	   .filter_map(|result| result.ok())
	   .map(|dir_entry| dir_entry.path())
	   .collect())
    }
    
    
    pub fn write_ressource(&self, ressource_path: PathBuf, content: Vec<u8>) -> Result<(), EngineError>
    {
        let path = self.ressources_folder_path.join(ressource_path);
        
        // checks if this is a valid location
        if path.ancestors().nth(1)
            .is_none()
        {
            return EngineError::new("invalid location");
        }
        
        let mut f = File::create(path)?;
        f.write_all(&content)?;
        Ok(())
    }

}




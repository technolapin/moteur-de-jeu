use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::string::String;
use base::Base;
use base::EngineError;

use crate::misc::{read_file};


use super::Display;


/**
The various registered shaders programs.
For now it has some shaders hard-coded but those will eventually be dissmissed.
*/
pub struct Programs
{
    pub programs: HashMap<String,glium::Program>,
}

impl Programs
{
    pub fn new() -> Self
    {
	Self
	{
 	    programs: HashMap::new()
	}
    }

    pub fn update(&mut self, display: &Display, base: &Base)
    {
	for path in base.open_folder(PathBuf::from("shaders"))
	    .expect("ressource folder missing!")
	    .iter()
	{
	    match self.add_program(display, path)
	    {
		Ok(()) => println!("loaded shader program {:?}", path),
		Err(e) => println!("warning: could not load shader program {:?} ({:?})", path, e)
	    }
	}
    }




//    pub fn add_program_from_name(&mut self, display: &Display, program_name: PathBuf)
  //  {}
    
    /**
    Loads a program from source files.
    The name of the program is the name of the folder containing its various shaders.
    */
    pub fn add_program(&mut self, display: &Display, program_path: &PathBuf) -> Result<(), EngineError>
    {
	let path_vertex = program_path.join(Path::new("vertex.glsl"));
	let path_fragment = program_path.join(Path::new("fragment.glsl"));
	let path_geometry = program_path.join(Path::new("geometry.glsl"));


	
	
	
	println!("{:?}", path_vertex);
	
	let pgrm = 
	if path_geometry.is_file()
	{	glium::Program::from_source(
			    &display.display,
			    read_file(path_vertex).as_str(),
			    read_file(path_fragment).as_str(),
			    Some(read_file(path_geometry).as_str()) )
		?
        } 
	else
	{	glium::Program::from_source(
			    &display.display,
			    read_file(path_vertex).as_str(),
			    read_file(path_fragment).as_str(),
			    None)
		?
	};

	self.programs.insert(program_path.file_stem().unwrap().to_str().unwrap().to_string(), pgrm);
	Ok(())
    }
}


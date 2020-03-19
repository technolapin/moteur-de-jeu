use std::path::{Path, PathBuf};
use std::string::String;
use base::Base;
use base::EngineError;
use crate::ressource_handling::ressources_holder::RessourcesHolder;

use crate::misc::{read_file};


use super::Display;
#[derive(Debug)]
pub struct ProgramCreator(usize);
impl ProgramCreator
{
    fn new() -> Self
    {
	Self(0)
    }

    fn build(&mut self) -> ProgramId
    {
	let prgm = ProgramId(self.0);
	self.0 += 1;
	prgm
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ProgramId(usize);


/**
The various registered shaders programs.
For now it has some shaders hard-coded but those will eventually be dissmissed.
 */
#[derive(Debug)]
pub struct ProgramsHolder
{
    programs: Vec<glium::Program>,
    creator: ProgramCreator
}

impl ProgramsHolder
{
    pub fn new() -> Self
    {
	Self
	{
	    programs: Vec::new(),
	    creator: ProgramCreator::new()
		
	}
    }


    /// ProgramIds are to be valid, should be deterministic
    pub fn get(&self, key: ProgramId) -> Result<&glium::Program, EngineError>
    {
	match self.programs.get(key.0)
	{
	    Some(thing) => Ok(thing),
	    None => Err(EngineError::NoneError)
	}
    }
    

    // TODO: give this responsability to base
    pub fn update(&mut self, display: &Display, base: &Base, holder: &mut RessourcesHolder)
    {
	for path in base.open_folder(PathBuf::from("shaders"))
	    .expect("ressource folder missing!")
	    .iter()
	{
	    match self.add_program(display, path)
	    {
		Ok((id, name)) =>
		{
		    holder.register_program(id.clone(), name.clone());
		    println!("loaded shader program n°{:?} ({}) at {:?}", id, name, path);
		},
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
    pub fn add_program(&mut self, display: &Display, program_path: &PathBuf) -> Result<(ProgramId, String), EngineError>
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

	self.programs.push(pgrm);
	let name = program_path.file_stem().unwrap().to_str().unwrap().to_string();
	let id = self.creator.build();
	Ok((id, name))
    }
}


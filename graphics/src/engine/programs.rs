use std::path::{Path, PathBuf};
use std::string::String;
use base::{Base, EngineError};

use super::Display;
use crate::ressource_handling::RessourcesHolder;
use crate::misc::{read_file};

/// This structure purpose is to create ProgramsIds
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

/**
 Corresponds to a program stored by the ProgramHolder
 Might be replaced by a Arc sometimes
*/
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ProgramId(usize);


/**
The various registered shaders programs.
 */
#[derive(Debug)]
pub struct ProgramsHolder
{
    programs: Vec<glium::Program>,
    creator: ProgramCreator
}

impl ProgramsHolder
{

    /// Creates the ProgramHolder
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
	    None => EngineError::new("WRONG PROGRAM ID")
	}
    }
    

    /// Loads all the programs in the ressources/shaders folder
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
    
    /**
    Loads a program from source files.
    The name of the program is the name of the folder containing its various shaders.
    */
    pub fn add_program(&mut self, display: &Display, program_path: &PathBuf) -> Result<(ProgramId, String), EngineError>
    {
	let path_vertex = program_path.join(Path::new("vertex.glsl"));
	let path_fragment = program_path.join(Path::new("fragment.glsl"));
	let path_geometry = program_path.join(Path::new("geometry.glsl"));

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


use super::{Material, Object, Wavefront, Holder, Tile, Group, Vertex};
use crate::engine::programs::ProgramId;
use crate::engine::Display;
use base::EngineError;
use std::path::Path;
use glium::vertex::VertexBuffer;
use std::sync::Arc;
use std::path::PathBuf;
use base::Base;
use crate::engine::params::Params;

/**
Owns the all Objects imported.
This is what the user is supposed to use.
 */
#[derive(Debug)]
pub struct RessourcesHolder {
    wavefronts: Holder<Wavefront>,
    programs: Holder<ProgramId>,
    tiles: Holder<Tile>,
    params: Holder<Arc<Params>>, // overkill
}

impl RessourcesHolder {
    /// Create a new RessourcesHolder
    pub fn new() -> Self {
        Self
        {
            wavefronts: Holder::new(),
            programs: Holder::new(),
            tiles: Holder::new(),
            params: Holder::new()
        }.init()
    }

    fn init(mut self) -> Self
    {
        self.add_parameters(Params::new(), "scene");
        self.add_parameters(Params::new().always_top(), "foreground");
        self
    }

    pub fn register_program(&mut self, pgrm: ProgramId, name: String)
    {
        self.programs.insert(name, pgrm);
    }
    
    pub fn get_program(&self, name: &str) -> Option<&ProgramId>
    {
        self.programs.get(name)
    }

    fn associate_program(&self, mat: &Material) -> ProgramId
    { 
        match mat
        {
            &Material::Default =>
                *self.programs.get("default").unwrap(),
            &Material::Textured { .. } =>
            {
                *self.programs.get("textured").unwrap()
            }
            &Material::NonTextured { .. } =>
            {
                *self.programs.get("nontextured").unwrap()            
            }
        }
    }

    pub fn add_parameters(&mut self, params: Params, name: &str)
    {
        self.params.insert(name.to_string(), Arc::new(params))
    }

//    pub fn with_parameters(&mut self)
    
    pub fn add_tile( &mut self,
                      display: &Display,
                      base: &Base,
                      image_path: &str ) -> Result<(), EngineError>
    {
        let path = PathBuf::from(image_path);
        let name = match path.file_stem()
        {
            None => EngineError::new("tile texture has invalid name"),
            Some(stem) =>
                match stem.to_str()
                .map(|s| s.to_string())
            {
                None => EngineError::new("tile texture has invalid name"),
                Some(thing) => Ok(thing)
            }
        }?;
        let tile = Tile::new(base, display, path)?;
        self.tiles.insert(name, tile);
        Ok(())
    }


    pub fn obj_parameters(&self, obj: Object, params_name: &str) -> Result<Object, EngineError>
    {
        match self.params.get(params_name)
        {
            None => EngineError::new("unknown parameter"),
            Some(params) => Ok(Object::new(obj.data, params.clone()))
        }
    }
    
    
    /// Fetch a displayable tile as an Object
    pub fn get_tile( &self,
                      name: &str,
                      display: &Display ) -> Result<Object, EngineError>
    {
        match self.tiles.get(name)
        {
            None => EngineError::new("Tile inexistant!"),
            Some(tile) =>
            {
                let program = *self.get_program("textured_2d").unwrap();
                let (w, h) = tile.dims;

	        let mesh = vec![
	            Vertex{position: [0., 0., 0.], texture: [0., 0.], .. Default::default()},
	            Vertex{position: [w, 0., 0.], texture: [1., 0.], .. Default::default()},
	            Vertex{position: [w, h, 0.], texture: [1., 1.], .. Default::default()},
	            Vertex{position: [0., 0., 0.], texture: [0., 0.], .. Default::default()},
	            Vertex{position: [0., h, 0.], texture: [0., 1.], .. Default::default()},
	            Vertex{position: [w, h, 0.], texture: [1., 1.], .. Default::default()},
	        ];

                let vbo = VertexBuffer::new(&display.display, &mesh).unwrap().into_vertex_buffer_any();

                let group = Group
                {
                    vertexes: Arc::new(vbo),
                    material: tile.texture.clone()
                };
                let params = self.params.get("foreground").unwrap().clone();
                Ok(Object::new(vec![(group, program)], params))
            }
        }
    }
   
    /// Fetch a displayable 3D object from a wavefront file name and the name of the requested object.
    pub fn get_object(&self, file: &str, model_name: &str) -> Result<Object, EngineError> {
        let shape_data = self.wavefronts.get_object(file, model_name);
        let params = self.params.get("scene").unwrap().clone();
        let whole_data = shape_data?
            .iter()
            .map(|group| {
                let mat: &Material = &group.material;
                let program = self.associate_program(mat);
                (
                    group.clone(),
                    program,
                )
            })
            .collect::<Vec<_>>();
        Ok(Object::new(whole_data, params))
    }

    /// Like get_object, but the object fetched is the union of all the objects of the wavefront file.
    pub fn get_whole_content(&self, file: &str) -> Result<Object, EngineError> {
        let shape_data = self.wavefronts.get_whole_content(file);
        let params = self.params.get("scene").unwrap().clone();
        let whole_data = shape_data?
            .iter()
            .map(|group| {
                let mat: &Material = &group.material;
                let program = self.associate_program(mat);
                (
                    group.clone(),
                    program,
                )
            })
            .collect::<Vec<_>>();
        Ok(Object::new(whole_data, params))
    }

    /// Loads a wavefront file
    pub fn load_wavefront(
        &mut self,
        disp: &Display,
        filename: &str,
        ressources_path: &Path,
    ) -> Result<(), &'static str> {
        self.wavefronts
            .load_wavefront(disp, filename, ressources_path)
    }

    /// Unloads a wavefront file.
    pub fn unload(&mut self, filename: &str) -> Result<(), &'static str> {
        self.wavefronts.unload(filename)
    }
}

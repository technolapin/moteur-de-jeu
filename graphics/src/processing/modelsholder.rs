use super::{Objects, Object};
use crate::engine::Graphical;

use std::collections::HashMap;
use glium::vertex::VertexBufferAny;
use std::path::Path;

#[derive(Debug)]
pub struct ModelsHolder
{
    wavefronts: HashMap<String, Objects>,
}



impl<'a> ModelsHolder
{
    pub fn new() -> Self
    {
        Self
        {
            wavefronts: HashMap::new(),
        }
    }

    pub fn get(&self, file: &str, model_name: &str) -> Option<Object>
    {
        match self.wavefronts.get(file)
        {
            None => None,
            Some(wavefront) => match wavefront.get_object_checked(model_name.to_string())
            {
                None => None,
                Some(groups) => Some(Object{groups: groups})
            }
        }
    }

    pub fn get_whole_file(&self, file: &str) -> Option<Vec<Object>>
    {
        match self.wavefronts.get(file)
        {
            None => None,
            Some(objects) =>
                Some({
                objects.objects.keys().map(
                    |obj_name|
                    {
                        self.get(file, obj_name).unwrap()
                    }
                ).collect::<Vec<_>>()
            })
        }
    }
    
    pub fn load_wavefront(&mut self,
                          gr: &Graphical,
                          filename: &str,
                          ressources_path: &Path)
    {
        let path_to_wavefront = Path::new(filename);
        let stem = path_to_wavefront.file_stem().unwrap().to_str().unwrap().to_string();
        let path_to_mtl = path_to_wavefront.with_extension("mtl");

        println!("stem: {:?}", stem);
        println!("mtl: {:?}", path_to_mtl);

        self.wavefronts.insert(stem.clone(), Objects::new(gr, path_to_wavefront, &path_to_mtl, ressources_path));
    }
    
}


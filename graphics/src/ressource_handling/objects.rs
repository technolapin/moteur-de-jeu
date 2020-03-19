use super::group::*;
use super::material::*;
use super::vertex::*;
pub use crate::engine::ProgramId;

use crate::engine::display::Display;
//use crate::engine::programs::ProgramsHolder;

use base::EngineError;

use glium::texture::{RawImage2d, Texture2d};
use glium::vertex::{VertexBuffer, VertexBufferAny};

use obj::{Mtl, Obj};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;



/**
This structure represents a 3D object.
It is created by the ModelsHolder, which owns the data.
 */
#[derive(Debug)]
pub struct Object<'a> {
    pub groups: Vec<(&'a VertexBufferAny, &'a Material, ProgramId)>,
}

/**
This structure represents a set of 3D objects and their shared materials.
It typicaly contains all the information a wavefront file and their associated mtl file can provide.
It is owned by the ModelsHolder struct.
 */
#[derive(Debug)]
pub struct Wavefront {
    pub objects: HashMap<String, Vec<Group>>,
    pub materials: HashMap<String, Material>,
}


impl Wavefront {
    /**
    Imports new wavefront file
     */
    pub fn new(
        disp: &Display,
        path_to_wavefront: &Path,
        path_to_mtl: &Path,
        ressources_path: &Path,
    ) -> Self {
        use genmesh::{Polygon, Quad, Triangle};

        // try to load the files
        let path_to_wavefront = ressources_path.join(path_to_wavefront);
        let path_to_mtl = ressources_path.join(path_to_mtl);
        let file = File::open(path_to_wavefront).expect("Can't open wavefront");
        let mut bufreader = ::std::io::BufReader::new(file);
        let obj = Obj::load_buf(&mut bufreader).unwrap();
        let file = File::open(path_to_mtl).unwrap();
        let mut bufreader = ::std::io::BufReader::new(file);
        let mtl = Mtl::load(&mut bufreader);

        /*
        parsing the materials from the .mtl
        several kind of material are supported for now: Textured, NonTextured
        and Default (constructed according to what is being found in the mtl)
        */
        let mut materials = HashMap::new();

        for material in mtl.materials.iter() {
            let mat = match &material {
                obj::Material {
                    ks: Some(specular),
                    ns: Some(specular_exponent),
                    tr: transparency,
                    d: opacityy,
                    map_kd: Some(texture_path),
                    ..
                } => {
                    let opacity = opacityy.unwrap_or(1.).min(1. - transparency.unwrap_or(0.));
                    let image = image::open(ressources_path.join(Path::new(texture_path)))
                        .unwrap()
                        .to_rgba();
                    let image_dimensions = image.dimensions();
                    let image =
                        RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                    let texture = Texture2d::new(&disp.display, image).unwrap();
                    Material::Textured {
                        texture: texture,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        opacity: opacity,
                    }
                }
                obj::Material {
                    ka: Some(ambiant),
                    kd: Some(diffuse),
                    ks: Some(specular),
                    ke: Some(emission),
                    ns: Some(specular_exponent),
                    tr: transparency,
                    d: opacity,
                    map_kd: None,
                    ..
                } => {
                    println!("trans/opac: {:?} {:?}", transparency, opacity);
                    let opacity = opacity.unwrap_or(1.).min(1. - transparency.unwrap_or(0.));
                    println!("OPA: {}", opacity);

                    Material::NonTextured {
                        ambiant_color: *ambiant,
                        diffuse_color: *diffuse,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        emission_color: *emission,
                        opacity: opacity,
                    }
                }

                _ => Material::Default,
            };
            materials.insert(material.name.clone(), mat);
        }

        // starting to parse the .obj and to construct the Wavefront structure
        let mut objects = Wavefront {
            objects: HashMap::new(),
            materials: materials,
        };

        for object in obj.objects.iter() {
            let mut groups = Vec::new();
            for group in object.groups.iter() {
                let mut mesh = Vec::new();

                for polygon in group.polys.iter() {
                    // polygons may be triangles or quads
                    match polygon {
                        &Polygon::PolyTri(Triangle {
                            x: v1,
                            y: v2,
                            z: v3,
                        }) => {
                            for v in [v1, v2, v3].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                        }
                        &Polygon::PolyQuad(Quad {
                            x: v1,
                            y: v2,
                            z: v3,
                            w: v4,
                        }) => {
                            for v in [v1, v2, v3].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                            for v in [v3, v4, v1].iter() {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex {
                                    position: position,
                                    normal: normal,
                                    texture: texture,
                                })
                            }
                        }
                    }
                }

                // adding the group of voxel just constated and the name of its associed material
                // this is fine since one is not supposed to access the material very often
                groups.push(Group {
                    voxels: VertexBuffer::new(&disp.display, &mesh)
                        .unwrap()
                        .into_vertex_buffer_any(),
                    material: match &group.material {
                        Some(mat) => Some(mat.name.clone()),
                        None => None,
                    },
                });
            }
            // the object is finished and added to the Wavefront structure
            objects.objects.insert(object.name.clone(), groups);
        }

        objects
    }

    /**
    Returns a displayable structure made of references of the datas stored in ModelsHolder.
     */
    pub fn get_object(&self, name: String) -> Result<Vec<(&VertexBufferAny, &Material)>, EngineError>
    {
        let groups = self.objects.get(&name).unwrap();
	let v = groups
            .iter()
            .map(|group| {
		let material = match &group.material
		{
                    None => &Material::Default,
                    Some(string) => {
                        self.materials.get(string)
			    .unwrap_or(&Material::Default)
                    }
                };
		
                (
                    &group.voxels,
		    material
                )
            })
            .collect::<Vec<_>>();
            Ok(v)
    }

    
    /**
    Same as get_object() but cannot fail. (preferable as this isn't an operation that's supposed to be repeated a lot).
     */
    pub fn get_object_checked(&self, name: String) -> Result<Vec<(&VertexBufferAny, &Material)>, EngineError> {
        match self.objects.get(&name) {
            None => Err(EngineError::NoneError),
            Some(groups) => Ok(groups
                    .iter()
                    .map(|group| {
			let material = match &group.material
			{
                            None => &Material::Default,
                            Some(string) => {
				self.materials.get(string)
				    .unwrap_or(&Material::Default)
                            }
			};
			
			(
                            &group.voxels,
			    material
			)
                    })
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

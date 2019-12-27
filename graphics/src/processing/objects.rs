use crate::misc::*;
use super::vertex::*;
use super::material::*;
use super::group::*;

use crate::engine::graphical::*;

use glium::vertex::{VertexBufferAny, VertexBuffer};
use glium::texture::{RawImage2d, Texture2d};

use std::fs::File;
use obj::{Obj, Mtl};
use std::io::Cursor;
use std::collections::HashMap;

/**
 * This structure represents a 3D object.
 * It is created by the ModelsHolder, which own the data.
 */
#[derive(Debug)]
pub struct Object<'a>
{
    pub groups: Vec<(&'a VertexBufferAny,
                 Option<&'a Material>)>
}

/**
 * This structure represents a set of 3D objects and their shared materials.
 * It typicaly contains all the information a wavefront file and their associated mtl file can provide.
 */
#[derive(Debug)]
pub struct Objects
{
    pub objects: HashMap<String, Vec<Group>>,
    pub materials: HashMap<String, Material>
}

use std::path::Path;

impl Objects
{
    pub fn new(gr: &Graphical, path_to_wavefront: &Path, path_to_mtl: &Path, ressources_path: &Path) -> Self
    {
        use genmesh::{Polygon, Triangle, Quad};
        
        let path_to_wavefront = ressources_path
            .join(path_to_wavefront);
        let path_to_mtl = ressources_path
            .join(path_to_mtl);

        println!("OBJ {:?}", path_to_wavefront);
        println!("MTL {:?}", path_to_mtl);

        let file = File::open(path_to_wavefront).expect("Can't open wavefront");
        
        let mut bufreader = ::std::io::BufReader::new(file);

        let mut obj = Obj::load_buf(&mut bufreader).unwrap();
        println!("MARCO!");
        //obj.load_mtls().unwrap(); // ne sert à priori à rien du tout pour nous, à voir (et fait planter)
        let file = File::open(path_to_mtl).unwrap();
        let mut bufreader = ::std::io::BufReader::new(file);
        
        let mtl = Mtl::load(&mut bufreader);
        
        //println!("{}", mtl.materials);
        /*
        let ambient_color = None; // Ka
        let diffuse_color = None; // Kd
        let specular_color = None; // Ks
        let emissive = None; // Ke
        // Km?
        // Tf?
        let specular_exponent = None; // Ns
        let refraction_indice = None;// Ni?
        let opacity = None; // d or Tr (d = 1-Tr)
        let illumination = None; // illum

        let map_ambiant_color = None;
        let map_diffuse_color = None;
        let map_specular_color = None;
        let map_emissive = None;
        let map_specular_exponent = None;
        let map_opacity = None;
        let map_bump = None;
        let map_reflexion = None;
         */
        let mut materials = HashMap::new();
        
        
        for material in mtl.materials.iter()
        {
            let mat = match &material
            {
                obj::Material {
                    name: _,
                    ka: _,
                    kd: _,
                    ks: Some(specular),
                    ke: _,
                    km: _,
                    tf: _,
                    ns: Some(specular_exponent),
                    ni: _,
                    tr: transparency,
                    d: opacityy,
                    illum: _,
                    map_ka: _,
                    map_kd: Some(texture_path),
                    map_ks: _,
                    map_ke: _,
                    map_ns: _,
                    map_d: _,
                    map_bump: _,
                    map_refl: _,
                }                =>
                {
                    let opacity = opacityy.unwrap_or(1.).min(
                        1. - transparency.unwrap_or(0.)
                    );

                    println!("TEXTURE PATH {}", texture_path);
                    let image = image::open(ressources_path.join(Path::new(texture_path))).unwrap().to_rgba();
                    let image_dimensions = image.dimensions();
                    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
                    let texture = Texture2d::new(&gr.display, image).unwrap();
                    Material::Textured{
                        texture: texture,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        opacity: opacity
                    }
                },
                obj::Material {
                    name: _,
                    ka: Some(ambiant),
                    kd: Some(diffuse),
                    ks: Some(specular),
                    ke: Some(emission),
                    km: _,
                    tf: _,
                    ns: Some(specular_exponent),
                    ni: _,
                    tr: transparency,
                    d: opacity,
                    illum: _,
                    map_ka: _,
                    map_kd: None,
                    map_ks: _,
                    map_ke: _,
                    map_ns: _,
                    map_d: _,
                    map_bump: _,
                    map_refl: _,
                } =>
                {
                    println!("trans/opac: {:?} {:?}", transparency, opacity);
                    let opacity = opacity.unwrap_or(1.).min(
                        1. - transparency.unwrap_or(0.)
                    );
                    println!("OPA: {}", opacity);

                    Material::NonTextured
                    {
                        ambiant_color: *ambiant,
                        diffuse_color: *diffuse,
                        specular_color: *specular,
                        specular_exponent: *specular_exponent,
                        emission_color: *emission,
                        opacity: opacity
                    }
                },

                _ =>
                {
                    Material::Default
                }
            };
            materials.insert(material.name.clone(), mat);
        }
                
        let mut objects = Objects{
            objects: HashMap::new(),
            materials: materials
        };
        
        
        for object in obj.objects.iter()
        {
            let mut groups = Vec::new();
            for group in object.groups.iter()
            {
                let mut mesh = Vec::new();
                
                
                for polygon in group.polys.iter()
                {
                    match polygon
                    {
                        &Polygon::PolyTri(
                            Triangle{x: v1, y: v2, z: v3}) =>
                        {
                            for v in [v1, v2, v3].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                            
                            }
                        },
                        &Polygon::PolyQuad(
                            Quad{x: v1, y: v2, z: v3, w: v4}) =>
                        {
                            for v in [v1, v2, v3].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                                    
                            }
                            for v in [v3, v4, v1].iter()
                            {
                                let position = obj.position[v.0];
                                let texture = v.1.map(|index| obj.texture[index]);
                                let normal = v.2.map(|index| obj.normal[index]);

                                let texture = texture.unwrap_or([0.0, 0.0]);
                                let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                                mesh.push(Vertex
                                          {
                                              position: position,
                                              normal: normal,
                                              texture: texture,
                                          })
                                    
                            }
                        }

                    }
                }

                groups.push(
                    Group
                    {
                        voxels: VertexBuffer::new(&gr.display, &mesh).unwrap()
                            .into_vertex_buffer_any(),
                        material: match &group.material
                        {
                            Some(mat) => Some(mat.name.clone()),
                            None => None
                        }
                    }
                );
                
            }
            objects.objects.insert(object.name.clone(), groups);
        }

        objects

    }

    pub fn get_object(&self, name: String) -> Vec<(&VertexBufferAny, Option<&Material>)>
    {
        let groups = maybe(self.objects.get(&name), "COULD NOT GET OBJECT");

        groups.iter().map(|group|
                          {
                              (
                                  &group.voxels,
                                  match &group.material
                                  {
                                      None => None,
                                      Some(string) => self.materials.get(string)
                                  }
                               )
                          }
        ).collect::<Vec<_>>()
        
        
    }
    pub fn get_object_checked(&self, name: String) -> Option<Vec<(& VertexBufferAny, Option<&Material>)>>
    {
        match self.objects.get(&name)
        {
            None => None,
            Some(groups) =>Some(
            {
                groups.iter().map(|group|
                                  {
                                      (
                                          &group.voxels,
                                          match &group.material
                                          {
                                              None => None,
                                              Some(string) => self.materials.get(string)
                                          }
                                      )
                                  }
                ).collect::<Vec<_>>()
            })
        }
        
    }

}




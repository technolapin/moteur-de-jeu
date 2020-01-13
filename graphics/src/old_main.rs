#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};
use glium::vertex::VertexBufferAny;


mod processing;
mod engine;
mod misc;

use processing::objects::*;
use processing::material::*;

use engine::graphical::*;
use engine::frame::*;

use misc::*;


use std::collections::HashMap;
        


use rayon::prelude::*;
use rayon::iter::*;



use std::path::Path;

struct Model;

struct DrawInstruction
{
    model: Model,
    position: (f32, f32, f32),
    orientation: (f32, f32, f32),
    size: f32
}

struct CameraInstruction
{
    position: (f32, f32, f32),
    orientation: (f32, f32, f32),
    fov: f32
}


enum GraphicInstruction
{
    Draw(DrawInstruction),
    SetCamera(CameraInstruction),
    Display
}




struct ModelsHolder<'a>
{
    wavefronts: HashMap<String, Objects>,
    objects: HashMap<String, Vec<(&'a VertexBufferAny, Option<&'a Material>)>>,
}


impl<'a, 'b> ModelsHolder<'a>
{
    pub fn new() -> Self
    {
        Self
        {
            wavefronts: HashMap::new(),
            objects: HashMap::new()
        }
    }

    pub fn get(&self, model_name: &str) -> Option<&Vec<(&VertexBufferAny, Option<&Material>)>>
    {
        self.objects.get(model_name)
    }
    
    pub fn load_wavefront(&mut self,
                          gr: &Graphical,
                          path_to_wavefront: &Path,
                          ressources_path: &Path)
    {
        let stem = path_to_wavefront.file_stem().unwrap().to_str().unwrap().to_string();
        let path_to_mtl = path_to_wavefront.with_extension("mtl");

        println!("stem: {:?}", stem);
        println!("mtl: {:?}", path_to_mtl);

        self.wavefronts.insert(stem.clone(), Objects::new(gr, path_to_wavefront, &path_to_mtl, ressources_path));
        for obj_name in self.wavefronts.get(&stem).unwrap().objects.keys()
        {
            let mut name = obj_name.to_string();
            let obj = self.wavefronts.get(&stem).unwrap().get_object(name.clone());
            name.insert_str(0, ":");
            name.insert_str(0, stem.as_str());
            println!("NAME: {} \n OBJ: {:?}", name.clone(), obj);
            self.objects.insert(name, obj);
        }
    }
    
}





fn main() {

    let args: Vec<String> = std::env::args().collect();
    let executable_path = Path::new(&args[0]);
    let crate_path = executable_path.parent().unwrap().parent().unwrap().parent().unwrap();
    let ressources_path = crate_path.join(Path::new("ressources"));
    // The first argument is the path that was used to call the program.
    println!("My path is {:?}.", executable_path);
    println!("Crate path is {:?}.", crate_path);


    
    let mut graphics = Graphical::new();

    // le nom est un jeu de mot nul
    let mut holden = ModelsHolder::new();
    
    holden.load_wavefront(&graphics, Path::new("teto.obj"), &ressources_path);
    
    //holden.load_wavefront(&graphics, Path::new("textured_cube.obj"), &ressources_path);
    //holden.load_wavefront(&graphics, Path::new("reds.obj"), &ressources_path);

    
    let kube = Objects::new(&graphics,
                            Path::new("textured_cube.obj"),
                            Path::new("textured_cube.mtl"),
                            &ressources_path);
    let teto = Objects::new(&graphics,
                            Path::new("teto.obj"),
                            Path::new("teto.mtl"),
                            &ressources_path);
    let red = Objects::new(&graphics,
                           Path::new("red_cube.obj"),
                           Path::new("red_cube.mtl"),
                           &ressources_path);
    let reds = Objects::new(&graphics,
                           Path::new("reds.obj"),
                           Path::new("reds.mtl"),
                           &ressources_path);



    
    
    
    
    println!("\nred: {:?}", red);
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 40)
        .map(|_| {
            let pos: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let dir: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let dir = (dir.0 * 1.5 - 0.75, dir.1 * 1.5 - 0.75, dir.2 * 1.5 - 0.75);
            (pos, dir)
        })
        .collect::<Vec<_>>();

    // building the vertex buffer with the attributes per instance
    // contient les positions des objets instanciés
    let mut per_instance = {


        // créé un vecteur de 10000 vertex (un point par object)
        let data = teapots.iter().map(|_| {
            Attr
            {
                world_position: (0.0, 0.0, 0.0),
            }
        }).collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&graphics.display, &data).unwrap()
    };

    struct ToDisp<'a>
    {
        vertex_buffer: &'a glium::vertex::VertexBufferAny,
        material: Option<&'a processing::material::Material>
    }

    graphics.camera.set_position((0., 0., 0.));
    
    let to_display =
    {
        let mut to_display = Vec::new();
        
        to_display.append(&mut kube.get_object(String::from("Cube.001")));
        to_display.append(&mut teto.get_object(String::from("Lat式改変テト_mesh_Lat式改変テト")));
        to_display.append(&mut reds.get_object(String::from("Cube_translaté_Cube.002")));
        to_display
    };


    {
        let mut mapping = per_instance.map();
        for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
            (src.0).0 += (src.0).0 * 0.01;
            (src.0).1 += (src.0).1 * 0.01;
            (src.0).2 += (src.0).2 * 0.01;

            dest.world_position = src.0;
        }
    }

    let mut t: f32 = 0.;


    // the main loop
    loop
    {
        graphics.camera.rotation((0., 0.01, 0.001));
        graphics.camera.set_position((0., 0., t.cos()));
        //graphics.camera.set_fov(t.cos().abs()*std::f32::consts::PI/2.);
        t += 0.01;
        

        let mut frame = graphics.frame();
        graphics.update_dimensions();
        frame.clear();

        
        to_display
            .iter()
            .for_each(
            |(vertexes, maybe_material)|
            {
                match maybe_material
                {
                    Some(material) =>
                    {
                        frame.draw(&graphics,
                                   vertexes,
                                   &per_instance, material);
                    },
                    None => unimplemented!()
                }
            }
        );

        
        frame.show();
        
    }   

}


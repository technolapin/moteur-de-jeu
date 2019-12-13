#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};


mod processing;
mod engine;

use processing::objects::*;

use engine::graphical::*;
use engine::frame::*;

mod misc;
use misc::*;

        











fn main() {
    let mut graphics = Graphical::new();

    
    let kube = Objects::new(&graphics, "textured_cube.obj", "textured_cube.mtl");
    let teto = Objects::new(&graphics, "teto.obj", "teto.mtl");
    let red = Objects::new(&graphics, "red_cube.obj", "red_cube.mtl");
    
    println!("\nteto: {:?}", teto);
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 100)
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


    let mut to_display = Vec::new();

    to_display.append(&mut kube.get_object(String::from("Cube.001")));
    to_display.append(&mut teto.get_object(String::from("Lat式改変テト_mesh_Lat式改変テト")));
    to_display.append(&mut red.get_object(String::from("Cube")));
    
    
    // the main loop
    loop
    {
        graphics.camera.rotation((0., 0.01, 0.001));
        // updating the teapots
        {
            let mut mapping = per_instance.map();
            for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
                (src.0).0 += (src.1).0 * 0.001;
                (src.0).1 += (src.1).1 * 0.001;
                (src.0).2 += (src.1).2 * 0.001;

                dest.world_position = src.0;
            }
        }
        

        let mut frame = graphics.frame();
        frame.clear();
        /*
        frame.draw(&graphics,
                   &teto_vertex_buffer,
                   &per_instance);
        frame.draw(&graphics,
                   &teapot_vertex_buffer,
                   &per_instance);
         */
        /*
        frame.draw(&graphics,
                   &textured_cube,
                   &per_instance, &texture);
*/
        to_display.iter().for_each(
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





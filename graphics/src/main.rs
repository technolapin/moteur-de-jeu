#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};


mod processing;
mod engine;

use processing::*;
use engine::*;

mod misc;
use misc::*;

        

use nalgebra::base::*;

use rayon::prelude::*;
use rayon::iter::*;



use std::path::Path;

fn matrix_to_array(mat: Matrix4<f32>) -> [[f32; 4]; 4]
{
	let mut out = [[0.; 4]; 4];
	for i in 0..4
	{
		for j in 0..4
		{
			out[j][i] = *mat.get(i + 4*j).unwrap();
		}
	}
	out
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
    let mut holden = ModelsHolder::new();

    holden.load_wavefront(&graphics,
                          "textured_cube.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "transparent_sphere.obj",
                          &ressources_path);
    holden.load_wavefront(&graphics,
                          "teto.obj",
                          &ressources_path);
    
    
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 30)
        .map(|_| {
            let pos: (f32, f32, f32) = ((rand::random::<f32>()), rand::random::<f32>(), rand::random::<f32>());
	    let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let rot: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
	    let rot = (rot.0*6., rot.1*6., rot.2*6.);
            let size: f32 = rand::random();
            (pos, rot, size)
	    
        })
        .collect::<Vec<_>>();

    
    // building the vertex buffer with the attributes per instance
    // contient les positions des objets instanciés
    let mut per_instance = {


        // créé un vecteur de 10000 vertex (un point par object)
        let data = teapots.iter().map(|_| {
            Attr
            {
                world_transformation: [[0.; 4]; 4],
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
    println!("\n HOLDER: {:?} \n", holden);
    let sphere_mauve = holden.get("transparent_sphere", "Sphere").unwrap();
    let teto = holden.get("teto", "Lat式改変テト_mesh_Lat式改変テト").unwrap();



    {//varaible locale aux crochets
        let mut mapping = per_instance.map();
        for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
		let rot = Matrix4::new_rotation(Vector3::new((src.1).0, (src.1).1, (src.1).2));
		let translation = Matrix4::new(
			1.,0.,0.,(src.0).0,
			0.,1.,0.,(src.0).1,
			0.,0.,1.,(src.0).2,
			0.,0.,0.,   1.    );
		let aggr=src.2/1000.;
		let aggrandissement = Matrix4::new(
		    aggr,0.,0.,0.,
		    0.,aggr,0.,0.,
		    0.,0.,aggr,0.,
		    0.,0.,0.,1. );
            
	let transfs =  translation*rot*aggrandissement;
            dest.world_transformation = matrix_to_array(transfs);
        }
    }
    
    
    // the main loop
    let mut t: f32 = 0.;
    loop
    {
	t+= 0.01;
        graphics.camera.rotation((0., 0.01, 0.001));
        graphics.camera.set_position((0., 0., 1.*t.cos()));
        

        let mut frame = graphics.frame();
        graphics.update_dimensions();
        frame.clear();

        frame.draw(&graphics, &teto, &per_instance);
        frame.draw(&graphics, &sphere_mauve, &per_instance);

        
        frame.show();
        
    }   

}





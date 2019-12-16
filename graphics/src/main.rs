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

        

use nalgebra::base::*;

use rayon::prelude::*;
use rayon::iter::*;



use std::path::Path;

fn produit_matrix(mat_a: [[f32; 4]; 4], mat_b: [[f32; 4]; 4]) -> [[f32; 4]; 4]
{
	let mut out = [[0.; 4]; 4];
	
	for i in 0..4
	{
		for j in 0..4
		{
			let mut somme = 0.;
			for k in 0..4
			{
				somme += mat_a[i][k]*mat_b[k][j];
			}
			out[i][j] = somme;
		}
	}
	out
}


fn scalar_product(mat: [[f32; 4]; 4], s: f32) -> [[f32; 4]; 4]
{
	let mut out = [[0.; 4]; 4];
	
	for i in 0..4
	{
		for j in 0..4
		{
			out[i][j] = mat[i][j]*s;
		}
	}
	out
	
}

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


fn transpose(mat: [[f32; 4]; 4]) -> [[f32; 4]; 4]
{
	let mut out = [[0.; 4]; 4];
	
	for i in 0..4
	{
		for j in 0..4
		{
			out[i][j] = mat[j][i];
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
    
    println!("\nred: {:?}", red);
    
    // list of teapots with position and direction
    let mut teapots = (0 .. 100)
        .map(|_| {
            let pos: (f32, f32, f32) = ((rand::random::<f32>()), rand::random::<f32>(), rand::random::<f32>());
	    let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let rot: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
		let rot = (rot.0*6., rot.1*6., rot.2*6.);
            (pos, rot)
	    
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
    
    let to_display =
    {
        let mut to_display = Vec::new();
        
        to_display.append(&mut kube.get_object(String::from("Cube.001")));
        to_display.append(&mut teto.get_object(String::from("Lat式改変テト_mesh_Lat式改変テト")));
        to_display.append(&mut red.get_object(String::from("Cube")));
        /*
        to_display.iter().map(|(vertexes, mat)|
                              ToDisp
                              {
                                  vertex_buffer: vertexes,
                                  material: *mat
                              })
            .collect::<Vec<_>>()
         */
        to_display
    };



    



    {//varaible locale aux crochets
        let mut mapping = per_instance.map();
        for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
		let rot = Matrix4::new_rotation(Vector3::new((src.1).0, (src.1).1, (src.1).2));
/*
	    let rotx= [ [1.,	0.,		0.,		0.],
		    [0.,((src.1).0).cos(), -((src.1).0).sin() , 0.],
		    [0.,((src.1).0).sin()  , ((src.1).0).cos(), 0.],
		    [0.,     	0. ,              0.,           1.] ];

	    let roty =[[((src.1).1).sin()	, 0.,	((src.1).1).cos(), 0.],
		       [0.			, 1., 		0. 	 , 0.],
		       [((src.1).1).cos()	, 0., -((src.1).1).sin() , 0.],
		       [0.			, 0.,           0.	 , 1.] ];

	    let rotz= [ [((src.1).2).cos(), -((src.1).2).sin() , 0., 0.],
		    [((src.1).2).sin()  , ((src.1).2).cos(), 0., 0.],
		    [0.,     	0. ,              1.,           0.] ,
		    [0.,     	0. ,              0.,           1.]];
		let rots = produit_matrix(rotx, produit_matrix(roty, rotz));
*/
/*
		let translation = [ [1.,0.,0.,(src.0).0],
				  [0.,1.,0.,(src.0).1],
				  [0.,0.,1.,(src.0).2],
					  [0.,0.,0.,1.]];
*/
		let translation = Matrix4::new(
			1.,0.,0.,(src.0).0,
			0.,1.,0.,(src.0).1,
			0.,0.,1.,(src.0).2,
			0.,0.,0.,   1.    );
		let aggr=0.0005;
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
        // updating the teapots
        /*
        {
            let mut mapping = per_instance.map();
            for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
                (src.0).0 += (src.1).0 * 0.001;
                (src.0).1 += (src.1).1 * 0.001;
                (src.0).2 += (src.1).2 * 0.001;

                dest.world_position = src.0;
            }
        }
         */
        

        let mut frame = graphics.frame();
        graphics.update_dimensions();
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





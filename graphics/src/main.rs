#[macro_use]
extern crate glium;
extern crate rand;

#[allow(unused_imports)]
use glium::{glutin, Surface};

pub mod engine;
pub mod misc;
pub mod processing;
use engine::*;
use misc::*;
use processing::*;
use nalgebra::base::*;


fn matrix_to_array(mat: Matrix4<f32>) -> [[f32; 4]; 4] {
    let mut out = [[0.; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            out[j][i] = *mat.get(i + 4 * j).unwrap(); // guaranteed
        }
    }
    out
}

fn main() -> Result<(), &'static str> {
    let ressources_path = get_ressources_path();

    let mut event_loop = glutin::EventsLoop::new();

    let mut graphics = Graphical::new(&event_loop);
    let mut holder = ModelsHolder::new();

    holder.load_wavefront(&graphics, "textured_cube.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "reds.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "teto.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "terrain.obj", &ressources_path)?;

    // list of teapots with position and direction
    let mut teapots = (0..30)
        .map(|_| {
            let pos: (f32, f32, f32) = (
                (rand::random::<f32>()),
                rand::random::<f32>(),
                rand::random::<f32>(),
            );
            let pos = (pos.0 * 1.5 - 0.75, pos.1 * 1.5 - 0.75, pos.2 * 1.5 - 0.75);
            let rot: (f32, f32, f32) = (rand::random(), rand::random(), rand::random());
            let rot = (rot.0 * 6., rot.1 * 6., rot.2 * 6.);
            let size: f32 = rand::random();
            (pos, rot, size)
        })
        .collect::<Vec<_>>();

    // building the vertex buffer with the attributes per instance
    // contient les positions des objets instanciés
    let mut per_instance = {
        // créé un vecteur de 10000 vertex (un point par object)
        let data = teapots
            .iter()
            .map(|_| Attr {
                world_transformation: [[0.; 4]; 4],
            })
            .collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&graphics.display.display, &data).unwrap()
    };

    
    let map_position = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &vec![Attr {
            world_transformation: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0, 1.0],
            ],
        }],
    )
    .unwrap();

    graphics.camera.set_position((0., 0., 0.));
    println!("\n HOLDER: {:?} \n", holder);
    let _sphere_mauve = holder.get("transparent_sphere", "Sphere").unwrap();
    let _teto = holder
        .get("teto", "Lat式改変テト_mesh_Lat式改変テト")
        .unwrap();
    let red = holder.get("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holder.get("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_file("terrain").unwrap();

    {
        //variable locale aux crochets
        let mut mapping = per_instance.map();
        for (src, dest) in teapots.iter_mut().zip(mapping.iter_mut()) {
            let rot = Matrix4::new_rotation(Vector3::new((src.1).0, (src.1).1, (src.1).2));
            let translation = Matrix4::new(
                1.,
                0.,
                0.,
                (src.0).0,
                0.,
                1.,
                0.,
                (src.0).1,
                0.,
                0.,
                1.,
                (src.0).2,
                0.,
                0.,
                0.,
                1.,
            );
            let aggr = src.2 / 1000.;
            let aggrandissement = Matrix4::new(
                aggr, 0., 0., 0., 0., aggr, 0., 0., 0., 0., aggr, 0., 0., 0., 0., 1.,
            );

            let transfs = translation * rot * aggrandissement;
            dest.world_transformation = matrix_to_array(transfs);
        }
    }

    use glutin::VirtualKeyCode;

    //    let event_loop = graphics.get_event_loop();

    let mut camera_pos = (0., 0., 0.);
    let mut camera_rot = (0., 0., 0.);

    use std::collections::HashSet;
    let mut keys = HashSet::new();
    let sensibility = 0.0005;

    // la boucle principale
    // pour l'instant on y récupère les évènements en plus de dessiner
    let mut close = false;
    while !close {
        println!("KEYS EVENTS {:?}", keys);

        graphics.camera.relative_move(camera_pos);

        graphics.camera.rotation(camera_rot.clone());
        camera_pos = (0., 0., 0.);

        //        println!("CAMERA IS AT {:?}", camera_pos.clone());

        let mut frame = graphics.frame();
        graphics.update_dimensions();
        frame.clear();

        frame.draw(&graphics, &red, &per_instance);
        frame.draw(&graphics, &zeldo, &per_instance);
        //frame.draw(&graphics, &teto, &per_instance);

        map_elements.iter().for_each(|ob| {
            frame.draw(&graphics, &ob, &map_position);
        });

        frame.show();

        // on appelle une closure pour chaque évènement présent dans la file des évènements (cachée dans l'eventloop ou un truc du genre)
        // y'a moyen de faire moins de matches pour améliorer la lisibilité du code
        event_loop.poll_events(|event| {
            println!("EVENT {:?}", event);
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => {
                        close = true;
                    }
                    _ => (),
                },
                glutin::Event::DeviceEvent { event, .. } => match event {
                    glutin::DeviceEvent::Key(keyboard_input) => {
                        match keyboard_input.virtual_keycode {
                            None => (),
                            Some(keycode) => {
                                if keyboard_input.state == glutin::ElementState::Released {
                                    keys.remove(&keycode);
                                } else {
                                    keys.insert(keycode);
                                }
                            }
                        }
                    }

                    glutin::DeviceEvent::Motion { axis, value } => {
                        println!("MOTION Axe:{} value:{}", axis, value);
                        match axis {
                            0 => {
                                camera_rot.1 += (value as f32) * sensibility;
                            }
                            1 => {
                                camera_rot.0 += (value as f32) * sensibility;
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                },
                _ => (),
            };
        });

        let speed = 0.1; // parce que pourquoi pas.

        // une fois qu'on a l'ensembles des touches appuyées, on fait des trucs avec, genre bouger la camera
        for keycode in keys.iter() {
            match keycode {
                VirtualKeyCode::Z => {
                    camera_pos.0 = camera_pos.0 + speed;
                }
                VirtualKeyCode::S => {
                    camera_pos.0 = camera_pos.0 - speed;
                }
                VirtualKeyCode::Q => {
                    camera_pos.2 = camera_pos.2 - speed;
                }
                VirtualKeyCode::D => {
                    camera_pos.2 = camera_pos.2 + speed;
                }
                _ => {
                    println!("NOTHING TO DO WITH {:?}", keycode);
                }
            };
        }
    }
    Ok(())
}

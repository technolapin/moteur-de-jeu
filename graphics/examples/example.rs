#[macro_use]
extern crate rand;
extern crate graphics;
extern crate nalgebra;

use graphics::engine::*;
use graphics::processing::*;
use graphics::misc::*;

use nalgebra::base::*;


use glutin::VirtualKeyCode;
use std::collections::HashSet;

// the holder outlives the scene
fn make_scene<'a, 'b>(
    graphics: &Graphical,
    holder: &'b mut RessourcesHolder,
) -> Result<Scene<'a>, &'static str>
where
    'b: 'a,
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(&graphics, "textured_cube.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "reds.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "teto.obj", &ressources_path)?;
    holder.load_wavefront(&graphics, "terrain.obj", &ressources_path)?;

    let _sphere_mauve = holder.get("transparent_sphere", "Sphere").unwrap();
    let _teto = holder
        .get("teto", "Lat式改変テト_mesh_Lat式改変テト")
        .unwrap();
    let red = holder.get("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holder.get("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_file("terrain").unwrap();

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
            .map(|_| Similarity {
                world_transformation: [[0.; 4]; 4],
            })
            .collect::<Vec<_>>();

        glium::vertex::VertexBuffer::dynamic(&graphics.display.display, &data).unwrap()
    };

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

    let map_position = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &vec![Similarity {
            world_transformation: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0, 1.0],
            ],
        }],
    )
    .unwrap();

    let mut scene = Scene::new();

    scene.add(vec![red, zeldo], per_instance);
    scene.add(map_elements, map_position);

    Ok(scene)
}





pub struct Base
{
    event_loop: glutin::EventsLoop,
    holder: RessourcesHolder,
}

impl Base
{
    pub fn new() -> Self
    {
        let mut event_loop = glutin::EventsLoop::new();
        let mut holder = RessourcesHolder::new();
        Self
        {
            event_loop: event_loop,
            holder: holder
        }
    }
}





fn main() -> Result<(), &'static str> {
    let mut base = Base::new();
//    let mut event_loop = glutin::EventsLoop::new();
  //  let mut holder = RessourcesHolder::new();
    let mut graphics = Graphical::new(&base.event_loop);
    let scene = make_scene(&graphics, &mut base.holder)?;

    let mut camera_pos = (0., 0., 0.);
    let mut camera_rot = (0., 0., 0.);

     let mut keys = HashSet::new();
    let sensibility = 0.0005;
    let speed = 0.1; // parce que pourquoi pas.

    // la boucle principale
    // pour l'instant on y récupère les évènements en plus de dessiner

    let mut close = false;
    while !close {
        ///////////////////////////////////////////
        graphics.camera.relative_move(camera_pos);
        graphics.camera.rotation(camera_rot.clone());
        let mut frame = graphics.frame();
        graphics.update_dimensions();
        frame.clear();
        scene.objects.iter().for_each(|(objects, instances)| {
            objects
                .iter()
                .for_each(|ob| frame.draw(&graphics, &ob, &instances))
        });
        frame.show();

        ///////////////////////////////////////////

        camera_pos = (0., 0., 0.);

        // on appelle une closure pour chaque évènement présent dans la file des évènements (cachée dans l'eventloop ou un truc du genre)
        // y'a moyen de faire moins de matches pour améliorer la lisibilité du code
        base.event_loop.poll_events(|event| {
            println!("EVENT {:?}", event);
            match event {
                glutin::Event::WindowEvent {
                    event: glutin::WindowEvent::CloseRequested,
                    ..
                } => {
                    close = true;
                }
                /*
                                glutin::Event::WindowEvent { event, .. } => match event {
                                    glutin::WindowEvent::CloseRequested => {
                                        close = true;
                                    }
                                    _ => (),
                                },
                */
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

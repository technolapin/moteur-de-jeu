extern crate graphics;
extern crate nalgebra;
extern crate rand;


use base::Base;
use events_handling::{EventsHandler, Key};

use graphics::engine::*;
use graphics::misc::*;
use graphics::processing::*;

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

/*


pub struct Base
{
    event_loop: glutin::EventsLoop,
    holder: RessourcesHolder,
}

impl Base
{
    pub fn new() -> Self
    {
        let event_loop = glutin::EventsLoop::new();
        let holder = RessourcesHolder::new();
        Self
        {
            event_loop: event_loop,
            holder: holder
        }
    }
}

*/

fn main() -> Result<(), &'static str> {
    let mut base = Base::new();
    let mut holder = RessourcesHolder::new();
    let mut graphics = Graphical::new(&base.get_events_loop());
    let scene = make_scene(&graphics, &mut holder)?;

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);

    let sensibility = 0.0005;
    let speed = 0.1; // parce que pourquoi pas.

    // la boucle principale
    // pour l'instant on y récupère les évènements en plus de dessiner

    let mut events_handler = EventsHandler::new(base.get_events_loop_mut());

    loop {
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

        camera_pos = Vector3::new(0., 0., 0.);

        events_handler.update();
        let devices = events_handler.state();

        let (mouse_x, mouse_y) = devices.mouse_motion();
        camera_rot[1] -= (mouse_x as f32) * sensibility;
        camera_rot[0] -= (mouse_y as f32) * sensibility;

        if devices.key_pressed(Key::Q) {
            camera_pos[2] = camera_pos[2] - speed;
        }
        if devices.key_pressed(Key::D) {
            camera_pos[2] = camera_pos[2] + speed;
        }
        if devices.key_pressed(Key::Z) {
            camera_pos[0] = camera_pos[0] + speed;
        }
        if devices.key_pressed(Key::S) {
            camera_pos[0] = camera_pos[0] - speed;
        }
    }
    Ok(())
}

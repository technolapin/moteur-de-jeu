extern crate graphics;
extern crate nalgebra;
extern crate rand;


use base::Base;
use events_handling::{EventsHandler, Key};

use graphics::engine::*;
use graphics::misc::*;
use graphics::processing::*;

use nalgebra::base::*; 

use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4}; //, normalize, look_at};
fn new_transformation((tx, ty, tz): (f32, f32, f32),
                      (rx, ry, rz): (f32, f32, f32), scale: f32) -> [[f32; 4]; 4]
{
    let rot =
        rotation(rx, &vec3(1., 0., 0.)) *
        rotation(ry, &vec3(0., 1., 0.)) *
        rotation(rz, &vec3(0., 0., 1.));
    let trans = translation(&vec3(tx, ty, tz));
    let resize = TMat4::from_diagonal(&vec4(scale, scale, scale, 1.));
    *(trans*rot*resize).as_ref()
    //*(look_at(&vec3(0., 0., 0.), &vec3(tx, ty, tz), &normalize(&vec3(rx, ry, rz)))*scale).as_ref()
}


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
    let teto = holder
        .get("teto", "Lat式改変テト_mesh_Lat式改変テト")
        .unwrap();
    let red = holder.get("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holder.get("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_file("terrain").unwrap();



    // le buffer d'instanciation pour la map
    let map_position = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &vec![Similarity {
            world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
        }],
    ).unwrap();

    

    // le buffer d'instanciation pour les cubes
    let instances = glium::vertex::VertexBuffer::dynamic(
        &graphics.display.display,
        &(0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>(),
    )
    .unwrap();

    
    let mut scene = Scene::new();

    scene.add(vec![red, zeldo, teto], instances);
    scene.add(map_elements, map_position);

    Ok(scene)
}

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
        graphics.update_dimensions();

        
        let mut frame = graphics.frame();
        frame.clear();//(0., 0.2, 0.5, 0.));
        
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
        if devices.key_pressed(Key::Escape) {
            break;
        }
    }
    Ok(())
}

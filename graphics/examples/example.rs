extern crate graphics;
extern crate nalgebra;
extern crate rand;


use base::Base;
use events_handling::{EventsHandler, Key};

use graphics::engine::*;
use graphics::misc::*;
use graphics::ressource_handling::*;

use std::path::PathBuf;
use glium::texture::{RawImage2d, Texture2d};
use nalgebra::base::*;
use nalgebra_glm::{vec3, vec4, translation, rotation, TMat4};



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
}


// the holder outlives the scene
fn make_scene<'a, 'b>(
    disp: & Display,
    holder: &'b mut RessourcesHolder,
) -> Result<Scene<'a>, &'static str>
where
    'b: 'a,
{
    let ressources_path = get_ressources_path();

    holder.load_wavefront(disp, "textured_cube.obj", &ressources_path)?;
    holder.load_wavefront(disp, "reds.obj", &ressources_path)?;
    holder.load_wavefront(disp, "transparent_sphere.obj", &ressources_path)?;
    holder.load_wavefront(disp, "teto.obj", &ressources_path)?;
    holder.load_wavefront(disp, "terrain.obj", &ressources_path)?;

    let _sphere_mauve = holder.get_object("transparent_sphere", "Sphere").unwrap();
    let teto = holder
        .get_object("teto", "Lat式改変テト_mesh_Lat式改変テト")
        .unwrap();
    let red = holder.get_object("reds", "Cube_translaté_Cube.002").unwrap();
    let zeldo = holder.get_object("textured_cube", "Cube.001").unwrap();
    let map_elements = holder.get_whole_content("terrain").unwrap();



    // le buffer d'instanciation pour la map
    let map_position = vec![Similarity {
        world_transformation: new_transformation((0., 0., 0.), (0., 0., 0.), 1.)
    }];

    

    // le buffer d'instanciation pour les cubes
    let instances = (0..30).map(|_| Similarity {
            world_transformation: new_transformation(
                (rand::random(), rand::random::<f32>(), rand::random::<f32>()), 
                (rand::random(), rand::random(), rand::random()),
                0.001)
        }).collect::<Vec<_>>();

    
    let mut scene = Scene::new();

    scene.add(vec![red, zeldo, teto], instances);
    scene.add(vec![map_elements], map_position);

    Ok(scene)
}



fn main() -> Result<(), &'static str> {

    
    let mut base = Base::new();
    let mut holder = RessourcesHolder::new();
    let mut gr = Graphical::new(&base.get_events_loop(), &base, &mut holder);

    // la texture pour le rectangle de test 2d
    let program_2d = holder.get_program("textured_2d".to_string()).unwrap();
    
    let scene = make_scene(&gr.display, &mut holder)?;

    let mut camera_pos = Vector3::new(0., 0., 0.);
    let mut camera_rot = Vector3::new(0., 0., 0.);


    // des trucs au pif
    let sensibility = 0.0005;
    let speed = 0.1; // parce que pourquoi pas.




    let image = base.open_image(PathBuf::from("edgytet.png"))
        .unwrap()
	.to_rgba();
    
    let image_dimensions = image.dimensions();
    let image =
        RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    
    let texture = Texture2d::new(&gr.display.display, image).unwrap();

    // creating the event handler
    // warning: it takes a mutable reference to the event loop
    let mut events_handler = EventsHandler::new(base.get_events_loop_mut());


    // la boucle principale
    // pour l'instant on y récupère les évènements en plus de dessiner
    
    loop {
        ///////////////////////////////////////////
        gr.camera.relative_move(camera_pos);
        gr.camera.rotation(camera_rot.clone());
        gr.update_dimensions();


	
        let mut frame = gr.frame();
        frame.clear();//(0., 0.2, 0.5, 0.));

        scene.render(&gr, &mut frame);
	
	frame.draw_image_2d(&gr, (0., 0., 0.7, 0.7), 0., &texture, program_2d);

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

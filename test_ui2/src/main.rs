extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use sdl2::mouse::MouseButton;
use std::time::Duration;


// Listener pour clavier (le focus doit être dans la fenêtre)
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Zone d'action du listener
    let _window = video_subsystem.window("Keyboard", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = sdl_context.event_pump()?;
    let mut state;
    let mut prev_keys = HashSet::new();

    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        }

        //Partie Clavier

        // Create a set of pressed Keys.
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys; // Touche récement appuyé
        let old_keys = &prev_keys - &keys; // Garde en mémoire les touches maintenues pressé

        if !new_keys.is_empty(){
            println!("J'appui : {:?}", new_keys);
            //if new_keys.contains("Q"){
             //   println!("Je veux sortir !");
            //}
        }

        if !old_keys.is_empty() {
            println!("Je relache : {:?}", old_keys);
        }

        prev_keys = keys;


        //Partie Souris

        // Event sur Clic droit ou gauche
        if events.mouse_state().is_mouse_button_pressed(MouseButton::Left) | events.mouse_state().is_mouse_button_pressed(MouseButton::Right){
            state = events.relative_mouse_state();
            println!("Relative - X = {:?}, Y = {:?}", state.x(), state.y()); // Déplacement de souris
            println!("State - X = {:?}, Y = {:?}", events.mouse_state().x(), events.mouse_state().y()); // Position de souris
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
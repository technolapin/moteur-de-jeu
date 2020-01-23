extern crate glium;

fn main() {
    use glutin::{MouseButton,ElementState,VirtualKeyCode,KeyboardInput};
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let _display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut closed = false;
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::MouseInput{device_id,state: ElementState::Pressed,button,modifiers} => {println!("Boutons Souris Appuyé  {:?} ",button)},
                    glutin::WindowEvent::MouseWheel{device_id,delta,phase,modifiers} => {println!("Boutons Souris Appuyé  {:?} ",delta)},
                    glutin::WindowEvent::KeyboardInput{device_id,input: KeyboardInput{scancode: u32,state: ElementState::Pressed,virtual_keycode,modifiers}} => {println!("Boutton Clavier Appuyé  {:?}",virtual_keycode)},
                    glutin::WindowEvent::CloseRequested => {closed = true;println!("Fermeture fenêtre")},

                    _ => (),
                },
                _ => (),
            }
        });
    }
}

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyPressed(Key),
    KeyReleased(Key),
    ButtonPressed(Button),
    ButtonReleased(Button),

    // ------------------------------------------
    MouseMove(f64, f64),

    // ------------------------------------------
    ScrollMouse(f32, f32),

    // ------------------------------------------
    Default, // Truc par défaut
}

impl Event {
    // https://docs.rs/glutin/0.21.2/glutin/enum.Event.html
    pub fn parse(ev: glium::glutin::event::Event<()>) -> Self {
        match ev {
            glium::glutin::event::Event::DeviceEvent {
                device_id: _,
                event,
            } => Self::parse_device_event(event),
            lol => {println!("{:?}", lol); Self::Default},
        }
    }

    // https://docs.rs/glutin/0.21.2/glutin/enum.DeviceEvent.html
    fn parse_device_event(ev: glium::glutin::event::DeviceEvent) -> Self {
        match ev {
            glium::glutin::event::DeviceEvent::Key(keyboard_input) => Self::parse_touche_clavier(keyboard_input),
            glium::glutin::event::DeviceEvent::MouseMotion { delta } => Self::parse_mouvement_souris(delta),
            glium::glutin::event::DeviceEvent::Button { button, state } => {
                Self::parse_bouton_souris(button, state)
            }
            glium::glutin::event::DeviceEvent::MouseWheel { delta } => Self::parse_scroll(delta),
            _ => Self::Default,
        }
    }

    // https://docs.rs/glutin/0.21.2/glutin/struct.KeyboardInput.html
    fn parse_touche_clavier(ev: glium::glutin::event::KeyboardInput) -> Self {
        let key = Key::convert_key(ev.virtual_keycode, ev.scancode);
        match ev.state {
            glium::glutin::event::ElementState::Pressed => Self::KeyPressed(key),
            glium::glutin::event::ElementState::Released => Self::KeyReleased(key),
        }
    }

    // https://docs.rs/glutin/0.21.2/glutin/enum.DeviceEvent.html
    fn parse_mouvement_souris(ev: (f64, f64)) -> Self {
        return Self::MouseMove(ev.0, ev.1);
    }

    // https://docs.rs/glutin/0.21.2/glutin/enum.DeviceEvent.html
    fn parse_bouton_souris(ev: u32, state: glium::glutin::event::ElementState) -> Self {
        let button = match ev {
            1 => Button::LeftClick,
            2 => Button::CentralClick,
            3 => Button::RightClick,
            other => Button::Other(other),
        };
        match state {
            glium::glutin::event::ElementState::Pressed => Self::ButtonPressed(button),
            glium::glutin::event::ElementState::Released => Self::ButtonReleased(button),
        }
    }

    // https://docs.rs/glutin/0.21.2/glutin/enum.MouseScrollDelta.html
    fn parse_scroll(ev: glium::glutin::event::MouseScrollDelta) -> Self {
        match ev {
            glium::glutin::event::MouseScrollDelta::LineDelta(x, y) => Self::ScrollMouse(x, y),

            _ => Self::Default,
        }
    }
}

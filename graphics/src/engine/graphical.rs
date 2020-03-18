use super::camera::*;
use super::frame::*;
use super::programs::*;
use super::display::*;
use super::params::*;
use base::Base;

/**
Owns the various components needed to display things on the screen.
*/
pub struct Graphical<'a>
{
    pub parameters: Params<'a>,
    pub display: Display,
    pub program: ProgramsHolder,
    pub camera: Camera,
}

use crate::ressource_handling::ressources_holder::RessourcesHolder;

impl<'a> Graphical<'a>
{
    /** Constructor of Graphical */
    pub fn new(event_loop: &glutin::EventsLoop, base: &Base, holder: &mut RessourcesHolder) -> Self
    {
	let display = Display::new(event_loop) ;
	let params = Params::new();
        let mut program = ProgramsHolder::new();
        program.update(&display, base, holder);
        Self
        {
            parameters: params,
            display: display,
	    program: program,
            camera: Camera::new(2.0),
        }
    }

    pub fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }

    pub fn update_dimensions(&mut self)
    {
        let (w, h) = self.display.display.get_framebuffer_dimensions();
        self.camera.set_aspect_ratio(w as f32, h as f32);
    }

}

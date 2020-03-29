use super::{Camera, Frame, Display, ProgramsHolder};
use crate::ressource_handling::RessourcesHolder;
use base::Base;
use glium::glutin::event_loop::EventLoop;

/**
Owns the various components needed to display things on the screen.
*/
pub struct Graphical
{
    pub display: Display,
    pub program: ProgramsHolder,
    pub camera: Camera,
}


impl Graphical
{
    /// Constructor of Graphical
    pub fn new(event_loop: &EventLoop<()>, base: &Base, holder: &mut RessourcesHolder) -> Self
    {
	let display = Display::new(event_loop) ;
        let mut program = ProgramsHolder::new();
        program.update(&display, base, holder);
        Self
        {
            display: display,
	    program: program,
            camera: Camera::new(2.0),
        }
    }

    /// Generates a Frame to be drawn onto
    pub fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }

    /// Update the aspect ratio of the camera, taking in account the current dimension of the context
    pub fn update_dimensions(&mut self)
    {
        let (w, h) = self.display.display.get_framebuffer_dimensions();
        self.camera.set_aspect_ratio(w as f32, h as f32);
    }

}

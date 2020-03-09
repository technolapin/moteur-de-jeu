/**
Wrapper around the "Display" struct from glium
*/
pub struct Display
{    
    pub display: glium::Display,
}


impl Display
{
   pub fn new(event_loop: &glutin::EventsLoop) -> Self
   {	
	let wb = glutin::WindowBuilder::new();  
	let cb = glutin::ContextBuilder::new().with_depth_buffer(24); 
	Self
	{	  
		display: glium::Display::new(wb, cb, event_loop).unwrap()   
	}
   }
}
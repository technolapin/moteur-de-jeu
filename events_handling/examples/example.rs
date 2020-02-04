extern crate events_handling;
use events_handling::EventsHandler;

use std::{thread, time};


fn main()
{
    let mut event_loop = glutin::EventsLoop::new();
    let mut handler = EventsHandler::new(&mut event_loop);

    let step = time::Duration::from_millis(1000/60);

    loop
    {
        thread::sleep(step);
        handler.debug();
    }
    
    
}



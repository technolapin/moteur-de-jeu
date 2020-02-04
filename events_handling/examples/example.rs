extern crate events_handling;
use events_handling::EventsHandler;

use std::{thread, time};


fn main()
{
    let mut event_loop = glutin::EventsLoop::new();
    let mut handler = EventsHandler::new(&mut event_loop);

    let step = time::Duration::from_millis(1000/60); // 60 fps

    loop
    {
        thread::sleep(step);
        while !handler.update(){}
        let devices = handler.state();
        println!("KEYBOARD: {:?}", devices.keyboard_state);
        println!("MOUSE BUTTONS: {:?}", devices.mouse_state);
        println!("MOUSE MOVE: {:?}", devices.mouse_move);
        println!("MOUSE SCROLL: {:?}", devices.mouse_scroll);
    }
    
    
}



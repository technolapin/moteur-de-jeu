/*
 * An example of how to use this crate to use the keyboard
 */

extern crate events_handling;
use events_handling::EventsHandler;
use events_handling::Key;

use std::{thread, time};


fn main()
{
    let mut event_loop = glutin::EventsLoop::new();
    let mut handler = EventsHandler::new(&mut event_loop);

    let step = time::Duration::from_millis(1000/60); // 60 fps

    let mut pos_x = 0isize;
    let mut pos_y = 0isize;

    loop
    {
        thread::sleep(step);
        while !handler.update(){}
        let devices = handler.state();
        if devices.keyboard_state.contains(&Key::Q)
        {
            pos_x-=1;
        }
        if devices.keyboard_state.contains(&Key::D)
        {
            pos_x+=1;
        }
        if devices.keyboard_state.contains(&Key::Z)
        {
            pos_y-=1;
        }
        if devices.keyboard_state.contains(&Key::S)
        {
            pos_y+=1;
        }
        println!("pos_x: {}    pos_y: {}", pos_x, pos_y);
    }
    
    
}



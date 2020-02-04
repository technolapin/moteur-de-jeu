use std::collections::{VecDeque, HashSet};

use crate::events::Event;
use crate::{Key, Button};



// The Ressource (the ECS way) managing the inputs
pub struct EventsCollector<'a>
{
    events: VecDeque<Event>,
    event_loop: &'a mut glutin::EventsLoop,
}

impl<'a> EventsCollector<'a>
{
    pub fn new(event_loop: &'a mut glutin::EventsLoop) -> Self
    {
        Self
        {
            events: VecDeque::new(),
            event_loop: event_loop,
        }
    }

    // get the parsed events
    pub fn gather(&mut self)
    {
        // we need an intermediate queue to avoid double &mut
        let mut collector = VecDeque::new();
        self.event_loop.poll_events(|event|
                                    {
                                        collector.push_back(Event::parse(event))
                                    });
        self.events.append(&mut collector);
    }

    
    pub fn debug(&mut self)
    {
        while let Some(ev) = self.events.pop_front()
        {
            println!("{:?}", ev);
        }
    }

    pub fn drain(&mut self) -> std::collections::vec_deque::Drain<Event>
    {
        self.events.drain(..)
    }

}


pub struct EventsHandler<'a>
{
    collector: EventsCollector<'a>,
    keyboard_state: HashSet<Key>,
    mouse_state: HashSet<Button>,
    mouse_move: (f64, f64)
}

impl<'a> EventsHandler<'a>
{
    pub fn new(event_loop: &'a mut glutin::EventsLoop) -> Self
    {
        Self
        {
            collector: EventsCollector::new(event_loop),
            keyboard_state: HashSet::new(),
            mouse_state: HashSet::new(),
            mouse_move: (0., 0.)
                
        }
    }

    
    fn update(&mut self) -> bool
    {
        self.mouse_move = (0., 0.);
        self.collector.gather();
        let events: Vec<Event> = self.collector.drain().collect();
        if events.len() == 0
        {
            false
        }
        else
        {
            events.iter().for_each(
                |&ev|
                {
                    match ev {
                        Event::KeyPressed(key) => {self.keyboard_state.insert(key);},
                        Event::KeyReleased(key) => {self.keyboard_state.remove(&key);},
                        Event::ButtonPressed(button) => {self.mouse_state.insert(button);},
                        Event::ButtonReleased(button) => {self.mouse_state.remove(&button);},
                        Event::MouseMove(x, y) => {self.mouse_move = (self.mouse_move.0+x, self.mouse_move.1+y);}
                        _ => ()
                    }
                }
            );
            true
        }
    }
    
    pub fn debug(&mut self)
    {
        while !self.update(){}
        println!("KEYBOARD: {:?}", self.keyboard_state);
        println!("MOUSE BUTTONS: {:?}", self.mouse_state);
        println!("MOUSE MOVE: {:?}", self.mouse_move);
    }
}

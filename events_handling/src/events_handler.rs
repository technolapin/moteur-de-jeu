use crate::{DevicesState, EventsCollector, Event};

/// The structure deducing the keyboard, mouse & co 's states from the events collected
pub struct EventsHandler<'a>
{
    collector: EventsCollector<'a>,
    devices: DevicesState
}

    
impl<'a> EventsHandler<'a>
{
    pub fn new(event_loop: &'a mut glutin::EventsLoop) -> Self
    {
        Self
        {
            collector: EventsCollector::new(event_loop),
            devices: DevicesState::new()
        }
    }


    pub fn state(&self) -> &DevicesState
    {
        &self.devices
    }
    
    pub fn update(&mut self) -> bool
    {
        self.devices.mouse_move = (0., 0.);
        self.devices.mouse_scroll = (0., 0.);
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
                        Event::KeyPressed(key) => {self.devices.keyboard_state.insert(key);},
                        Event::KeyReleased(key) => {self.devices.keyboard_state.remove(&key);},
                        Event::ButtonPressed(button) => {self.devices.mouse_state.insert(button);},
                        Event::ButtonReleased(button) => {self.devices.mouse_state.remove(&button);},
                        Event::MouseMove(x, y) => {self.devices.mouse_move = (self.devices.mouse_move.0+x, self.devices.mouse_move.1+y);}
                        Event::ScrollMouse(x, y) => {self.devices.mouse_scroll = (self.devices.mouse_scroll.0+x, self.devices.mouse_scroll.1+y);}
                        _ => ()
                    }
                }
            );
            true
        }
    }

}


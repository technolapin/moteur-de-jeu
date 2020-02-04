use std::collections::VecDeque;
use crate::Event;




/// The structure collecting the events
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

    

    pub fn drain(&mut self) -> std::collections::vec_deque::Drain<Event>
    {
        self.events.drain(..)
    }

}



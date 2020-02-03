extern crate events_handling;
use events_handling::Event;
use events_handling::EventsCollector;

fn main()
{
    let mut event_loop = glutin::EventsLoop::new();
    let mut collector = EventsCollector::new(&mut event_loop);

    loop
    {
        collector.gather();
        collector.debug();
    }
    
    
}



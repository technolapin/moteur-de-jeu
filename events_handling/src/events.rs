
#[derive(Debug)]
pub enum Event
{
    Lol
}

impl Event
{
    pub fn parse(ev: glutin::Event) -> Self
    {
        Self::Lol
    }
}

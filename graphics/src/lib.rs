#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true)
    }
}

extern crate glium;

pub mod engine;
pub mod misc;
pub mod ressource_handling;
pub use engine::*;

pub use glium;



#[cfg(test)]
mod tests {
    use crate::*;
    use std::path::PathBuf;
    use rand::Rng;
    #[test]
    fn write_read_integrity() -> Result<(), EngineError>
    {
        let base = Base::new();

        let ressource = PathBuf::from("lol");

        let mut rng = rand::thread_rng();
        let content = (0..1000000).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
        base.write_ressource(ressource.clone(), content.clone())?;
        let buffer = base.read_ressource(ressource)?;

        let difference = content.iter().zip(buffer.iter())
            .find(|(a, b)| a!=b);
        assert!(difference.is_none());
        Ok(())
    }
}

// todo: trouver un vrai nom pour base
mod base;
mod errors;

pub use self::base::*;
pub use self::errors::*;

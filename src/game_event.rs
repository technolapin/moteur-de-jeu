
#[derive(Debug)]
pub enum GameEvent
{
    /// Requests the end of the run() call of the Game
    QuitRequested,
    
    /// Requests for a number of GameStates to be popped
    Pop(usize),

    /// Requests for a number of GameStates to be pushed
    Push(String),

    /**
    Requests to play a sound of the given name and at the given position.
    If no position is given, the sound will be played globaly.
     */
    PlaySound(String,Option<[f32; 3]>),

    /// Same as PlaySound, but with a time limit
    PlaySoundTimeLimit(String,Option<f32>,Option<[f32; 3]>),

    /// Request a lowering of the global Volume.
    LowerVolume,

    /// Request a raise of the global Volume.
    RaiseVolume

}

use ears::Sound;
use ears::AudioController;
use ears::SoundData;
use std::time::Instant;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use base::EngineError;

/**
Ears use internally libsndfile so the format of sound ressources format must match libsndfile accepted formats.
including: WAV, FLAC, PAF
 */


/**
Use to play and control a sound
end == -1 -> play all the sound 
end > 0 -> play the sound during end time
end == -2  -> repeat the sound 
 */
pub struct OneSound
{
    pub music: Sound,
    pub start : Instant,
    pub end :  f32,

}



impl OneSound
{
    
    /// Constructor from the path of a sound file
    pub fn new(path_given : &str) -> Self 
    { 
	Self { music : Sound::new(path_given).unwrap(),
               start : Instant::now(),
               end : -1.,  
        } 
    }

    /// Constructor from a SoundRessource
    pub fn new_from_data(sound_data: SoundRessource) -> Result<Self, EngineError>
    {
        let sound = Sound::new_with_data(sound_data.data)?;
        Ok(Self{music: sound, start : Instant::now(), end : -1.,})
    }
    
    /// Fix the duration of the sound
    pub fn set_end(&mut self,duration: f32)
    {
	self.end=duration;
    }

    /// Plays the sound
    pub fn play_all(&mut self) 
    {
	self.music.play();
        self.start=Instant::now();
    }
    
    /// Checks is the sound is playing
    pub fn is_playing(& self)-> bool
    {
	return self.music.is_playing()
    }

    /// Stops the sound
    pub fn stop(&mut self )
    {
	self.music.stop();
    }

    /// Places spatialy the sound
    pub fn give_position(&mut self,position: [f32; 3])
    {
	self.music.set_position(position)
    }

    /// Raises the volume of the sound
    pub fn up_volume(&mut self)
    {
	let mut now=self.get_vol();
	if now+0.1 <=  1 as f32
	{  now=now+0.1;
           self.set_vol(now);
	}

    }
    
    /// Lowers the volume of the sound
    pub fn down_volume(&mut self)
    {
	let mut now=self.get_vol();
	if now-0.1 >= 0 as f32
	{  now=now-0.1;
           self.set_vol(now);
	}

    }

    /// Returns the current volume
    pub fn get_vol(&self) -> f32
    {
	self.music.get_volume()
    }

    /// Sets the volume
    pub fn set_vol(&mut self, vol: f32)
    {
	self.music.set_volume(vol);
    }
    
  

}


impl fmt::Debug for OneSound {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      
	write!(f, "OneSound")
    }
}


/// Sample of a sound
pub struct SoundRessource
{
   pub data : Rc<RefCell<SoundData>>
} 


impl SoundRessource{

    /// Constructor from the path of a sound file
    pub fn new(path: &str) -> Self
    {
        Self{ data : Rc::new(RefCell::new(SoundData::new(path).unwrap()))}
             
    }
    
    /// Constructor from a SoundRessource
    pub fn new_from_data(datas: &SoundRessource)-> Self
    {
       Self{ data : datas.data.clone()}
    }

}

impl fmt::Debug for SoundRessource {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      
	write!(f, "SoundRessource")
    }
}




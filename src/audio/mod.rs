pub mod music;
pub mod sound;

pub struct Audio {
    pub music: music::Music,
    pub sound: sound::Sound
}

impl Audio {
    pub fn new() -> Audio {
        Audio{
            music: music::Music{},
            sound: sound::Sound{}
        }
    }
}

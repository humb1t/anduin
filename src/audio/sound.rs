extern crate ears;

use self::ears::{Sound as SSound, AudioController};
use std::thread;

pub struct Sound {}

impl Sound {
    pub fn play(&self, file_path: &str) {
        ears::init();
        thread::spawn(|| {
            let mut sound = SSound::new("resources/shot.wav").expect("Error on Sound loading.");
            sound.play();
            while sound.is_playing() {}
        });
    }
}

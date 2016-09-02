extern crate ears;

use self::ears::{Music as MMusic, AudioController};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub struct Music {}

impl Music {
    pub fn play(&self, file_path: &str) {
        thread::spawn(|| {
            let mut music = MMusic::new("resources/music.ogg").unwrap();
            music.play();
            while music.is_playing() {
                sleep(Duration::from_millis(1000));
            }
        });
    }
}

extern crate ears;

use self::ears::{Music as EarsMusic, AudioController};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use audio;

pub struct Music {
    data: audio::AudioData,
}

impl audio::PlaybackController for  Music {

    fn new(file_name: &str) -> Music {
        Music {
            data: audio::AudioData {
                file_path: String::from(file_name), ..Default::default()
            }
        }
    }

    fn play(&self) {
        let file_name = self.data.file_path.clone();
        let mut music = match EarsMusic::new(&file_name) {
            Some(music) => music,
            None => panic!("Cannot load music")
        };
        thread::spawn(move || {
            music.play();
            while music.is_playing() {
                sleep(Duration::from_millis(1000));
            }
        });
    }
}

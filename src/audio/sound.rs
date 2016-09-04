extern crate ears;

use self::ears::{Sound as EarsSound, AudioController};
use std::thread;
use audio;

pub struct Sound {//TODO: add several instances
    data: audio::AudioData,
}

impl audio::PlaybackController for Sound {
    fn new(file_name: &str) -> Sound {
        Sound {
            data: audio::AudioData {
                file_path: String::from(file_name), ..Default::default()
            }
        }
    }

    fn play(&self) {
        ears::init(); //TODO: move to sound init
        //TODO: move without clone?
        let file_name = self.data.file_path.clone();
        thread::spawn(move || {
        let mut sound = EarsSound::new(&file_name)//"resources/shot.wav"
            .expect("Error on Sound loading.");
            sound.play();
            while sound.is_playing() {}
        });
    }

    fn pause(&self) {

    }
}

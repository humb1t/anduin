pub mod music;
pub mod sound;

struct AudioData {
    direction: [f32; 3],
    distance: f32,
    position: [f32; 3],
    is_relative: bool,
    is_playing: bool,
    is_looping: bool,
    volume: f32,
    max_volume: f32,
    min_volume: f32,
    pan: f32,
    pitch: f32,
    file_path: String
}

impl Default for AudioData {
    fn default() -> AudioData {
        AudioData {
            direction: [0.0,0.0,0.0],
            distance: 1.0,
            position: [0.0,0.0,0.0],
            is_relative: false,
            is_playing: false,
            is_looping: false,
            volume: 1.0,
            max_volume: 1.0,
            min_volume: 0.0,
            pan: 0.0, //from -1.0 left to 1.0 right
            pitch: 1.0, //pitch the pitch multiplier, 1 == default, >1 == faster, <1 == slower, the value has to be between 0.5 and 2.0
            file_path: String::from("")
        }
    }
}

pub trait PlaybackController {
    fn new(file_path: &str) -> Self;
    fn play(&self) {}
    fn pause(&self) {}
    fn stop(&self) {}
    fn resume(&self) {}
    fn play_in_loop(&self) {}
}

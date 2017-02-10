// Must get application and manipulate with it - based on Application LC
// Start: This is where the program performs any initialization.
// Stop: Here, any open resources should be released before exiting.
// Update: All the game logic goes here, modifying the state of objects.
// Draw: This is where all the drawing goes.
use core;
use logic::ApplicationListener;
use time;
use input;
use std::thread;

#[derive(Debug)]
pub struct GameLoop {
    pub max_skipped_frames: u16,
    pub max_time_dif: time::Duration,
    skipped_frames: u16,
}

impl GameLoop {
    pub fn default() -> Self {
        GameLoop {
            max_skipped_frames: 5,
            max_time_dif: time::Duration::milliseconds(1000),
            skipped_frames: 0
        }
    }

    pub fn new(max_skipped_frames: u16, max_time_diff: time::Duration) -> Self {
        GameLoop {
            max_skipped_frames: max_skipped_frames,
            max_time_dif: max_time_diff,
            skipped_frames: 0
        }
    }


    pub fn run(&mut self, application: &mut core::Application) {
        println!("GameLoop::run({:?})", application);
        let mut next_time = time::now();
        while !application.graphics.should_close {
            match application.lifetime {
                Some(lifetime) => {
                    match lifetime {
                        0 => {
                            application.graphics.should_close = true;
                            application.input.stop();
                        },
                        _ => {
                            let delta_time = application.graphics.delta_time.num_seconds() as u64;
                            application.lifetime = Some(lifetime - delta_time)
                        }
                    }
                },
                None => ()
            }
            let curr_time = time::now();
            application.input.handle();
            println!("GameLoop::run \n \r curr_time = {}", curr_time.ctime());
            println!("GameLoop::run \n \r next_time = {}", next_time.ctime());
            if (curr_time - next_time) > self.max_time_dif {
                next_time = curr_time;
            }
            application.listener.as_mut().update();
            if curr_time >= next_time {
                next_time = next_time + application.graphics.delta_time;
                let is_time_to_render = curr_time < next_time;
                let is_too_much_frames_skipped = self.skipped_frames > self.max_skipped_frames;
                let is_render: bool = is_time_to_render || is_too_much_frames_skipped;
                match is_render {
                    true => {
                        application.listener.as_ref().render();
                        self.skipped_frames = 1;
                    },
                    false => {
                        self.skipped_frames += 1;
                    }
                }
            } else {
                let delta_time = next_time - curr_time;
                GameLoop::sleep(delta_time);
            }
        }
        application.listener.as_mut().exit();
    }

    fn sleep(time_to_sleep: time::Duration) {
        println!("GameLoop::run \n \r time_to_sleep = {}", time_to_sleep);
        match time_to_sleep.to_std() {
            Ok(duration) => thread::sleep(duration),
            Err(err) => println!("GameLoop::run \n \r Error {:?}", err),
        }
    }
}

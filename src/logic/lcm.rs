// Must get application and manipulate with it - based on Application LC
// Start: This is where the program performs any initialization.
// Stop: Here, any open resources should be released before exiting.
// Update: All the game logic goes here, modifying the state of objects.
// Draw: This is where all the drawing goes.
use logic::{Application, ApplicationListener};
use time;
use std::thread;

#[derive(Debug)]
pub struct GameLoop {
    pub max_skipped_frames: u16,
    pub max_time_dif: time::Duration,
}

impl GameLoop {
    pub fn new() -> Self {
        GameLoop {
            max_skipped_frames: 5,
            max_time_dif: time::Duration::milliseconds(100),
        }
    }

    pub fn run(&self, app: &mut Application) {
        let mut next_time = time::now();
        let mut skipped_frames = 1;

        while !app.graphics.should_close {
            match app.lifetime {
                Some(x) => {
                    if x <= 0 {app.graphics.should_close = true}
                    else {app.lifetime = Some(x-app.graphics.delta_time.num_seconds() as u64)}
                },
                None => ()
            }
            app.process_input();
            let curr_time = time::now();
            println!("curr_time = {:?}", curr_time);
            println!("next_time = {:?}", next_time);
            if (curr_time - next_time) > self.max_time_dif {
                next_time = curr_time;
            }
            if curr_time >= next_time {
                next_time = next_time + app.graphics.delta_time;
                app.update();
                if (curr_time < next_time) || (skipped_frames > self.max_skipped_frames) {
                    app.render();
                    skipped_frames = 1;
                } else {
                    skipped_frames += 1;
                }
            } else {
                let time_to_sleep = next_time - curr_time;
                println!("time_to_sleep = {}", time_to_sleep);
                if !time_to_sleep.is_zero() {
                    match time_to_sleep.to_std() {
                        Ok(duration) => thread::sleep(duration),
                        Err(err) => println!("Error {:?}", err),
                    }
                }
            }
        }
        app.exit();
    }
}

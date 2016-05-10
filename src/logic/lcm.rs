//Must get application and manipulate with it - based on Application LC
//Start: This is where the program performs any initialization.
//Stop: Here, any open resources should be released before exiting.
//Update: All the game logic goes here, modifying the state of objects.
//Draw: This is where all the drawing goes.
use logic::{Application, ApplicationListener};
use time;
use std::thread;
use std::time::Duration;

pub struct GameLoop {
    pub max_skipped_frames: u16,
    pub max_time_dif: i64
}

impl GameLoop {
        pub fn new() -> Self {
            GameLoop {
                max_skipped_frames: 5,
                max_time_dif: 1
            }
        }

        pub fn run<T: ApplicationListener>(&self, app: &mut Application<T>) {
            let mut next_time = time::get_time().sec;
            let mut skipped_frames = 1;
            while !app.graphics.window.should_close() {
                app.process_input();
                let curr_time = time::get_time().sec;
                println!("curr_time = {}", curr_time);
                println!("next_time = {}", next_time);
                if (curr_time - next_time) > self.max_time_dif {
                     next_time = curr_time;
                 }
                if curr_time >= next_time {
                    next_time += app.graphics.delta_time;
                    app.app_listener.update();
                    if (curr_time < next_time) || (skipped_frames > self.max_skipped_frames) {
                         app.app_listener.render();
                         skipped_frames = 1;
                     } else {
                         skipped_frames += 1;
                     }
                } else {
                    let time_to_sleep = next_time - curr_time;
                    println!("time_to_sleep = {}", time_to_sleep);
                    if time_to_sleep > 0 {
                         thread::sleep(Duration::from_secs(time_to_sleep as u64));
                     }
                }
            }
            app.exit();
        }
}

// Must get application and manipulate with it - based on Application LC
// Start: This is where the program performs any initialization.
// Stop: Here, any open resources should be released before exiting.
// Update: All the game logic goes here, modifying the state of objects.
// Draw: This is where all the drawing goes.
use logic::{Application, ApplicationAdapter, ApplicationListener};
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

    pub fn run(&self, application: &mut ApplicationAdapter){
        let mut next_time = time::now();
        let mut skipped_frames = 1;
        while !application.get_application().graphics.should_close{
            match application.get_application().lifetime {
                Some(x) => {
                    if x <= 0 {application.get_application().graphics.should_close = true}
                    else { application.get_application().lifetime = Some(x-application.get_application().graphics.delta_time.num_seconds() as u64)}
                },
                None => ()
            }
            application.update();
            let curr_time = time::now();
            println!("curr_time = {:?}", curr_time);
            println!("next_time = {:?}", next_time);
            if (curr_time - next_time) > self.max_time_dif {
                next_time = curr_time;
            }
            if curr_time >= next_time {
                next_time = next_time + application.get_application().graphics.delta_time;
                if (curr_time < next_time) || (skipped_frames > self.max_skipped_frames) {
                    application.render();
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
        application.exit();
    }
}

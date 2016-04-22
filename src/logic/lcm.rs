//Must get application and manipulate with it - based on Application LC
//Start: This is where the program performs any initialization.
//Stop: Here, any open resources should be released before exiting.
//Update: All the game logic goes here, modifying the state of objects.
//Draw: This is where all the drawing goes.
use logic::ApplicationListener;

pub struct Loop {
    pub runned: bool
}

pub trait Runnable {
    fn new(runned: bool) -> Self;
    fn run<T: ApplicationListener>(&self, app: T, delta: i32);
}

impl Runnable for Loop {
        fn new(runned: bool) -> Self {
            Loop {
                runned: runned
            }
        }

        fn run<T: ApplicationListener>(&self, app: T, delta: i32) {
            app.create();
            while self.runned {
                app.render();
            }
            app.dispose();
        }
}

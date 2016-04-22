mod scene;
mod rules;
pub mod lcm;
mod physic;
mod ai;

pub struct Application {
    pub name: &'static str,
    pub platform: &'static str
}

pub trait ApplicationListener {
    fn new(name: &'static str, platform: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn platform(&self) -> &'static str;

    fn create(&self){
        println!("Create application {} for platform {}",
         self.name(), self.platform());
    }

    fn resize(&self, width: i32, height: i32){
            println!("Resize to {}x{}",
             width, height);
    }
    fn update(&self);
    fn render(&self);
    fn pause(&self);
    fn resume(&self);
    fn dispose(&self);
}

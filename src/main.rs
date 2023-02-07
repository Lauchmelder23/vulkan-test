use application::Application;

mod application;
mod window;
mod error;

fn main() {
    let mut app = Application::new().unwrap();    
    
    if let Err(err) = app.run() {
        eprintln!("Application terminated unexplectedly: {err}");
        return;
    }
}

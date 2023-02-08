use application::Application;

mod application;
mod window;
mod error;

fn main() {
    let mut app = match Application::new() {
        Ok(app) => app,
        Err(err) => { eprintln!("{err}"); return; }
    };
    
    if let Err(err) = app.run() {
        eprintln!("Application terminated unexplectedly: {err}");
        return;
    }
}

use crate::window::Window;

fn key_callback_func(window: &mut glfw::Window, key: glfw::Key, _: glfw::Scancode, action: glfw::Action, _: glfw::Modifiers) {
    if key == glfw::Key::Escape && action == glfw::Action::Press {
        window.set_should_close(true);
    }
}

pub struct Application {
    glfw: glfw::Glfw,
    window: Window
}

impl Application {
    pub fn new() -> Result<Application, &'static str> {
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(instance) => instance,
            Err(_) => return Err("Failed to initialize GLFW")
        };

        let mut window = Window::new(&mut glfw, 800, 800, "Vulkan Test")?;
        window.set_key_callback(Some(key_callback_func));

        Ok(Application {
            glfw: glfw,
            window: window
        })
    }

    pub fn run(&mut self) -> Result<(), &'static str> {
        while !self.window.should_close() {
            self.glfw.poll_events();
            self.window.handle_events();
        }

        Ok(())
    }

}

impl Drop for Application {
    fn drop(&mut self) {
        
    }
}
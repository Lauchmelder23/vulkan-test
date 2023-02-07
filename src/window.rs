use std::{sync::mpsc::Receiver, ops::Deref};

type KeyCallbackFunc = fn(&mut glfw::Window, glfw::Key, glfw::Scancode, glfw::Action, glfw::Modifiers);

pub struct Window {
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,

    key_callback: Option<KeyCallbackFunc>
}

impl Window {
    pub fn new(glfw: &mut glfw::Glfw, width: u32, height: u32, title: &str) -> Result<Window, &'static str> {
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (mut window, events) = match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
            Some(val) => val,
            None => return Err("Failed to create window and event receiver") 
        };

        window.set_key_polling(true);

        Ok(Window {
            window: window,
            events: events,

            key_callback: None
        })
    }

    pub fn set_key_callback(&mut self, callback: Option<KeyCallbackFunc>) {
        self.key_callback = callback;
    }

    pub fn handle_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, scancode, action, mods) => if let Some(func) = self.key_callback {
                    func(&mut self.window, key, scancode, action, mods)
                },
                _ => {}
            }
        }
    }
}

impl Deref for Window {
    type Target = glfw::Window;

    fn deref(&self) -> &Self::Target {
        return &self.window;
    }
}
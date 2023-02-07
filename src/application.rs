use std::sync::Arc;

use crate::window::Window;

fn key_callback_func(window: &mut glfw::Window, key: glfw::Key, _: glfw::Scancode, action: glfw::Action, _: glfw::Modifiers) {
    if key == glfw::Key::Escape && action == glfw::Action::Press {
        window.set_should_close(true);
    }
}

pub struct Application {
    glfw: glfw::Glfw,
    vulkan: Arc<vulkano::instance::Instance>,

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

        let vulkan = Application::init_vulkan(&glfw)?;

        Ok(Application {
            glfw: glfw,
            vulkan: vulkan,

            window: window
        })
    }

    fn init_vulkan(glfw: &glfw::Glfw) -> Result<Arc<vulkano::instance::Instance>, &'static str> {
        let mut create_info = vulkano::instance::InstanceCreateInfo::application_from_cargo_toml();
        create_info.engine_name = Some("No engine".into());
        create_info.engine_version = vulkano::Version::V1_0;
        create_info.max_api_version = Some(vulkano::Version::V1_0);

        if let Some(extensions) = glfw.get_required_instance_extensions() {
            create_info.enabled_extensions = vulkano::instance::InstanceExtensions::from_iter(extensions.iter().map(|name| name.as_str()));
        } else {
            return Err("Failed to fetch GLFW's required instance extensions");
        }

        if let Ok(library) = vulkano::VulkanLibrary::new() {
            return match vulkano::instance::Instance::new(library, create_info) {
                Ok(instance) => Ok(instance),
                Err(_) => Err("Failed to create Vulkan instance")
            };
        }

        return Err("Failed to create Vulkano library");
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
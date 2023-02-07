use std::sync::Arc;

use crate::{window::Window, error::ApplicationError};

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
    pub fn new() -> Result<Application, ApplicationError> {
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(instance) => instance,
            Err(_) => return Err(ApplicationError::new("GLFW", "Failed to initialize GLFW"))
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

    fn init_vulkan(glfw: &glfw::Glfw) -> Result<Arc<vulkano::instance::Instance>, ApplicationError> {
        let mut create_info = vulkano::instance::InstanceCreateInfo::application_from_cargo_toml();
        create_info.engine_name = Some("No engine".into());
        create_info.engine_version = vulkano::Version::V1_0;
        create_info.max_api_version = Some(vulkano::Version::V1_0);

        let required_extensions = glfw.get_required_instance_extensions().ok_or(
            ApplicationError::new("GLFW", "Failed to get required GLFW extensions")
        )?;

        create_info.enabled_extensions = vulkano::instance::InstanceExtensions::from_iter(
            required_extensions.iter().map(|ext| ext.as_str())
        );

        let library = vulkano::library::VulkanLibrary::new()?;
        let instance = vulkano::instance::Instance::new(library, create_info)?;

        Ok(instance)
    }

    pub fn run(&mut self) -> Result<(), ApplicationError> {
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
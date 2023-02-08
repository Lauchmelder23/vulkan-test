use std::sync::Arc;

use vulkano::buffer::ExternalBufferInfo;

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
        let library = vulkano::library::VulkanLibrary::new()?;

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

        if cfg!(debug_assertions) {
            println!("Enabled extensions:");
            for extension in required_extensions {
                println!("\t{extension}");
            }

            Application::register_validation_layers(library.clone(), &mut create_info)?;
        }

        let instance = vulkano::instance::Instance::new(library, create_info)?;

        

        Ok(instance)
    }

    #[cfg(debug_assertions)]
    fn register_validation_layers(library: Arc<vulkano::library::VulkanLibrary>, create_info: &mut vulkano::instance::InstanceCreateInfo) -> Result<(), ApplicationError>{
        const VALIDATION_LAYERS: [&'static str; 1] = [
            "VK_LAYER_KHRONOS_validation"
        ];

        Application::check_validation_layer_support(library.clone(), &VALIDATION_LAYERS)?;
        create_info.enabled_layers = VALIDATION_LAYERS.iter().map(|&layer| layer.into()).collect::<Vec<String>>();

        if cfg!(debug_assertions) {
            println!("Enabled validation layers:");
            for layer in VALIDATION_LAYERS {
                println!("\t{layer}");
            }
        }

        Ok(())
    }

    #[cfg(debug_assertions)]
    fn check_validation_layer_support(library: Arc<vulkano::library::VulkanLibrary>, layers: &[&'static str]) -> Result<(), ApplicationError>{
        let mut supported_layers = library.layer_properties()?;
       
        let mut missing_validation_layer: &str = "";

        let contains_all_layers = layers.iter().all(|&layer| {
            if supported_layers.find(|supported_layer| 
                supported_layer.name() == layer
            ).is_none() {
                missing_validation_layer = layer;
                return false;
            }

            true
        });

        if !contains_all_layers {
            return Err(ApplicationError::new("Validation Layers", format!("This driver does not support the \"{missing_validation_layer}\" validation layer").as_str()));
        }

        Ok(())
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
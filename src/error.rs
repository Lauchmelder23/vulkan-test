use std::fmt::Display;

use vulkano::{LoadingError, instance::{self, InstanceCreationError, Instance, InstanceCreateInfo}};

#[derive(Debug)]
pub struct ApplicationError {
    what: String,
    which: String
}

impl ApplicationError {
    pub fn new(which: &str, what: &str) -> ApplicationError {
        ApplicationError { what: what.into(), which: which.into() }
    }
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{}] {}", self.which, self.what)
    }
} 

impl From<LoadingError> for ApplicationError {
    fn from(value: LoadingError) -> Self {
        ApplicationError {
            which: "Loading Error".into(),
            what: match value {
                LoadingError::LibraryLoadFailure(error) => error.to_string(),
                LoadingError::OomError(error) => error.to_string()
            }
        }
    }
}

impl From<InstanceCreationError> for ApplicationError {
    fn from(value: InstanceCreationError) -> Self {
        ApplicationError { 
            what: "Instance Creation Error".into(), 
            which: match value {
                InstanceCreationError::ExtensionNotPresent => "Extension not present".into(),
                InstanceCreationError::ExtensionRestrictionNotMet(error) => error.to_string(),
                InstanceCreationError::IncompatibleDriver => "Incompatible driver".into(),
                InstanceCreationError::InitializationFailed => "Initialization failed".into(),
                InstanceCreationError::LayerNotPresent => "Layer not present".into(),
                InstanceCreationError::OomError(error) => error.to_string(),
                InstanceCreationError::RequirementNotMet { required_for, requires_one_of } => format!("{} requires one of {}", required_for, requires_one_of)
            }
        }
    }
}
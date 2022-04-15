use project::Project;

pub mod project;
pub mod types;

pub use serde;
pub use serde_json;

pub fn deserialize_project(json: String) -> Project {
    serde_json::from_str(&json).unwrap()
}

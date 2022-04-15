use project::Project;

pub mod project;
pub mod types;

pub use serde;
pub use serde_json;

pub fn deserialize_project(json: String) -> Result<Project, String> {
    match serde_json::from_str(&json) {
        Ok(project) => Ok(project),
        Err(e) => Err(format!("{:?}", e)),
    }
}

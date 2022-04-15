use serde::Deserialize;

use crate::types::{Enum, Int};

use self::{definitions::Definitions, level::Level};

pub mod definitions;
pub mod instances;
pub mod level;
pub mod tileset;

#[derive(Clone, Deserialize)]
pub struct Project {
    /// Project background color
    #[serde(alias = "bgColor")]
    pub bg_color: String,

    /// A structure containing all the definitions of this project
    pub defs: Definitions,

    /// If TRUE, one file will be saved for the project (incl. all its definitions) and one file in a sub-folder for each level.
    #[serde(alias = "externalLevels")]
    pub external_levels: bool,

    /// File format version
    #[serde(alias = "jsonVersion")]
    pub json_version: String,

    /// All levels. The order of this array is only relevant in LinearHorizontal and linearVertical world layouts (see worldLayout value).
    /// Otherwise, you should refer to the worldX,worldY coordinates of each Level.
    pub levels: Vec<Level>,

    /// WARNING: this field will move to the worlds array after the "multi-worlds" update. It will then be null. You can enable the Multi-worlds advanced project option to enable the change immediately.
    /// Height of the world grid in pixels.
    #[serde(alias = "worldGridHeight")]
    pub world_grid_height: Option<Int>,

    /// WARNING: this field will move to the worlds array after the "multi-worlds" update. It will then be null. You can enable the Multi-worlds advanced project option to enable the change immediately.
    /// Width of the world grid in pixels.
    #[serde(alias = "worldGridWidth")]
    pub world_grid_width: Option<Int>,

    /// WARNING: this field will move to the worlds array after the "multi-worlds" update. It will then be null. You can enable the Multi-worlds advanced project option to enable the change immediately.
    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D space).
    /// Possible values: <null>, Free, GridVania, LinearHorizontal, LinearVertical
    #[serde(alias = "worldLayout")]
    pub world_layout: Option<Enum>,
}

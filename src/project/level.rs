use serde::Deserialize;

use crate::types::{Float, Int};

use super::instances::{field_instance::FieldInstance, layer_instance::LayerInstance};

#[derive(Clone, Deserialize)]
pub struct Level {
    /// Background color of the level (same as bgColor, except the default value is automatically used here if its value is null)
    #[serde(alias = "__bgColor")]
    pub bg_color: String,

    /// Position informations of the background image, if there is one.
    #[serde(alias = "__bgPos")]
    pub bg_pos: Option<BackgroundPosition>,

    /// An array listing all other levels touching this one on the world map.
    /// Only relevant for world layouts where level spatial positioning is manual (ie. GridVania, Free).
    /// For Horizontal and Vertical layouts, this array is always empty.
    #[serde(alias = "__neighbours")]
    pub neighbours: Vec<Neighbour>,

    /// The optional relative path to the level background image.
    #[serde(alias = "bgRelPath")]
    pub bg_rel_path: Option<String>,

    /// This value is not null if the project option "Save levels separately" is enabled.
    /// In this case, this relative path points to the level Json file.
    #[serde(alias = "externalRelPath")]
    pub external_rel_path: Option<String>,

    /// An array containing this level custom field values.
    #[serde(alias = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,

    /// User defined unique identifier
    pub identifier: String,

    /// Unique instance identifier
    pub iid: String,

    /// An array containing all Layer instances. IMPORTANT: if the project option "Save levels separately" is enabled, this field will be null.
    /// This array is sorted in display order: the 1st layer is the top-most and the last is behind.
    #[serde(alias = "layerInstances")]
    pub layer_instances: Option<Vec<LayerInstance>>,

    /// Height of the level in pixels
    #[serde(alias = "pxHei")]
    pub px_hei: Int,

    /// Width of the level in pixels
    #[serde(alias = "pxWid")]
    pub px_wid: Int,

    /// Unique Int identifier
    pub uid: Int,

    /// Index that represents the "depth" of the level in the world. Default is 0, greater means "above", lower means "below".
    /// This value is mostly used for display only and is intended to make stacking of levels easier to manage.
    #[serde(alias = "worldDepth")]
    pub world_depth: Int,

    /// World X coordinate in pixels.
    /// Only relevant for world layouts where level spatial positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the value is always -1 here.
    #[serde(alias = "worldX")]
    pub world_x: Int,

    /// World Y coordinate in pixels.
    /// Only relevant for world layouts where level spatial positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the value is always -1 here.
    #[serde(alias = "worldY")]
    pub world_y: Int,
}

#[derive(Clone, Deserialize)]
pub struct Neighbour {
    /// A single lowercase character tipping on the level location (north, south, west, east).
    pub dir: String,

    /// Neighbour Instance Identifier
    #[serde(alias = "levelIid")]
    pub level_iid: String,
}

#[derive(Clone, Deserialize)]
pub struct BackgroundPosition {
    /// An array of 4 float values describing the cropped sub-rectangle of the displayed background image.
    /// This cropping happens when original is larger than the level bounds. Array format: [ cropX, cropY, cropWidth, cropHeight ]
    #[serde(alias = "cropRect")]
    pub crop_rect: Vec<Float>,

    /// An array containing the [scaleX,scaleY] values of the cropped background image, depending on bgPos option.
    pub scale: Vec<Float>,

    #[serde(alias = "topLeftPx")]
    /// An array containing the [x,y] pixel coordinates of the top-left corner of the cropped background image, depending on bgPos option.
    pub top_left_px: Vec<Int>,
}

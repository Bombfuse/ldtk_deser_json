use serde::Deserialize;

use crate::{
    project::tileset::tileset_rect::TilesetRect,
    types::{Float, Int},
};

use super::field_instance::FieldInstance;

#[derive(Clone, Deserialize)]
pub struct EntityInstance {
    /// Grid-based coordinates ([x,y] format)
    #[serde(alias = "__grid")]
    pub grid_coords: Vec<Int>,

    /// Entity definition identifier
    #[serde(alias = "__identifier")]
    pub identifier: String,

    /// Pivot coordinates ([x,y] format, values are from 0 to 1) of the Entity
    #[serde(alias = "__pivot")]
    pub pivot: Vec<Float>,

    /// The entity "smart" color, guessed from either Entity definition, or one its field instances.
    #[serde(alias = "__smartColor")]
    pub smart_color: String,

    /// Array of tags defined in this Entity definition
    #[serde(alias = "__tags")]
    pub tags: Vec<String>,

    /// Array of tags defined in this Entity definition
    #[serde(alias = "__tile")]
    pub tile: Option<TilesetRect>,

    /// Reference of the Entity definition UID
    #[serde(alias = "defUid")]
    pub def_uid: Int,

    /// An array of all custom fields and their values.
    #[serde(alias = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,

    /// Entity height in pixels. For non-resizable entities, it will be the same as Entity definition.
    pub height: Int,

    /// Unique instance identifier
    pub iid: String,

    /// Pixel coordinates ([x,y] format) in current level coordinate space. Don't forget optional layer offsets, if they exist!
    pub px: Vec<Int>,

    /// Entity width in pixels. For non-resizable entities, it will be the same as Entity definition.
    pub width: Int,
}

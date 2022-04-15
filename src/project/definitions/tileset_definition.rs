use serde::Deserialize;

use crate::types::{Enum, Int};

#[derive(Clone, Deserialize)]
pub struct TilesetDefinition {
    /// Grid-based height
    #[serde(alias = "__cHei")]
    pub c_hei: Int,

    /// Grid-based width
    #[serde(alias = "__cWid")]
    pub c_wid: Int,

    /// An array of custom tile metadata
    #[serde(alias = "customData")]
    pub custom_data: Vec<CustomData>,

    // If this value is set, then it means that this atlas uses an internal LDtk atlas image instead of a loaded one.
    // Possible values: <null>, LdtkIcons
    #[serde(alias = "embedAtlas")]
    pub embed_atlas: Option<Enum>,

    // Tileset tags using Enum values specified by tagsSourceEnumId. This array contains 1 element per Enum value, which contains an array of all Tile IDs that are tagged with it.
    #[serde(alias = "enumTags")]
    pub enum_tags: Vec<EnumTag>,

    /// User defined unique identifier
    pub identifier: String,

    /// Distance in pixels from image borders
    pub padding: Int,

    /// Image height in pixels
    #[serde(alias = "pxHei")]
    pub px_hei: Int,

    /// Image width in pixels
    #[serde(alias = "pxWid")]
    pub px_wid: Int,

    /// Path to the source file, relative to the current project JSON file
    /// It can be null if no image was provided, or when using an embed atlas.
    #[serde(alias = "relPath")]
    pub rel_path: Option<String>,

    /// Space in pixels between all tiles
    pub spacing: Int,

    /// An array of user-defined tags to organize the Tilesets
    pub tags: Vec<String>,

    /// Optional Enum definition UID used for this tileset meta-data
    #[serde(alias = "tagsSourceEnumUid")]
    pub tags_source_enum_uid: Option<Int>,

    #[serde(alias = "tileGridSize")]
    pub tile_grid_size: Int,

    /// Unique Intidentifier
    #[serde(alias = "tileGridSize")]
    pub uid: Int,
}

#[derive(Clone, Deserialize)]
pub struct EnumTag {
    #[serde(alias = "enumValueId")]
    pub enum_value_id: String,

    #[serde(alias = "tileIds")]
    pub tile_ids: Vec<Int>,
}

#[derive(Clone, Deserialize)]
pub struct CustomData {
    pub data: String,

    #[serde(alias = "tileId")]
    pub tile_id: Int,
}

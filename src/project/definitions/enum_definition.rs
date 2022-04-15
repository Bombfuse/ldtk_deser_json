use crate::types::Int;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct EnumDefinition {
    /// Relative path to the external file providing this Enum
    #[serde(alias = "externalRelPath")]
    pub external_rel_path: Option<String>,

    /// Tileset UID if provided
    #[serde(alias = "iconTilesetUid")]
    pub icon_tileset_uid: Option<Int>,

    /// User defined unique identifier
    pub identifier: String,

    /// An array of user-defined tags to organize the Enums
    pub tags: Vec<String>,

    /// Unique Int identifier
    pub uid: Int,

    /// All possible enum values, with their optional Tile infos.
    pub values: Vec<EnumValueDefinition>,
}

#[derive(Clone, Deserialize)]
pub struct EnumValueDefinition {
    ///	An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width, height ]`
    #[serde(alias = "__tileSrcRect")]
    pub tile_src_rect: Option<Vec<Int>>,

    /// Optional color
    pub color: Int,

    /// Enum value
    pub id: String,

    /// The optional ID of the tile
    #[serde(alias = "tileId")]
    pub tile_id: Option<Int>,
}

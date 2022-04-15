use serde::Deserialize;

use crate::types::Int;

#[derive(Clone, Deserialize)]
pub struct TilesetRect {
    /// Height in pixels
    pub h: Int,

    /// Width in pixels
    pub w: Int,

    /// UID of the tileset
    #[serde(alias = "tilesetUid")]
    pub tileset_uid: Int,

    /// X pixels coordinate of the top-left corner in the Tileset image
    pub x: Int,

    /// Y pixels coordinate of the top-left corner in the Tileset image
    pub y: Int,
}

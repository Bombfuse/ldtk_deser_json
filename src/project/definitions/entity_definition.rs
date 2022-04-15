use serde::Deserialize;

use crate::{
    project::tileset::tileset_rect::TilesetRect,
    types::{Float, Int},
};

#[derive(Clone, Deserialize)]
pub struct EntityDefinition {
    /// Base entity color
    pub color: String,

    /// Pixel height
    pub height: Int,

    /// User defined unique identifier
    pub identifier: String,

    /// An array of 4 dimensions for the up/right/down/left borders (in this order) when using 9-slice mode for tileRenderMode.
    /// If the tileRenderMode is not NineSlice, then this array is empty.
    /// See: https://en.wikipedia.org/wiki/9-slice_scaling
    #[serde(alias = "nineSliceBorders")]
    pub nine_slice_borders: Vec<Int>,

    /// Pivot X coordinate (from 0 to 1.0)
    #[serde(alias = "pivotX")]
    pub pivot_x: Float,

    /// Pivot Y coordinate (from 0 to 1.0)
    #[serde(alias = "pivotY")]
    pub pivot_y: Float,

    /// An object representing a rectangle from an existing Tileset
    #[serde(alias = "tileRect")]
    pub tile_rect: Option<TilesetRect>,

    /// An enum describing how the the Entity tile is rendered inside the Entity bounds.
    /// Possible values: Cover, FitInside, Repeat, Stretch, FullSizeCropped, FullSizeUncropped, NineSlice
    #[serde(alias = "tileRenderMode")]
    pub tile_render_mode: String,

    /// Tileset ID used for optional tile display
    #[serde(alias = "tilesetId")]
    pub tileset_id: Option<Int>,

    /// Unique Int identifier
    pub uid: Int,

    /// Pixel width
    pub width: Int,
}

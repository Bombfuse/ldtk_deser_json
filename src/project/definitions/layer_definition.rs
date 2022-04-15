use serde::Deserialize;

use crate::types::{Float, Int};

#[derive(Clone, Deserialize)]
pub struct LayerDefinition {
    /// Type of the layer (IntGrid, Entities, Tiles or AutoLayer)
    #[serde(alias = "__type")]
    pub layer_type: String,

    #[serde(alias = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: Option<Int>,

    /// Opacity of the layer (0 to 1.0)
    #[serde(alias = "displayOpacity")]
    pub display_opacity: Float,

    /// Width and height of the grid in pixels
    #[serde(alias = "gridSize")]
    pub grid_size: Int,

    /// User defined unique identifier
    pub identifier: String,

    /// An array that defines extra optional info for each IntGrid value.
    /// WARNING: the array order is not related to actual IntGrid values! As user can re-order IntGrid values freely, you may value "2" before value "1" in this array.
    #[serde(alias = "intGridValues")]
    pub int_grid_values: Vec<IntGridValue>,

    /// Parallax horizontal factor (from -1 to 1, defaults to 0) which affects the scrolling speed of this layer, creating a fake 3D (parallax) effect.
    #[serde(alias = "parallaxFactorX")]
    pub parallax_factor_x: Float,

    /// Parallax vertical factor (from -1 to 1, defaults to 0) which affects the scrolling speed of this layer, creating a fake 3D (parallax) effect.
    #[serde(alias = "parallaxFactorY")]
    pub parallax_factor_y: Float,

    /// If true (default), a layer with a parallax factor will also be scaled up/down accordingly.
    #[serde(alias = "parallaxScaling")]
    pub parallax_scaling: bool,

    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the LayerInstance optional offset)
    #[serde(alias = "pxOffsetX")]
    pub px_offset_x: Float,

    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the LayerInstance optional offset)
    #[serde(alias = "pxOffsetY")]
    pub px_offset_y: Float,

    /// Reference to the default Tileset UID being used by this layer definition.
    /// WARNING: some layer instances might use a different tileset. So most of the time, you should probably use the __tilesetDefUid value found in layer instances.
    /// Note: since version 1.0.0, the old autoTilesetDefUid was removed and merged into this value.
    #[serde(alias = "tilesetDefUid")]
    pub tileset_def_uid: Option<Int>,

    /// Unique Int identifier
    pub uid: Int,
}

#[derive(Clone, Deserialize)]
pub struct IntGridValue {
    /// Hex color "#rrggbb"
    pub color: String,

    /// User defined unique identifier
    pub identifier: Option<String>,

    /// The IntGrid value itself
    pub value: Int,
}

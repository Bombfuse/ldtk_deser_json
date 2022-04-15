use serde::Deserialize;

use crate::types::Int;

#[derive(Clone, Deserialize)]
pub struct TileInstance {
    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.
    /// - Bit 0 = X flip
    /// - Bit 1 = Y flip
    /// Examples: f=0 (no flip), f=1 (X flip only), f=2 (Y flip only), f=3 (both flips)
    pub f: Int,

    /// Pixel coordinates of the tile in the layer ([x,y] format). Don't forget optional layer offsets, if they exist!
    pub px: Vec<Int>,

    /// Pixel coordinates of the tile in the tileset ([x,y] format)
    pub src: Vec<Int>,

    /// The Tile ID in the corresponding tileset.
    pub t: Int,
}

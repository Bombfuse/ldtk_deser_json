use crate::{project::tileset::tileset_rect::TilesetRect, types::Int};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct FieldInstance {
    /// Field definition identifier
    #[serde(alias = "__identifier")]
    pub identifier: String,

    /// Optional TilesetRect used to display this field (this can be the field own Tile, or some other Tile guessed from the value, like an Enum).
    #[serde(alias = "__tile")]
    pub tile: Option<TilesetRect>,

    /// Actual value of the field instance. The value type varies, depending on __type:
    /// - For classic types (ie. Integer, Float, Boolean, String, Text and FilePath), you just get the actual value with the expected type.
    /// - For Color, the value is an hexadecimal string using "#rrggbb" format.
    /// - For Enum, the value is a String representing the selected enum value.
    /// - For Point, the value is a GridPoint object.
    /// - For Tile, the value is a TilesetRect object.
    /// - For EntityRef, the value is an EntityReferenceInfos object.
    /// If the field is an array, then this __value will also be a JSON array.
    #[serde(alias = "__value")]
    pub value: serde_json::Value,

    /// Reference of the Field definition UID
    #[serde(alias = "defUid")]
    pub def_uid: Int,
}

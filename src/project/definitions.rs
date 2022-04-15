pub mod entity_definition;
pub mod enum_definition;
pub mod layer_definition;
pub mod tileset_definition;

use serde::Deserialize;

use self::{
    entity_definition::EntityDefinition, enum_definition::EnumDefinition,
    layer_definition::LayerDefinition, tileset_definition::TilesetDefinition,
};

#[derive(Clone, Deserialize)]
pub struct Definitions {
    /// All entities definitions, including their custom fields
    pub entities: Vec<EntityDefinition>,

    /// All internal enums
    pub enums: Vec<EnumDefinition>,

    /// Note: external enums are exactly the same as enums, except they have a relPath to point to an external source file.
    #[serde(alias = "externalEnums")]
    pub external_enums: Vec<EnumDefinition>,

    /// All layer definitions
    pub layers: Vec<LayerDefinition>,

    /// All tilesets
    pub tilesets: Vec<TilesetDefinition>,
}

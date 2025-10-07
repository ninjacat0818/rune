pub extern crate bevy_remote;
pub extern crate serde_json;

pub const BRP_LOAD_IMAGE_METHOD: &str = "asset_server.load_image";
pub const BRP_SPAWN_SPRITE_METHOD: &str = "asset_server.spawn_sprite";

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct AssetLoadParams {
    pub path: String,
}

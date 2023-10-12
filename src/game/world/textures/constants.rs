pub const TEXTURE_ATLAS_PATH: &'static str = "assets/tilemaps/packed_tilemap.png";
pub const ASSET_TEXTURE_ATLAS_PATH: &'static str = "tilemaps/packed_tilemap.png";
pub const NEIGHBOURS_TO_CHECK: [(i8, i8); 4] = [
    (-1, 0), // Left
    (1, 0),  // Right
    (0, 1),  // Up
    (0, -1), // Down
];

pub const FILLED_HEIGHT: u8 = 0;
pub const NON_FILLED_HEIGHT: u8 = 1;

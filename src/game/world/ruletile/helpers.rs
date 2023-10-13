use crate::{
    game::world::{
        biomes::helpers::{BiomeData, TileTextureData},
        generation::constants::CHUNK_SIZE,
        textures::{
            constants::{FILLED_HEIGHT, NON_FILLED_HEIGHT},
            helpers::HeightMap,
        },
    },
    math::map::ValueMap2D,
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct RuletileMap {
    pub size: (usize, usize),
    points: Vec<RuleTile>,
}

impl ValueMap2D<RuleTile> for RuletileMap {
    fn new(size: (usize, usize)) -> Self {
        let (width, height) = size;
        let map_size = width * height;
        let ruletile_map = Self {
            size,
            points: vec![RuleTile::Middle; map_size],
        };

        ruletile_map
    }

    fn get_size(&self) -> (usize, usize) {
        self.size
    }

    fn get_points(&self) -> &[RuleTile] {
        self.points.as_slice()
    }

    fn mut_points(&mut self) -> &mut Vec<RuleTile> {
        &mut self.points
    }
}

impl RuletileMap {
    pub fn generate(height_map: &HeightMap) -> RuletileMap {
        let mut ruletile_map = RuletileMap::new((CHUNK_SIZE as usize, CHUNK_SIZE as usize));

        for x in 0..CHUNK_SIZE as usize {
            for y in 0..CHUNK_SIZE as usize {
                let height = height_map.get_value(x, y).unwrap();

                if height == NON_FILLED_HEIGHT {
                    ruletile_map.set_value(x, y, RuleTile::Water);
                    continue;
                }

                ruletile_map.set_value(x, y, RuleTile::determine(x, y, height_map));
            }
        }

        ruletile_map
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum RuleTile {
    TopLeft,
    TopMiddle,
    TopRight,
    Middle,
    MiddleLeft,
    MiddleRight,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
    BottomLeft,
    BottomMiddle,
    BottomRight,
    Water,
}
use RuleTile::*;

impl RuleTile {
    pub fn iter() -> std::slice::Iter<'static, RuleTile> {
        static DIRECTIONS: [RuleTile; 14] = [
            TopLeft,
            TopMiddle,
            TopRight,
            Middle,
            MiddleLeft,
            MiddleRight,
            DownRight,
            DownLeft,
            UpRight,
            UpLeft,
            BottomLeft,
            BottomMiddle,
            BottomRight,
            Water,
        ];

        DIRECTIONS.iter()
    }

    pub fn determine(x: usize, y: usize, height_map: &HeightMap) -> Self {
        for ruletile in Self::iter() {
            if !ruletile.check_excluded_neighbours(x, y, height_map) {
                continue;
            }

            if !ruletile.check_required_neighbours(x, y, height_map) {
                continue;
            }

            return *ruletile;
        }

        Self::Water
    }

    pub fn get_tile_data<'biome>(&self, biome_data: &'biome BiomeData) -> &'biome TileTextureData {
        match self {
            TopLeft => &biome_data.tiles.top_left,
            TopMiddle => &biome_data.tiles.top_middle,
            TopRight => &biome_data.tiles.top_right,
            Middle => &biome_data.tiles.middle,
            MiddleLeft => &biome_data.tiles.middle_left,
            MiddleRight => &biome_data.tiles.middle_right,
            DownRight => &biome_data.tiles.down_right,
            DownLeft => &biome_data.tiles.down_left,
            UpRight => &biome_data.tiles.up_right,
            UpLeft => &biome_data.tiles.up_left,
            BottomLeft => &biome_data.tiles.bottom_left,
            BottomMiddle => &biome_data.tiles.bottom_middle,
            BottomRight => &biome_data.tiles.bottom_right,
            Water => &biome_data.tiles.water,
        }
    }

    pub fn check_excluded_neighbours(&self, x: usize, y: usize, height_map: &HeightMap) -> bool {
        let excluded_neighbours = self.get_excluded_neighbours();

        for neighbours in excluded_neighbours {
            let nx = (x as i32 + neighbours.x) as usize;
            let ny = (y as i32 + neighbours.y) as usize;
            let height = height_map.get_value(nx, ny).unwrap_or(NON_FILLED_HEIGHT);

            if height == FILLED_HEIGHT {
                return false;
            }
        }

        true
    }

    pub fn check_required_neighbours(&self, x: usize, y: usize, height_map: &HeightMap) -> bool {
        let required_neighbours = self.get_required_neighbours();

        for neighbours in required_neighbours {
            let nx = (x as i32 + neighbours.x) as usize;
            let ny = (y as i32 + neighbours.y) as usize;
            let height = height_map.get_value(nx, ny).unwrap_or(NON_FILLED_HEIGHT);

            if height == NON_FILLED_HEIGHT {
                return false;
            }
        }

        true
    }

    fn get_required_neighbours(&self) -> Vec<IVec2> {
        match self {
            TopLeft => vec![
                IVec2::new(0, 0),
                IVec2::new(1, 0),  // Right
                IVec2::new(0, -1), // Below
                IVec2::new(1, -1), // Right below
            ],
            MiddleLeft => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(0, -1), // Below
                IVec2::new(1, 0),  // Right
                IVec2::new(1, 1),  // Right above
                IVec2::new(1, -1), // Right below
            ],
            BottomLeft => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1), // Above
                IVec2::new(1, 0), // Right
                IVec2::new(1, 1), // Right above
            ],
            TopRight => vec![
                IVec2::new(0, 0),
                IVec2::new(-1, 0),  // Left
                IVec2::new(0, -1),  // Below
                IVec2::new(-1, -1), // Left below
            ],
            MiddleRight => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),   // Above
                IVec2::new(0, -1),  // Below
                IVec2::new(-1, 0),  // Left
                IVec2::new(-1, 1),  // Left above
                IVec2::new(-1, -1), // Left below
            ],
            BottomRight => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(-1, 0), // Left
                IVec2::new(-1, 1), // Left above
            ],
            TopMiddle => vec![
                IVec2::new(0, 0),
                IVec2::new(-1, 0),  // Left
                IVec2::new(1, 0),   // Right
                IVec2::new(0, -1),  // Below
                IVec2::new(1, -1),  // Right below
                IVec2::new(-1, -1), // Left below
            ],
            Middle => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),   // Above
                IVec2::new(0, -1),  // Below
                IVec2::new(-1, 0),  // Left
                IVec2::new(1, 0),   // Right
                IVec2::new(-1, 1),  // Left above
                IVec2::new(-1, -1), // Left below
                IVec2::new(1, 1),   // Right above
                IVec2::new(1, -1),  // Right below
            ],
            BottomMiddle => vec![
                IVec2::new(0, 0),
                IVec2::new(-1, 0), // Left
                IVec2::new(1, 0),  // Right
                IVec2::new(0, 1),  // Above
                IVec2::new(-1, 1), // Above left
                IVec2::new(1, 1),  // Above right
            ],
            UpRight => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(1, 0),  // Right
                IVec2::new(-1, 0), // Left
                IVec2::new(0, -1), // Below
            ],
            UpLeft => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(1, 0),  // Right
                IVec2::new(-1, 0), // Left
                IVec2::new(0, -1), // Below
            ],
            DownRight => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(1, 0),  // Right
                IVec2::new(-1, 0), // Left
                IVec2::new(0, -1), // Below
            ],
            DownLeft => vec![
                IVec2::new(0, 0),
                IVec2::new(0, 1),  // Above
                IVec2::new(1, 0),  // Right
                IVec2::new(-1, 0), // Left
                IVec2::new(0, -1), // Below
            ],
            Water => Vec::new(),
        }
    }

    pub fn get_excluded_neighbours(&self) -> Vec<IVec2> {
        match self {
            TopLeft => vec![
                IVec2::new(-1, 0), // Left
                IVec2::new(-1, 1), // Left above
                IVec2::new(0, 1),  // Above
            ],
            MiddleLeft => vec![
                IVec2::new(-1, 0), // Left,
            ],
            BottomLeft => vec![
                IVec2::new(-1, 0),  // Left
                IVec2::new(-1, -1), // Left below
                IVec2::new(0, -1),  // Below
            ],
            TopRight => vec![
                IVec2::new(1, 0), // Right
                IVec2::new(1, 1), // Right above
                IVec2::new(0, 1), // Above
            ],
            MiddleRight => vec![
                IVec2::new(1, 0), // Right
            ],
            BottomRight => vec![
                IVec2::new(1, 0),  // Right
                IVec2::new(1, -1), // Right below
                IVec2::new(0, -1), // below
            ],
            TopMiddle => vec![
                IVec2::new(0, 1), // Above
            ],
            Middle => Vec::new(),
            BottomMiddle => vec![
                IVec2::new(0, -1), // Below
            ],
            UpRight => vec![
                IVec2::new(1, 1), // Right above
            ],
            UpLeft => vec![
                IVec2::new(-1, 1), // Left above
            ],
            DownRight => vec![
                IVec2::new(1, -1), // Right below
            ],
            DownLeft => vec![
                IVec2::new(-1, -1), // Left below
            ],
            Water => Vec::new(),
        }
    }
}

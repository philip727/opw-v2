use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;

use crate::{
    game::world::{
        biomes::resources::BiomeManager,
        helpers::{IntoChunkPos, IntoTranslation, IntoWorldPos},
        resources::WorldManager,
        textures::helpers::{HeightMap, TextureMap}, ruletile::helpers::RuletileMap,
    },
    math::{map::ValueMap2D, noise::generate_perlin_noise},
};

use super::{
    components::ComputeTextureMap,
    constants::*,
    events::{RequestChunkRender, RequestTextureMap},
    helpers::{create_chunk_tilemap, HeightNoiseMap, PrecipitationNoiseMap, TemperatureNoiseMap},
};

pub fn spawn_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_manager: ResMut<WorldManager>,
    mut request_texture_map_event_writer: EventWriter<RequestTextureMap>,
) {
    println!("Main chunk spawned");
    let start_pos = Vec2 { x: 0.0, y: 0.0 };
    let chunk_pos = start_pos.to_chunk_pos();

    let chunk_entity = create_chunk_tilemap(&mut commands, &asset_server, &chunk_pos);
    world_manager.chunk_entity = Some(chunk_entity);

    request_texture_map_event_writer.send(RequestTextureMap {
        world_position: chunk_pos.to_translation().to_world_pos(),
    });
}


pub fn generate_texture_maps(
    mut commands: Commands,
    mut request_texture_map_event_reader: EventReader<RequestTextureMap>,
    biome_manager: Res<BiomeManager>,
) {
    let seed = 1203;
    let noise_map_events: Vec<_> = request_texture_map_event_reader.iter().cloned().collect();

    for event in noise_map_events {
        let thread_pool = AsyncComputeTaskPool::get();
        let chunk_world_pos = event.world_position.clone();
        let biome_manager = biome_manager.clone();
        info!("Generating new texture map");

        // Moves the noise generation on to a seperate thread
        let task = thread_pool.spawn(async move {
            let height_noise_map = HeightNoiseMap(generate_perlin_noise(
                chunk_world_pos.x as i32,
                chunk_world_pos.y as i32,
                seed,
                HEIGHT_OFFSET,
                HEIGHT_OCTAVES,
                HEIGHT_LACUNARITY,
                HEIGHT_PERSISTENCE,
                HEIGHT_FREQUENCY / HEIGHT_SCALE,
            ));

            let temperature_noise_map = TemperatureNoiseMap(generate_perlin_noise(
                chunk_world_pos.x as i32,
                chunk_world_pos.y as i32,
                seed,
                TEMPERATURE_OFFSET,
                TEMPERATURE_OCTAVES,
                TEMPERATURE_LACUNARITY,
                TEMPERATURE_PERSISTENCE,
                TEMPERATURE_FREQUENCY / TEMPERATURE_SCALE,
            ));

            let precipitation_noise_map = PrecipitationNoiseMap(generate_perlin_noise(
                chunk_world_pos.x as i32,
                chunk_world_pos.y as i32,
                seed,
                PRECIPITATION_OFFSET,
                PRECIPITATION_OCTAVES,
                PRECIPITATION_LACUNARITY,
                PRECIPITATION_PERSISTENCE,
                PRECIPITATION_FREQUENCY / PRECIPITATION_SCALE,
            ));

            let mut height_map = HeightMap::generate(&height_noise_map, &precipitation_noise_map);
            height_map.smoothen_height_map(Some(3));

            let ruletile_map = RuletileMap::generate(&height_map);
            let texture_map = TextureMap::generate(
                &height_map,
                &temperature_noise_map,
                &precipitation_noise_map,
                &ruletile_map,
                &biome_manager,
            );


            info!("Finished texture map");
            (texture_map, chunk_world_pos)
        });

        commands
            .spawn(ComputeTextureMap(task))
            .insert(Name::new("Compute Texture Map Task"));
    }
}

pub fn handle_texture_map_generation_task(
    mut commands: Commands,
    mut texture_map_tasks: Query<(Entity, &mut ComputeTextureMap)>,
    mut request_chunk_rerender_event_writer: EventWriter<RequestChunkRender>,
) {
    for (task_entity, mut task) in &mut texture_map_tasks {
        if let Some((texture_map, world_pos)) = future::block_on(future::poll_once(&mut task.0)) {
            request_chunk_rerender_event_writer.send(RequestChunkRender {
                texture_map,
                world_position: world_pos,
            });

            commands.entity(task_entity).despawn();
        }
    }
}

use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use futures_lite::future;

use crate::{
    game::world::{
        biomes::resources::BiomeManager,
        helpers::{
            adjust_translation_for_chunk, IntoChunkPos, IntoThresholdPos, IntoTranslation,
            IntoWorldPos, SetZToChunkZ, ThresholdPos,
        },
        resources::WorldManager,
        ruletile::helpers::RuletileMap,
        textures::{
            helpers::{HeightMap, TextureMap},
            resources::WorldTextureManager,
        },
    },
    math::{map::ValueMap2D, noise::generate_perlin_noise},
};

use super::{
    components::{ChunkTarget, ComputeTextureMap},
    constants::*,
    events::{RequestChunkRender, RequestTextureMap},
    helpers::{create_chunk_tilemap, HeightNoiseMap, PrecipitationNoiseMap, TemperatureNoiseMap},
    resources::WorldGenerationManager,
};

pub fn spawn_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_manager: ResMut<WorldManager>,
    mut request_texture_map_event_writer: EventWriter<RequestTextureMap>,
) {
    println!("Main chunk spawned");
    let chunk_pos = ThresholdPos { x: 0, y: 0 };

    let chunk_entity = create_chunk_tilemap(&mut commands, &asset_server, &chunk_pos);
    world_manager.chunk_entity = Some(chunk_entity);

    request_texture_map_event_writer.send(RequestTextureMap {
        threshold_pos: chunk_pos,
    });
}

pub fn update_chunk_from_target(
    mut world_gen_manager: ResMut<WorldGenerationManager>,
    mut request_texture_map_event_writer: EventWriter<RequestTextureMap>,
    target_query: Query<&Transform, With<ChunkTarget>>,
) {
    let target = target_query.single();
    let target_threshold_pos = target.translation.to_threshold_pos();
    if world_gen_manager.last_update_pos == target_threshold_pos {
        return;
    }
    info!("Request chunk update");

    // Moves the chunk to the right position
    world_gen_manager.last_update_pos = target_threshold_pos;
    request_texture_map_event_writer.send(RequestTextureMap {
        threshold_pos: target_threshold_pos,
    });
}

pub fn generate_texture_for_chunk(
    mut commands: Commands,
    mut request_texture_map_event_reader: EventReader<RequestTextureMap>,
    world_texture_manager: Res<WorldTextureManager>,
    biome_manager: Res<BiomeManager>,
) {
    let seed = 1203;
    let noise_map_events: Vec<_> = request_texture_map_event_reader.iter().cloned().collect();

    for event in noise_map_events {
        info!("Generating new texture map");

        let thread_pool = AsyncComputeTaskPool::get();
        let threshold_pos = event.threshold_pos.clone();
        let world_pos = event.threshold_pos.clone().to_translation().to_world_pos();
        let cached_maps = world_texture_manager.cached_texture_maps.clone();
        let biome_manager = biome_manager.clone();

        // Moves the noise generation on to a seperate thread
        let task = thread_pool.spawn(async move {
            if let Some(texture_map) = cached_maps.get(&threshold_pos) {
                return (texture_map.clone(), threshold_pos);
            }

            let height_noise_map = HeightNoiseMap(generate_perlin_noise(
                world_pos.x as i32,
                world_pos.y as i32,
                seed,
                HEIGHT_OFFSET,
                HEIGHT_OCTAVES,
                HEIGHT_LACUNARITY,
                HEIGHT_PERSISTENCE,
                HEIGHT_FREQUENCY / HEIGHT_SCALE,
            ));

            let temperature_noise_map = TemperatureNoiseMap(generate_perlin_noise(
                world_pos.x as i32,
                world_pos.y as i32,
                seed,
                TEMPERATURE_OFFSET,
                TEMPERATURE_OCTAVES,
                TEMPERATURE_LACUNARITY,
                TEMPERATURE_PERSISTENCE,
                TEMPERATURE_FREQUENCY / TEMPERATURE_SCALE,
            ));

            let precipitation_noise_map = PrecipitationNoiseMap(generate_perlin_noise(
                world_pos.x as i32,
                world_pos.y as i32,
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
            (texture_map, threshold_pos)
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
    mut world_texture_manager: ResMut<WorldTextureManager>,
) {
    for (task_entity, mut task) in &mut texture_map_tasks {
        if let Some((texture_map, threshold_pos)) = future::block_on(future::poll_once(&mut task.0))
        {
            world_texture_manager
                .cached_texture_maps
                .insert(threshold_pos, texture_map.clone());

            request_chunk_rerender_event_writer.send(RequestChunkRender {
                texture_map,
                world_position: threshold_pos.to_translation().set_z_to_chunk_z(),
            });

            commands.entity(task_entity).despawn();
        }
    }
}

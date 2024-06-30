use crate::{
    components::Player,
    constants::ENTITY_RENDER_DISTANCE,
    events::{CreateWorldEntitiesInRangeEvent, MoveEntitiesEvent, RenderEntitiesEvent},
    resources::{EntityPoolRes, RequiredAssets},
};
use bevy::{
    asset::Handle,
    hierarchy::DespawnRecursiveExt,
    prelude::{Camera3d, Commands, EventReader, EventWriter, Query, Res, ResMut, With},
    scene::{Scene, SceneBundle},
    transform::components::Transform,
};
use kiddo::SquaredEuclidean;

pub fn find_world_entities_in_range_system(
    required_assets: Res<RequiredAssets>,
    mut create_world_entities_in_range_event: EventReader<CreateWorldEntitiesInRangeEvent>,
    query_player: Query<&Transform, (With<Camera3d>, With<Player>)>,
    mut render_entities_event: EventWriter<RenderEntitiesEvent>,
) {
    for _ in create_world_entities_in_range_event.read() {
        // Only spawns entities with spawn points near player.
        // TODO Figure out how to remove spawn point after entity destroyed

        let player_location = query_player.get_single().unwrap();
        let nearest_neighbors: Vec<usize> = required_assets
            .model_tree
            .within_unsorted_iter::<SquaredEuclidean>(
                &[
                    player_location.translation.x,
                    player_location.translation.y,
                    player_location.translation.z,
                ],
                // NOTE Consider ENTITY_RENDER_DISTANCE * ENTITY_RENDER_DISTANCE
                ENTITY_RENDER_DISTANCE.powi(2),
            )
            .map(|nearest_neighbor| {
                return nearest_neighbor.item;
            })
            .collect();
        // Render Entities Event
        // println!(
        //     "Searching Player Location: {:?}\tNearest Neighbors: {:?}",
        //     player_location.translation, nearest_neighbors
        // );
        render_entities_event.send(RenderEntitiesEvent(nearest_neighbors));
    }
}

pub fn render_entities_system(
    mut render_entities_event: EventReader<RenderEntitiesEvent>,
    mut commands: Commands,
    mut entity_pool_res: ResMut<EntityPoolRes>,
    required_assets_res: Res<RequiredAssets>,
) {
    for close_entity_index_list in render_entities_event.read() {
        let temp_entity_pool = entity_pool_res.0.clone();
        for (entity_index, physical_entity) in temp_entity_pool.iter().enumerate() {
            // Needs to be Spawned

            let mut entity_id = None;
            if close_entity_index_list.0.contains(&entity_index) {
                // Isn't spawned
                if physical_entity.is_none() {
                    // Spawn entity
                    // println!(
                    //     "Rendering Entity: {:?}\nAt location: {:?}",
                    //     required_assets_res.models[entity_index].clone(),
                    //     required_assets_res.model_locations[entity_index]
                    // );

                    let entity = commands.spawn(SceneBundle {
                        scene: required_assets_res.models[entity_index].clone(),
                        transform: Transform::from_xyz(
                            required_assets_res.model_locations[entity_index][0],
                            required_assets_res.model_locations[entity_index][1],
                            required_assets_res.model_locations[entity_index][2],
                        ),
                        ..Default::default()
                    });

                    entity_id = Some(entity.id());
                }
                // Is spawned and should be
                else {
                    entity_id = *physical_entity;
                }
            }
            // println!("Possible Despawn: {:?}", physical_entity);

            // Needs to be despawned
            else if let Some(entity) = physical_entity {
                // println!("Possible Despawn: {:?}", entity);

                // Despawn entity
                commands.entity(*entity).despawn_recursive();
            }
            entity_pool_res.0[entity_index] = entity_id;
        }
        // Trigger move entities event
    }
}

pub fn move_entities_event(
    mut move_entities_event: EventReader<MoveEntitiesEvent>,
    _query: Query<&Transform, With<Handle<Scene>>>,
) {
    for _entities_to_move in move_entities_event.read() {
        // Move based on type
    }
}

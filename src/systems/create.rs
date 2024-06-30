use crate::{
    components::{AppStateComponent, Player},
    events::{CreateLoadingPageEvent, RenderProgressBarEvent},
    plugins::game_play::PanOrbitCameraBundle,
};
use bevy::{
    math::Vec3,
    prelude::{Camera2dBundle, Camera3d, Commands, Entity, EventReader, EventWriter, Query},
};

pub fn create_loading_page(
    mut create_loading_page_event: EventReader<CreateLoadingPageEvent>,
    mut render_progress_bar_event: EventWriter<RenderProgressBarEvent>,
    // mut commands: Commands,
) {
    for _ in create_loading_page_event.read() {
        // TODO
        // commands.spawn(bundle);
        // Create Page
        // Render Text

        render_progress_bar_event.send(RenderProgressBarEvent);
    }
}

pub fn create_2d_camera_event<T>(mut commands: Commands)
where
    T: AppStateComponent,
{
    println!("Creating 2D Camera");
    commands
        .spawn(Camera2dBundle {
            ..Default::default()
        })
        .insert(T::default());
}

pub fn create_3d_camera_event<T>(mut commands: Commands)
where
    T: AppStateComponent,
{
    println!("Creating 3D Camera");
    let mut camera = PanOrbitCameraBundle::default();
    // Position our camera using our component,
    // not Transform (it would get overwritten)
    camera.state.center = Vec3::new(0.0, 0.0, 0.0);
    // camera.state.radius = 50.0;
    // camera.state.pitch = 15.0f32.to_radians();
    // camera.state.yaw = 30.0f32.to_radians();
    commands.spawn(camera).insert(T::default());
}

pub fn create_player_system() {
    // TODO spawn systems for player entity
}

pub fn create_map_system() {
    // TODO RTIN Render goes here
}

pub fn create_player_entity_system(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Camera3d)>,
) {
    if let Ok((entity, _camera)) = camera_query.get_single_mut() {
        commands.entity(entity).insert(Player);
    }
}

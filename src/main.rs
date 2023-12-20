extern crate bevy;

use bevy::audio::{AudioPlugin, SpatialScale};
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::{close_on_esc, PrimaryWindow};

mod ball;
mod pid;
mod sounds;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rust PID Sim: with Bevy!".into(),
                        resolution: (1600., 900.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    // level: Level::DEBUG,
                    level: Level::WARN,
                    ..default()
                }),
                // .set(AudioPlugin {
                //     // TODO: move this around?
                //     spatial_scale: SpatialScale::new_2d(1./100.),
                //     ..default()
                // }),
            ball::BallPlugin,
            sounds::SpatialSoundBallPlugin,
        ))
        .init_resource::<MouseWorldCoords>()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_mouse_world_coords, close_on_esc))
        .run();
}
#[derive(Component)]
struct MainCamera;

#[derive(Resource, Default)]
struct MouseWorldCoords(Vec2);

fn setup(mut commands: Commands) {
    // Make sure to add the marker component when you set up your camera
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
// https://bevy-cheatbook.github.io/cookbook/cursor2world.html bruh
fn update_mouse_world_coords(
    mut mouse_world_coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_world_coords.0 = world_position;
    }
}

// Refer https://bevyengine.org/examples/Audio/spatial-audio-2d/
// and https://github.com/bevyengine/bevy/blob/main/examples/audio/decodable.rs

use std::time::Duration;

use crate::ball::{Ball, Velocity};
use bevy::prelude::*;

pub struct SpatialSoundBallPlugin;

impl Plugin for SpatialSoundBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_sound_ball)
            .add_systems(Update, play_sounds);
    }
}

/// Spatial audio uses the distance to attenuate the sound volume. In 2D with the default camera,
/// 1 pixel is 1 unit of distance, so we use a scale so that 100 pixels is 1 unit of distance for
/// audio.
// const AUDIO_SCALE: f32 = 1. / 100.0;

fn setup(mut commands: Commands, windows: Query<&Window>) {
    // TODO: Spatial audio!
    // let gap = 0.5 * windows.single().width(); // Space between the two ears
    // commands.spawn((SpatialBundle::default(), SpatialListener::new(gap)));
}

fn update_sound_ball(mut query: Query<(&Velocity, &mut PitchFrequency), With<Ball>>) {
    for (vel, mut pf) in &mut query {
        debug!("Vel {}", vel.0.length());
        pf.0 = calculate_pitch(vel.0.length());
    }

    fn calculate_pitch(vel: f32) -> f32 {
        // https://en.wikipedia.org/wiki/Piano_key_frequencies
        // Make it so that only valid (standard tuning) 88-keyboard notes are played
        // proportionate to the velocity (TODO: eventually not linearly lol)
        let min_vel = 100.;
        let max_vel = 10000.;
        let min_n = 20.; // Out of 88 keys
        let max_n = 64.; // 64 is C6

        // let recurved_v = 4. * (vel - 0.5).powi(3) + 0.5;
        let mapped = (vel - min_vel) * (max_n - min_n) / (max_vel - min_vel) + min_n;

        let key_num = mapped.floor();

        let pitch = 2.0_f32.powf((key_num - 49.) / 12.) * 440.;

        pitch.clamp(300., 2000.)
    }
}

fn play_sounds(
    mut commands: Commands,
    pitches: Query<&PitchFrequency>,
    mut pitch_assets: ResMut<Assets<Pitch>>,
) {
    for frequency in &pitches {
        commands.spawn(PitchBundle {
            source: pitch_assets.add(Pitch::new(frequency.0, Duration::from_millis(420))),
            settings: PlaybackSettings::ONCE, //.with_spatial(true),
        });
    }
}

#[derive(Component)]
pub struct PitchFrequency(pub f32);

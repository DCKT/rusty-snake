use std::time::Duration;

use bevy::prelude::*;

#[derive(Event, Default)]
pub struct FoodEatenPitchEvent;

#[derive(Resource)]
pub struct PitchFrequency(pub f32);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(PitchFrequency(120.));
}

pub fn play_food_eaten_pitch(
    mut pitch_assets: ResMut<Assets<Pitch>>,
    frequency: Res<PitchFrequency>,
    mut events: EventReader<FoodEatenPitchEvent>,
    mut commands: Commands,
) {
    for _ in events.read() {
        info!("playing pitch with frequency: {}", frequency.0);
        commands.spawn(PitchBundle {
            source: pitch_assets.add(Pitch::new(frequency.0, Duration::from_millis(150))),
            settings: PlaybackSettings::DESPAWN,
        });
        info!("number of pitch assets: {}", pitch_assets.len());
    }
}

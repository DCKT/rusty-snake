use std::time::Duration;

use bevy::prelude::*;

use super::food::Food;

#[derive(Event)]
pub struct FoodEatenPitchEvent(pub Food);

pub fn play_food_eaten_pitch(
    mut pitch_assets: ResMut<Assets<Pitch>>,
    mut events: EventReader<FoodEatenPitchEvent>,
    mut commands: Commands,
) {
    for e in events.read() {
        let pitch = match e.0 {
            Food::Grow => Pitch::new(120., Duration::from_millis(150)),
            Food::Shrink => Pitch::new(500., Duration::from_millis(150)),
        };
        commands.spawn(PitchBundle {
            source: pitch_assets.add(pitch),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

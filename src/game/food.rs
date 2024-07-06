use bevy::prelude::*;
use rand::prelude::random;

use crate::utils::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};

pub const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

fn generate_random_position() -> Position {
    Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    }
}

fn get_available_position(positions: Query<&Position>) -> Position {
    let new_pos = generate_random_position();

    let new_pos_is_available = positions.iter().find(|&&pos| pos == new_pos).is_none();

    if new_pos_is_available {
        new_pos
    } else {
        get_available_position(positions)
    }
}

pub fn food_spawner(
    mut commands: Commands,
    positions: Query<&Position>,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let position = get_available_position(positions);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(position)
            .insert(Size::square(0.8));
    }
}

use bevy::prelude::*;
use rand::prelude::random;

use crate::utils::{Position, Size, ARENA_HEIGHT, ARENA_WIDTH};

pub const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Food;

#[derive(Resource)]
pub struct FoodSpawnTimer(pub Timer);

pub fn food_spawner(mut commands: Commands, time: Res<Time>, mut timer: ResMut<FoodSpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: FOOD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(Food)
            .insert(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .insert(Size::square(0.8));
    }
}

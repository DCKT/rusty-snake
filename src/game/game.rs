use bevy::prelude::*;

use crate::{
    game::{food::*, snake::*},
    utils::{despawn_screen, GameState, Position, Size, ARENA_HEIGHT, ARENA_WIDTH},
};

// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
// #[source(GameState = GameState::Game)]
// enum InGameState {
//     #[default]
//     Running,
//     Paused,
// }

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Component)]
struct OnGameScreen;

pub fn game_plugin(app: &mut App) {
    app
        // .add_sub_state::<InGameState>()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .insert_resource(FoodSpawnTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .insert_resource(SnakeDirectionTimer(Timer::from_seconds(
            0.20,
            TimerMode::Repeating,
        )))
        .insert_resource(LastTailPosition::default())
        .insert_resource(SnakeSegments::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(OnEnter(GameState::Game), spawn_snake)
        .add_systems(
            Update,
            (
                snake_movement_input,
                snake_eating,
                snake_growth,
                snake_movement,
                game_over,
                food_spawner,
            )
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

fn game_over(
    mut reader: EventReader<GameOverEvent>,
    // segments_res: ResMut<SnakeSegments>,
    // food: Query<Entity, With<Food>>,
    // segments: Query<Entity, With<SnakeSegment>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if reader.read().next().is_some() {
        game_state.set(GameState::Menu)
    }
}

/*
The sizing logic goes like so: if something has a width of 1 in a grid of 40,
and the window is 400px across, then it should have a width of 10.
 */
fn size_scaling(windows: Query<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.single();

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.,
        )
    }
}
fn position_translation(windows: Query<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.single();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

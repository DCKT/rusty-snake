use bevy::{dev_tools::states::*, prelude::*};

use crate::{
    game::{food::*, snake::*},
    utils::{despawn_screen, GameState, Position, Size, ARENA_HEIGHT, ARENA_WIDTH, TEXT_COLOR},
};

use super::sound::{self, FoodEatenPitchEvent};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Game)]
enum InGameState {
    #[default]
    Running,
    Paused,
}

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Component)]
pub struct OnGameScreen;

pub fn game_plugin(app: &mut App) {
    app.add_sub_state::<InGameState>()
        .enable_state_scoped_entities::<InGameState>()
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<FoodEatenPitchEvent>()
        .add_systems(
            OnEnter(GameState::Game),
            (init_game_resources, spawn_snake).chain(),
        )
        .add_systems(Update, pause_menu.run_if(in_state(InGameState::Paused)))
        .add_systems(OnExit(InGameState::Paused), despawn_screen::<OnPauseScreen>)
        .add_systems(
            Update,
            (
                (
                    snake_movement_input,
                    snake_eating,
                    snake_growth,
                    snake_movement,
                    sound::play_food_eaten_pitch,
                    game_over,
                    food_spawner,
                )
                    .chain()
                    .run_if(in_state(InGameState::Running)),
                (toggle_pause.run_if(in_state(GameState::Game))),
            ),
        )
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_systems(Update, log_transitions::<GameState>)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

fn init_game_resources(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)));
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(SnakeDirectionTimer(Timer::from_seconds(
        0.20,
        TimerMode::Repeating,
    )));
    commands.insert_resource(LastTailPosition::default());
    commands.insert_resource(SnakeSegments::default());
    sound::setup(commands);
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(match current_state.get() {
            InGameState::Running => InGameState::Paused,
            InGameState::Paused => InGameState::Running,
        });
    }
}

#[derive(Component)]
struct OnPauseScreen;

fn pause_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnPauseScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: bevy::color::palettes::css::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Pause",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });
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

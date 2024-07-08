use bevy::{prelude::*, sprite::SpriteBundle, time::Timer};

use crate::{
    game::food::Food,
    game::game::GameOverEvent,
    utils::{Direction, Position, Size, ARENA_HEIGHT, ARENA_WIDTH},
};

use super::{
    game::{OnGameScreen, ScoreText},
    sound::FoodEatenPitchEvent,
};

#[derive(Component)]
pub struct SnakeSegment;
#[derive(Default, Resource)]
pub struct SnakeSegments(pub Vec<Entity>);
#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

#[derive(Resource)]
pub struct SnakeDirectionTimer(pub Timer);

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Event)]
pub struct GrowthEvent;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: SNAKE_HEAD_COLOR,
                        ..default()
                    },
                    ..default()
                },
                OnGameScreen,
            ))
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

pub fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    time: Res<Time>,
    mut timer: ResMut<SnakeDirectionTimer>,
    mut last_tail_position: ResMut<LastTailPosition>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some((head_entity, head)) = heads.iter_mut().next() {
            let segment_positions = segments
                .0
                .iter()
                .map(|e| *positions.get_mut(*e).unwrap())
                .collect::<Vec<Position>>();
            *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
            let mut head_pos = positions.get_mut(head_entity).unwrap();

            match &head.direction {
                Direction::Left => {
                    head_pos.x -= 1;
                }
                Direction::Right => {
                    head_pos.x += 1;
                }
                Direction::Up => {
                    head_pos.y += 1;
                }
                Direction::Down => {
                    head_pos.y -= 1;
                }
            }

            if head_pos.x < 0
                || head_pos.y < 0
                || head_pos.x as u32 >= ARENA_WIDTH
                || head_pos.y as u32 >= ARENA_HEIGHT
            {
                game_over_writer.send(GameOverEvent);
            }

            if segment_positions.contains(&head_pos) {
                game_over_writer.send(GameOverEvent);
            }

            segment_positions
                .iter()
                .zip(segments.0.iter().skip(1))
                .for_each(|(pos, segment)| {
                    *positions.get_mut(*segment).unwrap() = *pos;
                });
        }
    }
}
fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_SEGMENT_COLOR,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: ResMut<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    mut pitch_writer: EventWriter<FoodEatenPitchEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    mut texts: Query<&mut Text, With<ScoreText>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
                pitch_writer.send(FoodEatenPitchEvent);

                for mut text in &mut texts {
                    let current_store = text.sections[1]
                        .value
                        .parse::<u32>()
                        .expect("Can't get score");
                    text.sections[1].value = (current_store + 100).to_string();
                }
            }
        }
    }
}

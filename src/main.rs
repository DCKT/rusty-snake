// use bevy::core::FixedTimestep;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use rusty_snake::{
    game::game::game_plugin,
    menu::menu_plugin,
    splash::splash_plugin,
    utils::{GameState, Volume},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".into(),
                name: Some("bevy.app".into()),
                resolution: (500., 500.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Volume(7))
        .add_systems(Startup, setup)
        .add_plugins((splash_plugin, game_plugin, menu_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

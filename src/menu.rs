use bevy::prelude::*;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct QuitButton;

pub fn menu_setup(
    mut commands: Commands,
) {
    commands.spawn(
        ( 
        Button,
        PlayButton,
        Node {
        width: px(200.0),
        height: px(60.0),
        left: px(500.0),
        top: px(150.0),
        ..default()},
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    ));
    commands.spawn(
        ( 
        Button,
        SettingsButton,
        Node {
        width: px(200.0),
        height: px(60.0),
        left: px(500.0),
        top: px(300.0),
        ..default()},
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    ));
    commands.spawn(
        ( 
        Button,
        QuitButton,
        Node {
        width: px(200.0),
        height: px(60.0),
        left: px(500.0),
        top: px(450.0),
        ..default()},
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    ));
}
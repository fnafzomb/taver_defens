use crate::*;

// Spawn bullets
pub fn bullet(commands: &mut Commands, player_transform: &Transform, damage: f32) {
    commands.spawn((
        Bullet,
        PlayerProjectile,
        Damage { damage },
        Speed { speed: 200.0 },
        Hitbox { y: 1000.0, x: 10.0 },
        Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), Vec2::new(10.0, 5.0)),
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y,
            2.9,
        ),
    ));
}

pub fn player_entity(mut commands: Commands) {
    commands.spawn((
        Player,
        Hitbox { x: 40.0, y: 160.0 },
        Damage { damage: 10.0 },
        Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(40.0, 160.0)),
        Transform::from_xyz(-600.0, -40.0, 1.0),
    ));
}

pub fn attack_player(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    player_query: Query<(&Transform, &Damage), With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (player_transform, damage) = player_query.single().unwrap();

        bullet(&mut commands, player_transform, damage.damage);
    }
}



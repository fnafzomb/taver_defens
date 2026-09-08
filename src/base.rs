use crate::*;

// base
pub fn command_center(mut commands: Commands) {
    commands.spawn((
        Base,
        Hp {
            max_hp: 500.0,
            hp: 500.0,
        },
        Hitbox{
            x: 20.0,
            y: 80.0
        },
        Sprite::from_color(Color::srgb(0.6, 0.9, 0.8), Vec2::new(20.0, 80.0)),
        Transform::from_xyz(-500.0, -20.0, 1.0),
    ));
}

pub fn damage_base(
    mut base: Query<(Entity, &mut Hp, &Hitbox, &Transform), With<Base>>,
    mut commands: Commands,
){
    
}
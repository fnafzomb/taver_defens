use crate::*;

pub fn wall_entity(mut commands: Commands) {
    commands.spawn((
        Wall,
        Hitbox { x: 30.0, y: 180.0 },
        Hp {
            max_hp: 200.0,
            hp: 200.0,
        },
        Sprite::from_color(Color::srgb(0.14, 0.09, 0.01), Vec2::new(30.0, 180.0)),
        Transform::from_xyz(-400.0, -40.0, 1.1),
    ));
}

pub fn collision_bullet_wall(
    mut commands: Commands,
    mut wall: Query<(Entity, &mut Hp, &Hitbox, &Transform), With<Wall>>,
    geschoss: Query<(Entity, &Hitbox, &Transform, &Damage), (With<Bullet>, With<ZombieProjectile>)>,
) {
    for (geschoss_entity, geschoss_hitbox, geschoss_transform, damage) in &geschoss {
        let target = wall.iter_mut().find(|(_, _, wall_hitbox, wall_transform)| {
            check_hitbox(
                geschoss_hitbox,
                geschoss_transform,
                wall_hitbox,
                wall_transform,
            )
        });
        if let Some((_, mut hp, _, _)) = target {
            hp.hp -= damage.damage;
            info!("HP стены: {}", hp.hp);
            commands.entity(geschoss_entity).despawn();
        }
    }
}

pub fn death_wall(mut commands: Commands, query: Query<(Entity, &Hp), With<Wall>>) {
    for (entity, hp) in query.iter() {
        if hp.hp <= 0.0 {
            commands.entity(entity).despawn()
        }
    }
}

pub fn wall_respawn(
    mut commands: Commands,
    mut timer: Local<Timer>,
    time: Res<Time>,
    death_wall: Query<Entity, With<Wall>>,
    mut attackers: Query<Entity, With<IsAttacking>>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(4.0, TimerMode::Once)
    }

    if death_wall.is_empty() {
        for entity in attackers.iter_mut() {
            commands.entity(entity).remove::<IsAttacking>();
        }
        timer.tick(time.delta());
        if timer.is_finished() {
            wall_entity(commands);
            timer.reset();
        }
    }
}

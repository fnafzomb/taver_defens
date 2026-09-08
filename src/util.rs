use crate::*;

// Camera
pub fn camera_game(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

// Fon
pub fn fon_game(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.4, 0.1), Vec2::new(2000.0, 1000.0)),
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));
}

// Logic move
pub fn move_entity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), Without<IsAttacking>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        transform.translation.x += speed.speed * time.delta_secs();
    }
}

pub fn check_hitbox(
    hitbox_a: &Hitbox,
    transform_a: &Transform,
    hitbox_b: &Hitbox,
    transform_b: &Transform,
) -> bool {
    let x = (transform_a.translation.x - transform_b.translation.x).abs();
    let y = (transform_a.translation.y - transform_b.translation.y).abs();

    x < (hitbox_a.x + hitbox_b.x) / 2.0 && y < (hitbox_a.y + hitbox_b.y) / 2.0
}

// Delete entity off the map
pub fn delete_entity(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    query: Query<(
        Entity,
        &Transform,
        Option<&Bullet>,
        Option<&Monster>,
        Option<&ZombiType>,
    )>,
) {
    for (entity, transfor, bullet, monster, zombie_type) in query.iter() {
        if bullet.is_some() && transfor.translation.x > 2000.0 {
            commands.entity(entity).despawn();
            info!("Сущность пуля удалена");
        }
        if bullet.is_some() && transfor.translation.x < -2000.0 {
            commands.entity(entity).despawn();
            info!("Сущность пуля зомби удалена");
        }

        if monster.is_some() && transfor.translation.x < -2000.0 {
            commands.entity(entity).despawn();
            info!("Сущность зомби удалена");
            match zombie_type {
                Some(ZombiType::Normal) => zombie_count.normal -= 1,
                Some(ZombiType::Toxick) => zombie_count.toxick -= 1,
                Some(ZombiType::Fat) => zombie_count.fat -= 1,
                None => {}
            }
        }
    }
}

// Level up logic
pub fn level_up(
    mut level: ResMut<ThreatLevel>,
    mut money: ResMut<Money>,
    mut spawn_limit: ResMut<SpawnLimit>,
) {
    if level.killed >= level.need_killed && money.money >= level.upgrade_cost {
        level.level += 1;
        level.killed = 0;
        level.need_killed = (level.need_killed as f32 * 1.5) as u32;
        money.money -= level.upgrade_cost;
        level.upgrade_cost *= 1.8;
        spawn_limit.normal_max = (spawn_limit.normal_max as f32 * 1.5).round() as u32;

        spawn_limit.toxic_max = (spawn_limit.toxic_max as f32 * 1.5).round() as u32;

        spawn_limit.fat_max = (spawn_limit.fat_max as f32 * 1.5).round() as u32;
        println!("Уровень угрозы повышен до {}", level.level);
    }
}

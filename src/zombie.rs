use bevy::transform;

use crate::*;

pub fn zombie_statistic(count: Res<ZombieCount>, time: Res<Time>, mut timer: Local<Timer>) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(5.0, TimerMode::Repeating);
    }
    if timer.tick(time.delta()).just_finished() {
        info!(
            "Зомби на карте: обычных {}, токсичных {}, толстых {} | всего {}",
            count.normal,
            count.toxick,
            count.fat,
            count.normal + count.toxick + count.fat
        );
    }
}

//созднаие снаряда зомби
pub fn geschoss_zombie(commands: &mut Commands, zombi_type: &Transform, damage: f32) {
    commands.spawn((
        Geschoss,
        ZombieProjectile,
        Damage { damage },
        Speed { speed: -200.0 },
        Hitbox { y: 1000.0, x: 10.0 },
        Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), Vec2::new(10.0, 5.0)),
        Transform::from_xyz(zombi_type.translation.x, zombi_type.translation.y, 2.9),
    ));
}



// атака зомби
pub fn attack_zombie(
    mut commands: Commands,
    monster_query: Query<(Entity, &Transform, &ZombiType, &Damage), With<Monster>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (entity, monster_transform, zombi_type, damage) in &monster_query {
        let attack_range = match zombi_type {
            ZombiType::Normal => 2.0,
            ZombiType::Toxick => 50.0,
            ZombiType::Fat => 2.0,
        };

        let distance =
            (monster_transform.translation.x - player_transform.translation.x).abs();


        if distance <= attack_range {
        geschoss_zombie(&mut commands, monster_transform, damage.damage);
        commands.entity(entity).insert(IsAttacking);
        }
    }
}

//Проэктирование зомби
pub fn setup_monster_zombie(
    zombi_type: ZombiType,
    commands: &mut Commands,
    zombie_count: &mut ZombieCount,
) {
    let (monster, hitbox, speed, damage, hp, reward, color, size, zombie) = match zombi_type {
        ZombiType::Normal => (
            Monster,
            Hitbox { x: 40.0, y: 80.0 },
            Speed { speed: -10.0 },
            Damage { damage: 8.0 },
            Hp {
                max_hp: 50.0,
                hp: 50.0,
            },
            Reward { money: 10.0 },
            Color::srgb(0.2, 0.8, 0.2),
            Vec2::new(40.0, 80.0),
            zombie_count.normal += 1,
        ),
        ZombiType::Toxick => (
            Monster,
            Hitbox { x: 40.0, y: 75.0 },
            Speed { speed: -15.0 },
            Damage { damage: 11.0 },
            Hp {
                max_hp: 40.0,
                hp: 40.0,
            },
            Reward { money: 12.0 },
            Color::srgb(0.7, 0.2, 0.8),
            Vec2::new(40.0, 75.0),
            zombie_count.toxick += 1,
        ),
        ZombiType::Fat => (
            Monster,
            Hitbox { x: 60.0, y: 78.0 },
            Speed { speed: -3.0 },
            Damage { damage: 50.0 },
            Hp {
                max_hp: 200.0,
                hp: 200.0,
            },
            Reward { money: 50.0 },
            Color::srgb(0.6, 0.4, 0.2),
            Vec2::new(60.0, 78.0),
            zombie_count.fat += 1,
        ),
    };
    commands.spawn((
        zombi_type,
        monster,
        hitbox,
        speed,
        damage,
        hp,
        reward,
        Sprite::from_color(color, size),
        Transform::from_xyz(800.0, rng().random_range(-100.0..100.0), 1.0),
        zombie,
    ));
}

// Спавн зомби
pub fn spawn_all_zombie(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    spawn_limit: Res<SpawnLimit>,
    level: Res<ThreateLevel>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(3.0 / level.spawn_per_second, TimerMode::Repeating);
    }
    if timer.tick(time.delta()).just_finished() {
        if zombie_count.normal < spawn_limit.normal_max {
            setup_monster_zombie(ZombiType::Normal, &mut commands, &mut zombie_count);
        }
        if zombie_count.toxick < spawn_limit.toxick_max {
            setup_monster_zombie(ZombiType::Toxick, &mut commands, &mut zombie_count);
        }
        if zombie_count.fat < spawn_limit.fat_max {
            setup_monster_zombie(ZombiType::Fat, &mut commands, &mut zombie_count);
        }
    }
}

// Логика попадние
pub fn collision_geschoss_zombie(
    mut commands: Commands,
    mut zombies: Query<(Entity, &mut Hp, &Hitbox, &Transform), With<Monster>>,
    geschoss: Query<(Entity, &Hitbox, &Transform, &Damage), (With<Geschoss>, With<PlayerProjectile>)>,
) {
    for (geschoss_entity, geschoss_hitbox, geschoss_transform, damage) in &geschoss {
        let target = zombies
            .iter_mut()
            .find(|(_, _, zombie_hitbox, zombie_transform)| {
                check_hitbox(
                    geschoss_hitbox,
                    geschoss_transform,
                    zombie_hitbox,
                    zombie_transform,
                )
            });

        if let Some((_, mut hp, _, _)) = target {
            hp.hp -= damage.damage;
            info!("HP зомби: {}", hp.hp);
            commands.entity(geschoss_entity).despawn();
        }
    }
}

use crate::zombie::*;
use bevy::{ecs::error::info, prelude::*};
use component::*;
use rand::{RngExt, rng};

mod component;
mod zombie;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Money { money: 0.0 })
        .insert_resource(ZombieCount {
            normal: 0,
            toxick: 0,
            fat: 0,
        })
        .insert_resource(ThreateLevel {
            level: 1,
            killed: 0,
            need_killed: 30,
            upgrade_cost: 100.0,
            spawn_per_second: 0.5,
        })
        .insert_resource(SpawnLimit {
            normal_max: 5,
            toxick_max: 5,
            fat_max: 5,
        })
        .add_systems(
            Startup,
            (camera_game, fon_game, command_center, player_entity),
        )
        .add_systems(
            Update,
            (
                death_entity_translation,
                spawn_all_zombie,
                zombie_statistic,
                death_entity,
                move_entity,
                level_up,
                attack_player,
                collision_geschoss_zombie,
                attack_zombie,
                collision_geschoss_player,
                death_player,
                player_respawn,
            ),
        )
        .run();
}

//камера
fn camera_game(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2d));
}

//фон
fn fon_game(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.4, 0.1), Vec2::new(2000.0, 1000.0)),
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));
}

// база
fn command_center(mut commands: Commands) {
    commands.spawn((
        Base,
        Hp {
            max_hp: 500.0,
            hp: 500.0,
        },
        Sprite::from_color(Color::srgb(0.6, 0.9, 0.8), Vec2::new(20.0, 80.0)),
        Transform::from_xyz(-500.0, -20.0, 1.0),
    ));
}

// Логика прокачки
fn level_up(
    mut level: ResMut<ThreateLevel>,
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

        spawn_limit.toxick_max = (spawn_limit.toxick_max as f32 * 1.5).round() as u32;

        spawn_limit.fat_max = (spawn_limit.fat_max as f32 * 1.5).round() as u32;
        println!("Уровень угрозы повышен до {}", level.level);
    }
}

//создание снаряда
fn geschoss(commands: &mut Commands, player_transform: &Transform, damage: f32) {
    commands.spawn((
        Geschoss,
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

// Уничтожение сущносте за картой
fn death_entity_translation(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    query: Query<(
        Entity,
        &Transform,
        Option<&Geschoss>,
        Option<&Monster>,
        Option<&ZombiType>,
    )>,
) {
    for (entity, transfor, geschoss, monster, zombie_type) in query.iter() {
        if geschoss.is_some() && transfor.translation.x > 2000.0 {
            commands.entity(entity).despawn();
            info!("Сущность пуля удалена");
        }
        if geschoss.is_some() && transfor.translation.x < -2000.0{
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

// Создание игрока
fn player_entity(mut commands: Commands) {
    commands.spawn((
        Player,
        Hitbox { x: 40.0, y: 160.0 },
        Damage { damage: 10.0 },
        Hp {
            max_hp: 100.0,
            hp: 100.0,
        },
        Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(40.0, 160.0)),
        Transform::from_xyz(-420.0, -40.0, 1.0),
    ));
}
// Респавн игрока
fn player_respawn(
    mut commands: Commands,
    mut timer: Local<Timer>,
    time: Res<Time>,
    death_player: Query<Entity, With<Player>>,
    mut attackers: Query<Entity, With<IsAttacking>>
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(4.0, TimerMode::Once)
    }

    if death_player.is_empty() {
        for entity in attackers.iter_mut() {
                commands.entity(entity).remove::<IsAttacking>();
            }
        timer.tick(time.delta());
        if timer.is_finished() {
            

            player_entity(commands);
            timer.reset();
        }
    }
}

// Создание аттаки
fn attack_player(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    player_query: Query<(&Transform, &Damage), With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let (player_transform, damage) = player_query.single().unwrap();

        geschoss(&mut commands, player_transform, damage.damage);
    }
}

// Логика передвижение
pub fn move_entity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), Without<IsAttacking>>,
) {
    for (mut transform, speed) in query.iter_mut() {
        transform.translation.x += speed.speed * time.delta_secs();
    }
}
// Проверка хитбокса
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

// Логика смерти
fn death_entity(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    mut threat_level: ResMut<ThreateLevel>,
    mut money: ResMut<Money>,
    query: Query<(Entity, &Hp, &ZombiType, &Reward), With<Monster>>,
) {
    for (entity, hp, zombie_type, reward) in query.iter() {
        if hp.hp <= 0.0 {
            match zombie_type {
                ZombiType::Normal => zombie_count.normal -= 1,
                ZombiType::Toxick => zombie_count.toxick -= 1,
                ZombiType::Fat => zombie_count.fat -= 1,
            }
            money.money += reward.money;
            threat_level.killed += 1;
            info!("денег на счету {}", money.money);
            commands.entity(entity).despawn();
        }
    }
}

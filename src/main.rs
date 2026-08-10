use bevy::prelude::*;
use component::*;
use rand::{RngExt, rng};

mod component;

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
                move_zombie,
                level_up,
                attack_player,
                collision_geschoss_zombie,
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

// Статистика
fn zombie_statistic(count: Res<ZombieCount>, time: Res<Time>, mut timer: Local<Timer>) {
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

//уничтожение сущносте за картой
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

//создание игрока
fn player_entity(mut commands: Commands) {
    commands.spawn((
        Player,
        Damage { damage: 10.0 },
        Hp {
            max_hp: 100.0,
            hp: 100.0,
        },
        Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(40.0, 160.0)),
        Transform::from_xyz(-420.0, -40.0, 1.0),
    ));
}

//Создание аттаки
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

//Проэктирование зомби
fn setup_monster_zombie(
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
fn spawn_all_zombie(
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

// Логика передвижение
fn move_zombie(time: Res<Time>, mut query: Query<(&mut Transform, &Speed)>) {
    for (mut transform, speed) in query.iter_mut() {
        transform.translation.x += speed.speed * time.delta_secs();
    }
}
// Логика попадние
fn collision_geschoss_zombie(
    mut commands: Commands,
    mut query: Query<(&mut Hp, &Hitbox, &Transform), With<Monster>>,
    geschoss: Query<(Entity, &Hitbox, &Transform, &Damage), With<Geschoss>>,
) {
    for (geschoss_entity, geschoss_hitbox, geschoss_transform, damage) in geschoss.iter() {
        for (mut hp, zombie_hitbox, zombie_transform) in query.iter_mut() {
            let x_distance =
                (geschoss_transform.translation.x - zombie_transform.translation.x).abs();

            let y_distance =
                (geschoss_transform.translation.y - zombie_transform.translation.y).abs();

            if x_distance < (geschoss_hitbox.x + zombie_hitbox.x) / 2.0
                && y_distance < (geschoss_hitbox.y + zombie_hitbox.y) / 2.0
            {
                hp.hp -= damage.damage;
                info!("HP зомби: {}", hp.hp);
                commands.entity(geschoss_entity).despawn();
                break;
            }
        }
    }
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

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

pub fn bullet_zombie(commands: &mut Commands, zombi_type: &Transform, damage: f32) {
    commands.spawn((
        Bullet,
        ZombieProjectile,
        Damage { damage },
        Speed { speed: -200.0 },
        Hitbox { y: 1000.0, x: 10.0 },
        Sprite::from_color(Color::srgb(0.0, 0.0, 0.0), Vec2::new(10.0, 5.0)),
        Transform::from_xyz(zombi_type.translation.x, zombi_type.translation.y, 2.9),
    ));
}

pub fn setup_monster_zombie(
    zombi_type: ZombiType,
    commands: &mut Commands,
    zombie_count: &mut ZombieCount,
    asset_server: &AssetServer,
) {
    let (monster, range, hitbox, speed, damage, hp, reward, texture, zombie) =
        match zombi_type {
            ZombiType::Normal => (
                Monster,
                RangeAttack { range: 5.0 },
                Hitbox { x: 40.0, y: 80.0 },
                Speed { speed: -10.0 },
                Damage { damage: 8.0 },
                Hp {
                    max_hp: 50.0,
                    hp: 50.0,
                },
                Reward { money: 10.0 },
                asset_server.load("textures/zombie_normal.png"),
                zombie_count.normal += 1,
            ),

            ZombiType::Toxick => (
                Monster,
                RangeAttack { range: 70.0 },
                Hitbox { x: 40.0, y: 75.0 },
                Speed { speed: -15.0 },
                Damage { damage: 11.0 },
                Hp {
                    max_hp: 40.0,
                    hp: 40.0,
                },
                Reward { money: 12.0 },
                asset_server.load("textures/zombie_toxic.png"),
                zombie_count.toxick += 1,
            ),

            ZombiType::Fat => (
                Monster,
                RangeAttack { range: 2.0 },
                Hitbox { x: 60.0, y: 78.0 },
                Speed { speed: -3.0 },
                Damage { damage: 50.0 },
                Hp {
                    max_hp: 200.0,
                    hp: 200.0,
                },
                Reward { money: 50.0 },
                asset_server.load("textures/zombie_fat.png"),
                zombie_count.fat += 1,
            ),
        };

    commands.spawn((
        zombi_type,
        monster,
        range,
        hitbox,
        speed,
        damage,
        hp,
        reward,
        Sprite::from_image(texture),
        Transform::from_xyz(
            800.0,
            rng().random_range(-60.0..60.0),
            1.0,
        ),
        zombie,
    ));
}

pub fn spawn_all_zombie(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    spawn_limit: Res<SpawnLimit>,
    level: Res<ThreatLevel>,
    asset_server: Res<AssetServer>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(3.0 / level.spawn_per_second, TimerMode::Repeating);
    }
    if timer.tick(time.delta()).just_finished() {
        if zombie_count.normal < spawn_limit.normal_max {
            setup_monster_zombie(ZombiType::Normal, &mut commands, &mut zombie_count, &asset_server,);
        }
        if zombie_count.toxick < spawn_limit.toxic_max {
            setup_monster_zombie(ZombiType::Toxick, &mut commands, &mut zombie_count, &asset_server);
        }
        if zombie_count.fat < spawn_limit.fat_max {
            setup_monster_zombie(ZombiType::Fat, &mut commands, &mut zombie_count, &asset_server);
        }
    }
}

pub fn attack_zombie(
    mut commands: Commands,
    monster_query: Query<(Entity, &Transform, &Damage, &RangeAttack), With<Monster>>,
    wall_query: Query<(&Transform, &Hitbox), With<Wall>>,
    mut timer: Local<Timer>,
    time: Res<Time>,
) {
    let Ok((wall_transform, wall_hitbox)) = wall_query.single() else {
        return;
    };

    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.3, TimerMode::Repeating)
    }
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    for (entity, monster_transform, damage, range) in &monster_query {
        let wall_right = wall_transform.translation.x + wall_hitbox.x / 2.0;
        let distance = monster_transform.translation.x - wall_right;

        if distance >= 0.0 && distance <= range.range {
            bullet_zombie(&mut commands, monster_transform, damage.damage);
            commands.entity(entity).insert(IsAttacking);
        }
    }
}

pub fn collision_bullet_zombie(
    mut commands: Commands,
    mut zombies: Query<(Entity, &mut Hp, &Hitbox, &Transform), With<Monster>>,
    bullet: Query<(Entity, &Hitbox, &Transform, &Damage), (With<Bullet>, With<PlayerProjectile>)>,
) {
    for (bullet_entity, bullet_hitbox, bullet_transform, damage) in &bullet {
        let target = zombies
            .iter_mut()
            .find(|(_, _, zombie_hitbox, zombie_transform)| {
                check_hitbox(
                    bullet_hitbox,
                    bullet_transform,
                    zombie_hitbox,
                    zombie_transform,
                )
            });

        if let Some((_, mut hp, _, _)) = target {
            hp.hp -= damage.damage;
            info!("HP зомби: {}", hp.hp);
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn death_zombie_entity(
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    mut threat_level: ResMut<ThreatLevel>,
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

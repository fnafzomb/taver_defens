
use bevy::{prelude::*};
use rand::{RngExt, rng};
use component::*;

mod component;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Money{
            money: 0.0,
        })
        .insert_resource(ZombieCount{
            noraml: 0,
            toxick: 0,
            fat: 0,
        })
        .insert_resource(ThreateLevel{
            level: 1,
            killed: 0,
            need_killed: 30,
            upgrade_cost: 100.0,
        })
        .insert_resource(SpawnLimit{
            normal_max: 5,
            toxick_max: 5,
            fat_max: 5,
        })
        .add_systems(Startup, (
           
            camera_game, 
            fon_game, 
            command_center, 
            player_entety, 
        ))
        .add_systems(Update, (
            zombie_statistic,
            move_zombie,
            death_base,
            death_player,
            death_zombie,
            level_up, spawn_all_zombie,
            attack_player,
            // hp_monster_logic,
        ))
        .run();
}

//камера
fn camera_game(mut commands: Commands){
    commands.spawn((
        MainCamera,
        Camera2d
    ));
}
//фон 
fn fon_game(mut commands: Commands){
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.3, 0.4, 0.1),
            Vec2::new  (100.0, 100.0),        
        ),
        Transform::from_xyz(0.0,0.0, 0.1)
    ));
}

// база
fn command_center(
    mut commands: Commands,
    
){
    commands.spawn((
        Base,
        Hp  {
            max_hp: 500.0,
            hp: 500.0
        },
        Sprite::from_color(
            Color::srgb (0.6,0.9,0.8),
            Vec2::new(20.0,80.0),
        ),
        Transform::from_xyz(-500.0, -20.0, 1.0)
    ));
}

fn zombie_statistic(
    count: Res<ZombieCount>,
){
    println!(
        "Зомби на карте: обычных {}, токсичных {}, толстых {} | всего {}",
        count.noraml,
        count.toxick,
        count.fat,
        count.noraml + count.toxick + count.fat
    );
}
// Логика прокачки
fn level_up(
    mut level: ResMut<ThreateLevel>,
    mut money: ResMut<Money>,
    mut spawn_limit: ResMut<SpawnLimit>,
){
    if level.killed >= level.need_killed && money.money >= level.upgrade_cost{
        level.level += 1;
        level.killed = 0;
        level.need_killed = (level.need_killed as f32 * 1.5) as u32;
        money.money -= level.upgrade_cost;
        level.upgrade_cost *= 1.8;
        spawn_limit.normal_max += 5;
        spawn_limit.toxick_max += 5;
        spawn_limit.fat_max += 5;
        println!("Уровень угрозы повышен до {}", level.level);
    }
}

//создание снаряда
fn geschoss(
    commands: &mut Commands,
    player_transform: &Transform
){
    commands.spawn((
        Geschoss,
        Hitbox{
            y: 100.0,
            x: 10.0
        },
        Sprite::from_color(
            Color::srgb(0.0, 0.0 , 0.0), 
            Vec2::new(10.0, 5.0)),
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y,
            2.9,
        ),
    ));
}

//создание игрока
fn player_entety(
    mut commands: Commands,
){
    commands.spawn((
        Player,
        Damage{
            damage: 10.0
        },
        Hp{
            max_hp: 100.0,
            hp: 100.0
        },
        Sprite::from_color(
            Color::srgb(1.0, 1.0, 1.0),
            Vec2::new(40.0, 160.0)
        ),
        Transform::from_xyz(-420.0, -40.0, 1.0)
    ));
}

//Создание аттаки
fn attack_player(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let player_transform = player_query.single().unwrap();

        geschoss(&mut commands, player_transform);
    }
}

//Проэктирование зомби
fn setup_monster_zombie(
    zombi_type: ZombiType,
    commands: &mut Commands,
    zombie_count: &mut ZombieCount,
){
    let (monster, damage, hp,  reward,color, size, zombie) = match zombi_type{
        ZombiType::Normal =>(
            Monster{speed: 10.0},
            Damage{damage:8.0},
            Hp{max_hp:50.0, hp:50.0},
            Reward{money:10.0},
            Color::srgb(0.2, 0.8, 0.2),
            Vec2::new(40.0, 80.0),
            zombie_count.noraml += 1,
        ),
        ZombiType::Toxick => (
            Monster{speed: 15.0},
            Damage{damage:11.0},
            Hp{max_hp:40.0,hp:40.0},
            Reward{money:12.0},
            Color::srgb(0.7, 0.2, 0.8),
            Vec2::new(40.0, 75.0),
            zombie_count.toxick += 1,
        ),
        ZombiType::Fat =>(
            Monster{speed:3.0},
            Damage{damage:50.0},
            Hp{max_hp:200.0,hp:200.0},
            Reward{money:50.0},
            Color::srgb(0.6, 0.4, 0.2),
            Vec2::new(60.0,78.0),
            zombie_count.fat += 1,
        ),
        
    };
    commands.spawn((
        zombi_type,
        monster,
        damage,
        hp,
        reward,
        Sprite::from_color(
            color,
            size,
        ),
        Transform:: from_xyz(
            800.0,
            rng().random_range(-100.0..100.0),
            1.0
        ),
        zombie,
    ));
}

fn spawn_all_zombie(
    mut commands: Commands,
    mut zombie_count : ResMut<ZombieCount>,
){
    setup_monster_zombie(ZombiType::Normal, &mut commands, &mut zombie_count);
    setup_monster_zombie(ZombiType::Toxick, &mut commands, &mut zombie_count);
    setup_monster_zombie(ZombiType::Fat, &mut commands, &mut zombie_count);
}
// Логика передвижение
fn move_zombie(
    time : Res<Time>,
    mut query : Query<(&mut Transform, &Monster)>
){
    for (mut transform, monster) in  query.iter_mut(){
        transform.translation.x -= monster.speed *  time.delta_secs();
    }
}

// Логика нанесение урона зомби
// fn hp_monster_logic(
//     player_query: Query<&Damage, With<Player>>,
//     mut monster_query: Query<&mut Hp, With<Monster>>,
// ) {
//     let damage = player_query.single().unwrap().damage;

//     for mut hp in monster_query.iter_mut() {
//         hp.hp -= damage;
//         println!("Зомби получил урон, осталось {}", hp.hp);
//     }
// }



// Логика смерти
fn death_base(
    query : Query<(Entity, &Hp), With<Base>>,
    mut commands: Commands
){
    for (entity, hp) in query.iter() {
        if hp.hp <= 0.0{
            commands.entity(entity).despawn();
            println!("База уничтожена")
        }
    }
}

fn death_player(
    query : Query<(Entity, &Hp), With<Player>>,
    mut commands: Commands
){
    for (entity, hp) in query.iter() {
        if hp.hp <= 0.0{
            commands.entity(entity).despawn();
            println!("Игрок  умер")
        }
    }
}

fn death_zombie (
    query : Query<(Entity, &Hp, &Reward, &ZombiType), With<Monster>>,
    mut commands: Commands,
    mut money: ResMut<Money>,
    mut level: ResMut<ThreateLevel>,
    mut zombie_count: ResMut<ZombieCount>,
){
    for (entity, hp, reward, zombie_type) in  query.iter(){
        if hp.hp <= 0.0{
            match zombie_type {
                ZombiType::Normal =>{
                    zombie_count.noraml -= 1;
                },
                ZombiType::Toxick =>{
                    zombie_count.toxick -= 1;
                },
                ZombiType::Fat => {
                    zombie_count.fat -= 1;
                }
            } 
            if hp.hp <= 0.0{}
            commands.entity(entity).despawn();
            println!("Зомби умер");           
            money.money += reward.money;
            level.killed += 1;
            
        }
    }
}
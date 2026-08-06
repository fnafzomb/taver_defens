
use bevy::{prelude::*};
use rand::{RngExt, rng};

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
        },
        )
        .add_systems(Startup, (
            camera_game, 
            fon_game, 
            command_center, 
            player_entety, 
        ))
        .add_systems(Update, (
            economics,
            // hp_base,
            // hp_player,
            // hp_monster,
            move_zombie,
            death_base,
            death_player,
            death_zombie,
            // hp_monster_logic,
            setup_monster_zombie,
            setup_monster_toxick_zombie,
            setup_monster_fat_zombie,
        ))
        .run();
}


// Компоненты зомби
#[derive(Resource)]
struct ZombieCount{
    noraml: u32,
    toxick: u32,
    fat:  u32,
}

#[derive(Component)]
enum ZombiType {
    Normal,
    Toxick,
    Fat,
}
// Компоненты хп и домага


#[derive(Component)]
struct Hp {
    max_hp: f32,
    hp: f32
}
#[derive(Component)]
struct Damage{
    damage: f32
}
// Деньги
#[derive(Resource)]
struct Money{
    money: f32
}

// Компаненты обьектов

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Monster {
    speed: f32,
}

#[derive(Component)]
struct Base;

#[derive(Component)]
struct MainCamera;

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
//Проэктирование зомби
fn setup_monster_zombie(
    mut commands: Commands,
){
    let mut rng = rng();
    let y = rng.random_range(-80.0..80.0);
    commands.spawn((
        Monster{
            speed: 10.0,
        },
        Damage{
            damage: 9.0,
        },
        Hp{
            max_hp: 50.0,
            hp: 50.0
        },
        Sprite::from_color(
            Color::srgb(0.18, 0.19, 0.13),
            Vec2 ::new(20.0,80.0),
        ),
        Transform::from_xyz(800.0, y, 1.0),
    ));
    println!("Зомби появился на координатах: y = {}", y);
}

//Проэктирование токсичного зомби
fn setup_monster_toxick_zombie(
    mut commands: Commands,
){
    
    let mut rng = rng();
    let y = rng.random_range(-80.0..80.0);
    commands.spawn((
        Monster{
            speed: 12.0,
        },
        Damage{
            damage: 9.0,
        },
        Hp{
            max_hp: 40.0,
            hp: 40.0
        },
        Sprite::from_color(
            Color::srgb(0.11, 0.08, 0.02),
            Vec2 ::new(20.0,60.0),
        ),
        Transform::from_xyz(800.0, y, 1.0),
    ));
    println!("Токсичный зомби появился на координатах: y = {}", y);
}



//Проэктирование  толстого зомби
fn setup_monster_fat_zombie(
    mut commands: Commands
){
    let mut rng = rng();
    let y = rng.random_range(-80.0..80.0);
    commands.spawn((
        Monster{
            speed: 3.0,
        },
        Damage{
            damage: 53.0,
        },
        Hp{
            max_hp: 250.0,
            hp: 250.0
        },
        Sprite::from_color(
            Color::srgb(0.21, 0.38, 0.92),
            Vec2 ::new(20.0,60.0),
        ),
        Transform::from_xyz(800.0, y, 1.0),   
    ));
    println!("Толстый зомби появился на координатах: y = {}", y);
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
    query : Query<(Entity, &Hp), With<Monster>>,
    mut commands: Commands
){
    for (entity, hp) in  query.iter(){
        if hp.hp <= 0.0{
            commands.entity(entity).despawn();
            println!("Зомби умер")
        }
    }
}



// логика урона 
fn damage_base(
    damge_queru: Query<&mut Hp, (With<Base>, )>,
){

}   

fn damage_player(

){

}

fn damage_monster(

){

}

// Проверка хп
// Changed  -  реагирует только на изменение компанента
// fn hp_base (
//     query: Query<&Hp, (With<Base>,Changed<Hp>)>,
// ){
//     for hp in query.iter(){
//         if hp.hp <= 0.0 {
//             println!("уничтожена")
//         } else {
//             println!("получен урон: {}",  hp.hp)
//         }
//     }
// }

// fn hp_player(
//     query: Query<&Hp, (With<Player>,Changed<Hp>)>
// ){
//     for hp in query.iter(){
//         if hp.hp <= 0.0 {
//             println!("Мёртв")
//         } else {
//             println!("получен урон{}",  hp.hp)
//         }
//     }
// }

// fn hp_monster (
//     query: Query<&Hp, (With<Monster>,Changed<Hp>)>
// ){
//     for hp in query.iter() {
//         if hp.hp <= 0.0 {
//             println!("Мёртв")
//         } else {
//             println!("получен урон{}", hp.hp)
//         }
//     }
// }



// Эканомика
fn economics(
   mut  economy: ResMut<Money>, 
){ 
    // economy.money += 1.0;
    // println!("Деньги: {}", economy.money);
}

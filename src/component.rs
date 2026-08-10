use bevy::prelude::*;

#[derive(Resource)]
pub struct ZombieCount {
    pub normal: u32,
    pub toxick: u32,
    pub fat: u32,
}

// Компонент уровня
#[derive(Resource)]
pub struct ThreateLevel {
    pub level: u32,
    pub killed: u32,
    pub need_killed: u32,
    pub upgrade_cost: f32,
    pub spawn_per_second: f32,
}

// Компаненты максимального спавна
#[derive(Resource)]
pub struct SpawnLimit {
    pub normal_max: u32,
    pub toxick_max: u32,
    pub fat_max: u32,
}

#[derive(Component)]
pub enum ZombiType {
    Normal,
    Toxick,
    Fat,
}

// Компоненты хп и домага
#[derive(Component)]
pub struct Hitbox {
    pub y: f32,
    pub x: f32,
}

#[derive(Component)]
pub struct Hp {
    pub max_hp: f32,
    pub hp: f32,
}

#[derive(Component)]
pub struct Damage {
    pub damage: f32,
}

// Деньги
#[derive(Resource)]
pub struct Money {
    pub money: f32,
}

#[derive(Component)]
pub struct Reward {
    pub money: f32,
}

// Компаненты обьектов
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Base;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Geschoss;

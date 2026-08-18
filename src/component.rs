use bevy::prelude::*;

//Статистика
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

// Радиус атаки
#[derive(Component)]
pub struct RangeAttack {
    pub range: f32,
}



// Компаненты максимального спавна
#[derive(Resource)]
pub struct SpawnLimit {
    pub normal_max: u32,
    pub toxick_max: u32,
    pub fat_max: u32,
}
// Типы зомби
#[derive(Component)]
pub enum ZombiType {
    Normal,
    Toxick,
    Fat,
}

// Хитбокс
#[derive(Component)]
pub struct Hitbox {
    pub y: f32,
    pub x: f32,
}
// Хп
#[derive(Component)]
pub struct Hp {
    pub max_hp: f32,
    pub hp: f32,
}
// Урон
#[derive(Component)]
pub struct Damage {
    pub damage: f32,
}

// Деньги
#[derive(Resource)]
pub struct Money {
    pub money: f32,
}
// Награда
#[derive(Component)]
pub struct Reward {
    pub money: f32,
}

// Игрок
#[derive(Component)]
pub struct Player;
// Скорость
#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

// Игрок снаряд
#[derive(Component)]
pub struct PlayerProjectile;

// Зомби Снаряд
#[derive(Component)]
pub struct ZombieProjectile;

// Зомби
#[derive(Component)]
pub struct Monster;

// База
#[derive(Component)]
pub struct Base;

// Камера
#[derive(Component)]
pub struct MainCamera;

// Пуля
#[derive(Component)]
pub struct Geschoss;

// Остановка перед аттакай
#[derive(Component)]
pub struct IsAttacking;

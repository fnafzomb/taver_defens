use bevy::prelude::*;

// Zombie statistic
#[derive(Resource)]
pub struct ZombieCount {
    pub normal: u32,
    pub toxick: u32,
    pub fat: u32,
}

// Level component
#[derive(Resource)] 
pub struct ThreatLevel {
    pub level: u32,
    pub killed: u32,
    pub need_killed: u32,
    pub upgrade_cost: f32,
    pub spawn_per_second: f32,
}

#[derive(Component)]
pub struct RangeAttack {
    pub range: f32,
}

// Component max spawn
#[derive(Resource)]
pub struct SpawnLimit {
    pub normal_max: u32,
    pub toxic_max: u32,
    pub fat_max: u32,
}

#[derive(Component)]
pub enum ZombiType {
    Normal,
    Toxick,
    Fat,
}

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

#[derive(Resource)]
pub struct Money {
    pub money: f32,
}

#[derive(Component)]
pub struct Reward {
    pub money: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed {
    pub speed: f32,
}

#[derive(Component)]
pub struct PlayerProjectile;

#[derive(Component)]
pub struct ZombieProjectile;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Base;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct IsAttacking;

#[derive(Component)]
pub struct Wall;

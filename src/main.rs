use crate::{base::*, player::*, util::*, wall::*, zombie::*};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use component::*;
use rand::{RngExt, rng};

mod base;
mod component;
mod player;
mod util;
mod wall;
mod zombie;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Money { money: 0.0 })
        .insert_resource(ZombieCount {
            normal: 0,
            toxick: 0,
            fat: 0,
        })
        .insert_resource(ThreatLevel {
            level: 1,
            killed: 0,
            need_killed: 30,
            upgrade_cost: 100.0,
            spawn_per_second: 0.5,
        })
        .insert_resource(SpawnLimit {
            normal_max: 5,
            toxic_max: 5,
            fat_max: 5,
        })
        .add_systems(
            Startup,
            (
                camera_game,
                fon_game,
                command_center,
                player_entity,
                wall_entity,
            ),
        )
        .add_systems(
            Update,
            (
                wall_respawn,
                delete_entity,
                spawn_all_zombie,
                zombie_statistic,
                death_zombie_entity,
                move_entity,
                level_up,
                attack_player,
                collision_bullet_zombie,
                attack_zombie,
                death_wall,
                collision_bullet_wall,
            ),
        )
        .run();
}

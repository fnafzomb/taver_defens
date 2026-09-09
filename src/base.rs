use crate::*;

// base
pub fn command_center(mut commands: Commands) {
    commands.spawn((
        Base,
        Hp {
            max_hp: 500.0,
            hp: 500.0,
        },
        Hitbox{
            x: 20.0,
            y: 80.0
        },
        Sprite::from_color(Color::srgb(0.6, 0.9, 0.8), Vec2::new(20.0, 80.0)),
        Transform::from_xyz(-500.0, -20.0, 1.0),
    ));
}

pub fn damage_base(
    mut base: Query<(Entity, &mut Hp, &Hitbox, &Transform), With<Base>>,
    mut commands: Commands,
    mut zombie_count: ResMut<ZombieCount>,
    zombies: Query<(Entity, &Transform, &Hitbox, &Damage, &ZombiType), With<Monster>>,
    
){
    for (zombie_entity, zombie_transfor, zombie_hitbox,damage, zombi_type) in &zombies{
        let target = base
        .iter_mut()
        .find(|(_, _, base_hitbox, base_transform)|{
            check_hitbox(
                zombie_hitbox,
                zombie_transfor,
                base_hitbox,
                base_transform,
            )
        });

        if let Some((_, mut hp, _, _)) = target {
            hp.hp -= damage.damage;
            info!("HP  базы: {}", hp.hp);
            commands.entity(zombie_entity).despawn();
            match zombi_type {
                ZombiType::Fat => zombie_count.fat -= 1,
                ZombiType::Normal => zombie_count.normal -=1,
                ZombiType::Toxick => zombie_count.toxick -=1
            }
        }
    }
}

pub fn death_base(
    mut commands: Commands, 
    query:Query<(Entity, &Hp),With<Base>>
){
    for (entity,hp)  in query.iter(){
        if hp.hp <= 0.0{
            commands.entity(entity).despawn();
        }
    }
}
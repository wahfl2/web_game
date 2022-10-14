use bevy::prelude::*;

#[derive(Component)]
pub struct Level;

pub struct LevelEntity {
    pub entity: Option<Entity>,
}

pub fn level_startup(
    mut commands: Commands,
    mut level_entity: ResMut<LevelEntity>,
) {
    let entity = commands.spawn_bundle(TransformBundle::default())
        .insert(Level)
        .insert_bundle(VisibilityBundle::default()).id();

    level_entity.entity = Some(entity);
}
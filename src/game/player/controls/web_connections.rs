use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{util::{EntityQuery, ExtraTransformMethods}, game::player::components::*};

use super::controls::WebPartEntities;

pub fn update_web_connections(
    mut commands: Commands,
    
    web_parts: Res<WebPartEntities>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    web_connections: Query<(Entity, &WebPartConnection)>,

    mut transform_query: Query<&mut Transform>,
) {
    if web_parts.entities.is_empty() { return }

    if web_connections.is_empty() {
        for i in 1..web_parts.entities.len() {
            let e1 = web_parts.entities[i-1];
            let e2 = web_parts.entities[i];

            let p1 = transform_query.get(e1).unwrap().translation;
            let p2 = transform_query.get(e2).unwrap().translation;

            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Box::new(1.0, 1.0, 0.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_pt_to_pt(p1, p2, 8.0),
                ..default()
            }).insert(WebPartConnection { e1, e2 });
        }
    } else {
        for (entity, connection) in web_connections.iter() {
            let p1 = transform_query.get(connection.e1).unwrap().translation;
            let p2 = transform_query.get(connection.e2).unwrap().translation;

            let mut transform = transform_query.get_mut(entity).unwrap();
            *transform = Transform::from_pt_to_pt(p1, p2, 8.0);
        }
    }
}


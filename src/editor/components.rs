use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use serde::{Serialize, Deserialize};

use crate::METERS_PER_PIXEL;

use super::editor::SpawnShapeParam;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub enum ShapeType {
    Rectangle,
    Oval,
}

#[derive(Component)]
pub struct EditorSelectBox {
    pub start: Vec2
}

impl Default for EditorSelectBox {
    fn default() -> Self {
        Self { start: Vec2::ZERO }
    }
}

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct EditorShape {
    pub shape_type: ShapeType,
    pub half_extents: Vec2,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Hovered;

impl EditorShape {
    pub fn new(shape_type: ShapeType, half_extents: Vec2) -> Self {
        Self {
            shape_type,
            half_extents,
        }
    }

    pub fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam, transform: &Transform) {
        let coll_half_extents = self.half_extents * METERS_PER_PIXEL * 5.0; // ????

        let (collider, mesh_bundle) = match self.shape_type {
            ShapeType::Rectangle => {(
                Collider::cuboid(coll_half_extents.x, coll_half_extents.y),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(
                        shape::Box::new(2.0, 2.0, 0.0).into()
                    ).into(),

                    material: param.materials.add(ColorMaterial::from(Color::RED)),

                    transform: transform.clone()
                        .with_scale(self.half_extents.extend(1.0)) ,
                    ..default()
                }
            )},
            ShapeType::Oval => {(
                Collider::ball(coll_half_extents.x),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(shape::Circle::new(1.0).into()).into(),
                    material: param.materials.add(ColorMaterial::from(Color::RED)),
                    transform: transform.clone().with_scale(self.half_extents.extend(1.0)),
                    ..default()
                }
            )},
        };

        let child = commands.spawn_bundle(mesh_bundle)
            .insert_bundle((
                collider,
                RigidBody::Fixed,
                Friction::coefficient(0.7),
                self,
            )).id();

        commands.entity(param.level.single()).add_child(child);
    }
}
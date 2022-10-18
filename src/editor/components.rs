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
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Hoverable;

impl EditorShape {
    pub fn new(shape_type: ShapeType, dimensions: Vec2) -> Self {
        Self {
            shape_type,
        }
    }

    pub fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam, transform: &Transform) {
        let (collider, mesh_bundle) = match self.shape_type {
            ShapeType::Rectangle => {(
                Collider::cuboid(1.0, 1.0),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(
                        shape::Box::new(2.0, 2.0, 0.0).into()
                    ).into(),

                    material: param.materials.add(ColorMaterial::from(Color::RED)),

                    transform: transform.clone(),
                    ..default()
                }
            )},
            ShapeType::Oval => {(
                Collider::ball(1.0),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(shape::Circle::new(1.0).into()).into(),
                    material: param.materials.add(ColorMaterial::from(Color::RED)),
                    transform: transform.clone(),
                    ..default()
                }
            )},
        };

        let child = commands.spawn_bundle(mesh_bundle)
            .insert_bundle((
                collider,
                RigidBody::Fixed,
                Friction::coefficient(0.1),
                Restitution::coefficient(0.4),
                CollisionGroups::new(
                    Group::from_bits_truncate(0b1), 
                    Group::from_bits_truncate(0b11111110)
                ),
                self,
            )).id();

        commands.entity(param.level.single()).add_child(child);
    }
}
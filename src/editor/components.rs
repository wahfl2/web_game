use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use serde::{Serialize, Deserialize};

use crate::util::ColorUpdate;

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
    pub stickable: bool,
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
    pub fn new(shape_type: ShapeType, stickable: bool) -> Self {
        Self {
            shape_type,
            stickable
        }
    }

    pub fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam, transform: &Transform) {
        let color = ColorUpdate {
            selected: false,
            hovered: false,
            stickable: self.stickable,
        }.get_color();

        let (collider, mesh_bundle) = match self.shape_type {
            ShapeType::Rectangle => {(
                Collider::cuboid(1.0, 1.0),
                MaterialMesh2dBundle {
                    mesh: param.preload.meshes.get("box 2").unwrap().clone(),
                    material: param.preload.get_bw_color_handle(color).clone(),
                    transform: transform.clone(),
                    ..default()
                }
            )},
            ShapeType::Oval => {(
                Collider::ball(1.0),
                MaterialMesh2dBundle {
                    mesh: param.preload.meshes.get("circle 1").unwrap().clone(),
                    material: param.preload.get_bw_color_handle(color).clone(),
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
                Selectable,
            )).id();

        commands.entity(param.level.single()).add_child(child);
    }
}
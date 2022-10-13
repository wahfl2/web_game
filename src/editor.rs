use std::f32::consts::PI;
use std::{fs, marker::PhantomData};
use std::time::Instant;

use bevy::{prelude::*, math::Vec3Swizzles, sprite::MaterialMesh2dBundle, ecs::system::SystemParam};
use bevy_rapier2d::prelude::*;
use serde::{Serialize, Deserialize};

use crate::util::{EntityQuery, Cursor};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub enum ShapeType {
    Rectangle,
    Oval,
}

#[derive(SystemParam)]
pub struct SpawnShapeParam<'w, 's> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<ColorMaterial>>,
    
    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct EditorShape {
    shape_type: ShapeType,
    half_extents: Vec2,
}

impl EditorShape {
    pub fn new(shape_type: ShapeType, half_extents: Vec2) -> Self {
        Self {
            shape_type,
            half_extents,
        }
    }

    fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam, transform: &Transform) {
        let (collider, mesh_bundle) = match self.shape_type {
            ShapeType::Rectangle => {(
                Collider::cuboid(self.half_extents.x, self.half_extents.y),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(shape::RegularPolygon::new(std::f32::consts::SQRT_2, 4).into()).into(),
                    material: param.materials.add(ColorMaterial::from(Color::RED)),
                    transform: transform.clone()
                        .with_scale(self.half_extents.extend(1.0))
                        .with_rotation(Quat::from_axis_angle(Vec3::Z, 0.25 * PI)),
                    ..default()
                }
            )},
            ShapeType::Oval => {(
                Collider::ball(self.half_extents.x),
                MaterialMesh2dBundle {
                    mesh: param.meshes.add(shape::Circle::new(1.0).into()).into(),
                    material: param.materials.add(ColorMaterial::from(Color::RED)),
                    transform: transform.clone().with_scale(self.half_extents.extend(1.0)),
                    ..default()
                }
            )},
        };

        commands.spawn_bundle(mesh_bundle)
        .insert_bundle((
            collider,
            Friction::coefficient(0.7),
            self,
        ));
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdeShape {
    shape: EditorShape,
    translation: Vec2,
    rotation: Quat,
}

impl SerdeShape {
    pub fn new(shape: EditorShape, transform: &Transform) -> Self {
        Self {
            shape,
            translation: transform.translation.xy(),
            rotation: transform.rotation,
        }
    }

    fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam) {
        self.shape.spawn(
            commands,
            param, 
            &Transform::from_translation(self.translation.extend(0.0)).with_rotation(self.rotation)
        );
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdeLevel {
    shapes: Vec<SerdeShape>,
}

impl SerdeLevel {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn push(&mut self, shape: SerdeShape) {
        self.shapes.push(shape);
    }
}

pub fn editor_load(
    mut commands: Commands,

    mut spawn_shape_param: SpawnShapeParam,
) {
    if let Ok(contents) = serde_json::from_str::<SerdeLevel>(
        fs::read_to_string("./level.json").unwrap().as_str()
    ) {
        for shape in contents.shapes {
            shape.spawn(&mut commands, &mut spawn_shape_param);
        }
    }
}

pub fn editor_save(
    keyboard_input: Res<Input<KeyCode>>,

    shapes: EntityQuery<EditorShape>,

    transform_query: Query<&Transform>,
    editor_shape_query: Query<&EditorShape>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        let start = Instant::now();
        let mut serde_level = SerdeLevel::new();

        for shape in shapes.iter() {
            let transform = transform_query.get(shape).unwrap();
            let editor_shape = editor_shape_query.get(shape).unwrap();

            let serde_shape = SerdeShape::new(editor_shape.clone(), transform);
            serde_level.push(serde_shape);
        }

        fs::write(
            "./level.json",
            serde_json::to_string_pretty(&serde_level).unwrap().as_str()
        ).expect("death");

        info!("Saved in {}ms", Instant::now().duration_since(start).as_millis());
    }
}

pub fn editor(
    mut commands: Commands,

    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor_input: Res<Cursor>,

    shapes: EntityQuery<EditorShape>,

    mut transform_query: Query<&mut Transform>,
    mut editor_shape_query: Query<&mut EditorShape>,

    mut spawn_shape_param: SpawnShapeParam,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        let shape = EditorShape {
            shape_type: ShapeType::Rectangle,
            half_extents: Vec2::new(20.0, 20.0),
        };

        shape.spawn(
            &mut commands, 
            &mut spawn_shape_param,
            &Transform::from_translation((cursor_input.pos).extend(0.0))
        );
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        let shape = EditorShape {
            shape_type: ShapeType::Oval,
            half_extents: Vec2::new(20.0, 20.0),
        };

        shape.spawn(
            &mut commands, 
            &mut spawn_shape_param,
            &Transform::from_translation((cursor_input.pos).extend(0.0))
        );
    }
}
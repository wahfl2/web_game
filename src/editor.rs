use std::f32::consts::PI;
use std::{fs, marker::PhantomData};
use std::time::Instant;

use bevy::{prelude::*, math::Vec3Swizzles, sprite::MaterialMesh2dBundle, ecs::system::SystemParam};
use bevy_rapier2d::prelude::*;
use serde::{Serialize, Deserialize};

use crate::METERS_PER_PIXEL;
use crate::level::{LevelEntity, Level};
use crate::util::{EntityQuery, Cursor};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SaveLoaded(false))
            .add_system(editor_load)
            .add_system(hover_shapes)
            .add_system(editor)
            .add_system(editor_save);
    }
}

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub enum ShapeType {
    Rectangle,
    Oval,
}

#[derive(SystemParam)]
pub struct SpawnShapeParam<'w, 's> {
    meshes: ResMut<'w, Assets<Mesh>>,
    materials: ResMut<'w, Assets<ColorMaterial>>,
    level: EntityQuery<'w, 's, Level>,
    
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

pub struct SaveLoaded(pub bool);

pub fn editor_load(
    mut commands: Commands,

    mut loaded: ResMut<SaveLoaded>,

    mut spawn_shape_param: SpawnShapeParam,
) {
    if loaded.0 { return }
    if spawn_shape_param.level.get_single().is_err() { return }

    if let Ok(contents) = serde_json::from_str::<SerdeLevel>(
        fs::read_to_string("./level.json").unwrap().as_str()
    ) {
        for shape in contents.shapes {
            shape.spawn(&mut commands, &mut spawn_shape_param);
        }
        loaded.0 = true;
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
    cursor: Res<Cursor>,

    camera: EntityQuery<Camera>,
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
            &Transform::from_translation((cursor.world_pos).extend(0.0))
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
            &Transform::from_translation((cursor.world_pos).extend(0.0))
        );
    }

    if mouse_button_input.pressed(MouseButton::Left) && !mouse_button_input.just_pressed(MouseButton::Left) {
        let mut transform = transform_query.get_mut(camera.single()).unwrap();
        transform.translation += cursor.delta().extend(0.0);
    }
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Hovered;

pub fn hover_shapes(
    mut commands: Commands,

    cursor: Res<Cursor>,
    rapier_context: Res<RapierContext>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    material_query: Query<&Handle<ColorMaterial>>,
    current_hovered_query: EntityQuery<Hovered>,
) {
    if cursor.moved {
        let mut last = None;
        rapier_context.intersections_with_point(
            cursor.world_pos, 
            QueryFilter::default(), 
            |entity| {
                last = Some(entity);
                true
           }
        );

        if let Some(hovered) = last {
            if !current_hovered_query.contains(hovered) {
                commands.entity(hovered).insert(Hovered);
                let mat_handle = material_query.get(hovered).unwrap();
                let material = materials.get_mut(mat_handle).unwrap();

                material.color = Color::YELLOW;
            }
        }

        for entity in current_hovered_query.iter() {
            if Some(entity) == last { continue }

            commands.entity(entity).remove::<Hovered>();
            let mat_handle = material_query.get(entity).unwrap();
            let material = materials.get_mut(mat_handle).unwrap();

            material.color = Color::RED;
        }
    }
}
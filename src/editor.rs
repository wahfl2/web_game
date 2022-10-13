use std::fs;
use std::time::Instant;

use bevy::{prelude::*, math::Vec3Swizzles, input::{mouse::MouseButtonInput, ButtonState}};
use bevy_rapier2d::prelude::*;
use serde::{Serialize, Deserialize};

use crate::util::{EntityQuery, Cursor};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub enum ShapeType {
    Rectangle,
    Oval,
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

    fn spawn(self, commands: &mut Commands, transform: &Transform) {
        let collider = match self.shape_type {
            ShapeType::Rectangle => {
                Collider::cuboid(self.half_extents.x, self.half_extents.y)
            },
            ShapeType::Oval => {
                Collider::ball(self.half_extents.x)
            },
        };

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(self.half_extents * 2.0),
                ..default()
            },
            transform: transform.clone(),
            ..default()
        })
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

    fn spawn(self, commands: &mut Commands) {
        self.shape.spawn(
            commands, 
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
) {
    commands.insert_resource(PrevMouseDown(false));
    commands.insert_resource(PrevSaveDown(false));

    if let Ok(contents) = serde_json::from_str::<SerdeLevel>(
        fs::read_to_string("./level.json").unwrap().as_str()
    ) {
        for shape in contents.shapes {
            shape.spawn(&mut commands);
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct PrevSaveDown(bool);

pub fn editor_save(
    keyboard_input: Res<Input<KeyCode>>,
    mut prev_save: ResMut<PrevSaveDown>,

    shapes: EntityQuery<EditorShape>,

    transform_query: Query<&Transform>,
    editor_shape_query: Query<&EditorShape>,
) {
    if keyboard_input.pressed(KeyCode::P) && !**prev_save {
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

    **prev_save = keyboard_input.pressed(KeyCode::P);
}

#[derive(Deref, DerefMut)]
pub struct PrevMouseDown(bool);

pub fn editor(
    mut commands: Commands,

    mouse_button_input: Res<Input<MouseButton>>,
    cursor_input: Res<Cursor>,
    mut prev_mouse: ResMut<PrevMouseDown>,
    windows: Res<Windows>,

    shapes: EntityQuery<EditorShape>,

    mut transform_query: Query<&mut Transform>,
    mut editor_shape_query: Query<&mut EditorShape>,
) {
    if mouse_button_input.pressed(MouseButton::Left) && !**prev_mouse {
        let window = windows.get_primary().unwrap();
        let window_size = Vec2::new(window.width(), window.height());

        let shape = EditorShape {
            shape_type: ShapeType::Rectangle,
            half_extents: Vec2::new(20.0, 20.0),
        };

        shape.spawn(
            &mut commands, 
            &Transform::from_translation((cursor_input.pos - (window_size / 2.0)).extend(0.0)))
    }
    
    **prev_mouse = mouse_button_input.pressed(MouseButton::Left);
}
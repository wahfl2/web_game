use std::fs;

use serde::{Serialize, Deserialize};
use bevy::{prelude::*, math::Vec3Swizzles, utils::Instant};

use crate::util::EntityQuery;

use super::{components::*, editor::SpawnShapeParam};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerdeShape {
    shape: EditorShape,
    translation: Vec2,
    rotation: Quat,
    scale: Vec2,
}

impl SerdeShape {
    pub fn new(shape: EditorShape, transform: &Transform) -> Self {
        Self {
            shape,
            translation: transform.translation.xy(),
            rotation: transform.rotation,
            scale: transform.scale.xy(),
        }
    }

    fn spawn(self, commands: &mut Commands, param: &mut SpawnShapeParam) {
        self.shape.spawn(
            commands,
            param, 
            &Transform::from_translation(self.translation.extend(1.0))
                .with_rotation(self.rotation)
                .with_scale(self.scale.extend(1.0))
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
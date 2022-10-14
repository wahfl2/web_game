use std::marker::PhantomData;

use bevy::{prelude::*, ecs::system::SystemParam};
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::level::Level;
use crate::util::{EntityQuery, Cursor};

use super::components::*;
use super::hover::hover_shapes;
use super::serde::*;

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

#[derive(SystemParam)]
pub struct SpawnShapeParam<'w, 's> {
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub level: EntityQuery<'w, 's, Level>,
    
    #[system_param(ignore)]
    marker: PhantomData<&'s usize>,
}

pub fn editor_startup(
    mut commands: Commands,
) {
    commands.spawn_bundle(TransformBundle::default())
        .insert_bundle((
            Collider::cuboid(0.5, 0.5),
            EditorSelectBox::default()
        )).insert(Sensor)
        .insert_bundle(VisibilityBundle {
            visibility: Visibility { is_visible: false },
            ..default()
        });
}

pub fn editor(
    mut commands: Commands,

    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,

    camera: EntityQuery<Camera>,
    shapes: EntityQuery<EditorShape>,
    mut select_box: Query<(Entity, &mut EditorSelectBox)>,

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

    if mouse_button_input.pressed(MouseButton::Middle) && !mouse_button_input.just_pressed(MouseButton::Middle) {
        let mut transform = transform_query.get_mut(camera.single()).unwrap();
        transform.translation += cursor.delta().extend(0.0);
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (entity, mut select_box) = select_box.single_mut();
        select_box.start = cursor.world_pos;

        let mut transform = transform_query.get_mut(entity).unwrap();
        transform.scale = Vec3::new(0.0, 0.0, 1.0);
    }
}
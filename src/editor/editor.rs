use std::marker::PhantomData;

use bevy::{prelude::*, ecs::system::SystemParam};
use bevy_rapier2d::prelude::{Collider, Sensor, RapierContext};

use crate::game::level::Level;
use crate::game::player::spawn::player_spawn;
use crate::util::{EntityQuery, Cursor, update_color_material, cursor_pos};

use super::camera::camera_movement;
use super::color_handler::color_handler;
use super::components::*;
use super::hover::hover_shapes;
use super::selection::selection_manipulation;
use super::serde::*;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(editor_startup)
            .add_startup_system(player_spawn)
            .add_system_to_stage(CoreStage::PreUpdate, camera_movement.after(cursor_pos))
            // Definitely off by one, but who cares
            .add_system(selection_manipulation)
            .add_system(editor_load)
            .add_system(hover_shapes)
            .add_system(editor.after(hover_shapes))
            .add_system_to_stage(CoreStage::PostUpdate, color_handler)
            .add_system_to_stage(CoreStage::PostUpdate, editor_save);
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
        )).insert(Sensor);
}

pub fn editor(
    mut commands: Commands,

    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor: Res<Cursor>,
    rapier_context: Res<RapierContext>,

    camera: EntityQuery<Camera>,
    shapes: EntityQuery<EditorShape>,
    selected: EntityQuery<Selected>,
    hovered: EntityQuery<Hovered>,
    mut select_box: Query<(Entity, &mut EditorSelectBox)>,

    material_query: Query<&Handle<ColorMaterial>>,
    mut transform_query: Query<&mut Transform>,
    mut editor_shape_query: Query<&mut EditorShape>,

    mut spawn_shape_param: SpawnShapeParam,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        let shape = EditorShape {
            shape_type: ShapeType::Rectangle,
        };

        shape.spawn(
            &mut commands, 
            &mut spawn_shape_param,
            &Transform::from_translation((cursor.world_pos).extend(0.0))
                .with_scale(Vec3::new(20.0, 20.0, 1.0))
        );
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        let shape = EditorShape {
            shape_type: ShapeType::Oval,
        };

        shape.spawn(
            &mut commands, 
            &mut spawn_shape_param,
            &Transform::from_translation((cursor.world_pos).extend(0.0))
                .with_scale(Vec3::new(20.0, 20.0, 1.0))
        );
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        if !keyboard_input.pressed(KeyCode::LShift) {
            for entity in selected.iter() {
                commands.entity(entity).remove::<Selected>();
            }
        }

        let (entity, mut select_box) = select_box.single_mut();
        select_box.start = cursor.world_pos;

        let mut transform = transform_query.get_mut(entity).unwrap();
        transform.scale = Vec3::new(0.0, 0.0, 1.0);

    } else if mouse_button_input.pressed(MouseButton::Right) {
        let (entity, select_box) = select_box.single();
        let mut transform = transform_query.get_mut(entity).unwrap();

        transform.translation = ((select_box.start + cursor.world_pos) / 2.0).extend(0.0);
        transform.scale = (select_box.start - cursor.world_pos).abs().extend(1.0);
    }

    if mouse_button_input.just_released(MouseButton::Right) {
        let (select_box_entity, box_select) = select_box.single();
        let select_box_size = (box_select.start - cursor.world_pos).abs();

        if select_box_size.x + select_box_size.y < 20.0 {
            if !hovered.is_empty() {
                for entity in hovered.iter() {
                    commands.entity(entity).insert(Selected);
                }
            }
        } else {
            for (e1, e2, _) in rapier_context.intersections_with(select_box_entity) {
                let selected = {
                    if e1 == select_box_entity { e2 } else { e1 }
                };
    
                commands.entity(selected).insert(Selected);
            }
        }
    }
}
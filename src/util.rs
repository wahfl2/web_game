use std::f32::consts::PI;

use bevy::{prelude::*, math::Vec3Swizzles, utils::HashMap, sprite::Mesh2dHandle};

pub const DEGREES: f32 = PI / 180.0;

pub type EntityQuery<'w, 's, T> = Query<'w, 's, Entity, With<T>>;

pub struct Cursor {
    pub pos: Vec2,
    pub prev_pos: Vec2,

    pub world_pos: Vec2,
    pub prev_world_pos: Vec2,

    pub delta: Vec2,

    pub moved: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self { 
            pos: Vec2::ZERO, 
            prev_pos: Vec2::ZERO,
            world_pos: Vec2::ZERO,
            prev_world_pos: Vec2::ZERO,
            delta: Vec2::ZERO,
            moved: false 
        }
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn cursor_pos(
    projection: Query<&OrthographicProjection>,
    c_transform_query: Query<&GlobalTransform, With<Camera>>,

    windows: Res<Windows>,
    mut cursor: ResMut<Cursor>,
    mut cursor_input: EventReader<CursorMoved>,
) {
    cursor.prev_pos = cursor.pos;
    cursor.prev_world_pos = cursor.world_pos;

    if let Some(cursor_moved) = cursor_input.iter().last() {        
        let window = windows.get_primary().unwrap();
        let window_size = Vec2::new(window.width(), window.height());

        let pos = cursor_moved.position - (window_size / 2.0);

        cursor.pos = pos;

        cursor.moved = true;
    } else {
        cursor.moved = false;
    }

    let scale = projection.single().scale;
    let cam_translation = c_transform_query.single().compute_transform().translation;
    cursor.world_pos = (cursor.pos * scale) + cam_translation.xy();

    cursor.delta = (cursor.prev_pos - cursor.pos) * scale;
}

pub struct ColorUpdate {
    pub selected: bool,
    pub hovered: bool,
    pub stickable: bool,
}

impl ColorUpdate {
    pub fn get_color(&self) -> Color {
        let mut ret = match self.stickable {
            true => Vec3::new(0.5, 0.5, 0.5),
            false => Vec3::new(0.25, 0.25, 0.25),
        };

        if self.selected {
            ret += 0.4;
        } else if self.hovered {
            ret += 0.1;
        }

        Color::rgb(ret.x, ret.y, ret.z)
    }
}

pub fn update_color_material(
    handle: &Handle<ColorMaterial>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    update: ColorUpdate,
) {
    let material_op = materials.get_mut(handle);
    if let Some(material) = material_op {

        material.color = update.get_color();
    }
}

pub struct ZipAll<I, T> {
    iter: I,
    zip_item: T,
}

impl<I, T> ZipAll<I, T> {
    pub fn new(iter: I, zip_item: T) -> Self {
        Self { iter, zip_item }
    }
}

impl<I, T> Iterator for ZipAll<I, T>
where 
    I: Iterator,
    T: Clone,
{
    type Item = (I::Item, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.iter.next() {
            Some((item, self.zip_item.clone()))
        } else {
            None
        }
    }
}

pub trait ZipAllTrait<T>: Iterator {
    fn zip_all(self, zip_item: T) -> ZipAll<Self, T>
    where 
        Self: Sized,
        T: Clone
    {
        ZipAll::new(self, zip_item)
    }
}

impl<T, I: Iterator> ZipAllTrait<T> for I {}

pub trait ExtraTransformMethods {
    fn from_pt_to_pt(p1: Vec3, p2: Vec3, width: f32) -> Self;
}

impl ExtraTransformMethods for Transform {
    fn from_pt_to_pt(p1: Vec3, p2: Vec3, width: f32) -> Self {
        Transform { 
            translation: (p1 + p2) * 0.5,
            rotation: Quat::from_rotation_arc(Vec3::Y, (p2 - p1).normalize()),
            scale: Vec3::new(width, p1.distance(p2), 1.0),
        }
    }
}

pub struct PreloadedAssets {
    pub materials: Vec<Handle<ColorMaterial>>,
    pub meshes: HashMap<&'static str, Mesh2dHandle>,
}

impl PreloadedAssets {
    pub fn new() -> Self {
        PreloadedAssets { materials: Vec::with_capacity(101), meshes: HashMap::new() }
    }

    pub fn get_bw_color_handle(&self, color: Color) -> &Handle<ColorMaterial> {
        let gray = (color.r() * 0.299) + (color.g() * 0.587) + (color.b() * 0.114);
        let i = (gray * 100.0).round() as usize;
        self.materials.get(i).unwrap()
    }
}

pub fn preload_assets(
    mut preloaded: ResMut<PreloadedAssets>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for i in 0..=100 {
        let f = i as f32 / 100.0;
        preloaded.materials.push(materials.add(ColorMaterial::from(Color::rgb(f, f, f))));
    }

    preloaded.meshes.insert(
        "box 1",
        meshes.add(shape::Box::new(1.0, 1.0, 0.0).into()).into()
    );

    preloaded.meshes.insert(
        "box 2",
        meshes.add(shape::Box::new(2.0, 2.0, 0.0).into()).into()
    );

    preloaded.meshes.insert(
        "circle 1",
        meshes.add(shape::Circle::new(1.0).into()).into()
    );

    preloaded.meshes.insert(
        "circle 4",
        meshes.add(shape::Circle::new(4.0).into()).into()
    );
}
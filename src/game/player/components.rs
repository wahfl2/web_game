use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub body: Entity,
    pub arm_r: Entity,
    pub arm_l: Entity,
    pub eyes: [Entity; 2],
    pub attached: Option<Attached>,
}

#[derive(Component, Debug)]
pub struct Attached {
    pub hit_point: Vec2,
    pub start_cursor_pos: Vec2,
    pub num_segments: u32,
    pub min_length: f32,
}

#[derive(Component)]
pub struct ShootingWeb {
    pub ray_norm: Vec2,
    pub ray_length: f32,
    pub max_length: f32,
    pub steps: u32,
}

#[derive(Deref, DerefMut)]
pub struct FailedShot(pub bool);

#[derive(Component)]
pub struct WebShotVisual;

#[derive(Component)]
pub struct WebPart;

#[derive(Component)]
pub struct WebPartConnection {
    pub e1: Entity,
    pub e2: Entity,
}

#[derive(Deref, DerefMut)]
pub struct FramesRestartKeyHeld(pub u32);

#[derive(Component)]
pub struct RespawnMessage;
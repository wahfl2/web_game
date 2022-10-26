use bevy::prelude::*;

use crate::editor::serde::editor_load;

use self::player::{spawn::{player_spawn, Respawn}, controls::{controls::{player_controls, WebPartEntities}, web_connections::update_web_connections}, camera::player_camera, components::*};

pub mod level;
pub mod player;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(editor_load)
            .insert_resource(Respawn(true))
            .add_system(player_spawn)
            .insert_resource(FailedShot(false))
            .insert_resource(WebPartEntities { entities: Vec::new() })
            .add_system(player_camera)
            .add_system(player_controls)
            .add_system_to_stage(CoreStage::PostUpdate, update_web_connections);
    }
}
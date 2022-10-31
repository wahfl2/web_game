use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::util::EntityQuery;

use super::components::*;

pub const MESSAGE_VELOCITY: f32 = 4000.0;

pub fn respawn_message(
    message: EntityQuery<RespawnMessage>,

    player_q: Query<&Player>,
    velocity_q: Query<&Velocity>,
    mut visibility_q: Query<&mut Visibility>,
) {
    if player_q.is_empty() { return; }
    let body = player_q.single().body;
    let velocity = velocity_q.get(body).unwrap();

    let mut vis = visibility_q.get_mut(message.single()).unwrap();
    if velocity.linvel.length() >= MESSAGE_VELOCITY {
        vis.is_visible = true;
    } else {
        vis.is_visible = false;
    }
}

pub fn spawn_message(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size {
                width: Val::Percent(100.0),
                ..default()
            },
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(100.0),
                ..default()
            },
            ..default()
        },
        color: UiColor(Color::NONE),
        ..default()
    }).insert(RespawnMessage)
    .insert_bundle(VisibilityBundle { 
        visibility: Visibility { is_visible: false }, 
        ..default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "Hold R to restart",
            TextStyle { 
                font: asset_server.load("fonts/FiraSans-Bold.ttf"), 
                font_size: 100.0, 
                color: Color::WHITE,
            }
        ));
    });
}
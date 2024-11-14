use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn sys_update_camera(_camera_query: Query<&mut OrthographicProjection, With<MainCamera>>) {}

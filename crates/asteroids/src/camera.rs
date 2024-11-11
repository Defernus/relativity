use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource)]
pub struct CameraSettings {
    pub scale: f32,
}

pub fn sys_update_camera(
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    settings: Res<CameraSettings>,
) {
    let window = window_query.single();
    let height = window.height();

    let mut camera = camera_query.single_mut();

    camera.scale = settings.scale / height;
}

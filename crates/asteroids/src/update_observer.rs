use bevy::math::DVec3;
use bevy::prelude::*;

/// observer data
#[derive(Debug, Component)]
pub struct ObserverData {
    pub proper_time: f64,
    /// Velocity of the observer in main reference frame.
    pub velocity: DVec3,
    pub pos: DVec3,
}

pub fn sys_update_observer(
    mut observer_query: Query<(&mut ObserverData, &Children)>,
    mut text: Query<&mut Text>,
    time: Res<Time>,
) {
    let (mut observer, children) = observer_query.single_mut();

    let dt = time.delta_seconds_f64();
    observer.proper_time += dt;

    observer.pos = observer.pos + observer.velocity * dt;

    for child in children.iter() {
        if let Ok(mut text) = text.get_mut(*child) {
            text.sections[0].value = format!("{:.3}", observer.proper_time);
        }
    }
}

pub fn sys_control_observer(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut observer_query: Query<&mut ObserverData>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds_f64();

    let mut velocity_dir = DVec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity_dir += DVec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        velocity_dir += DVec3::new(0.0, -1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity_dir += DVec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity_dir += DVec3::new(1.0, 0.0, 0.0);
    }

    if velocity_dir == DVec3::ZERO {
        return;
    }

    let mut observer = observer_query.single_mut();

    observer.velocity += velocity_dir * dt;
}

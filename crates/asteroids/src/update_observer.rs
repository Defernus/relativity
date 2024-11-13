use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use relativity::{lorentz_factor_from_vel, velocity_to_new_rf, SpacetimeEvent};

/// observer data
#[derive(Debug, Component)]
pub struct ObserverData {
    pub proper_time: f64,
    /// Velocity of the observer in main reference frame.
    pub velocity: DVec3,
    /// Current position in main reference frame.
    pub coord: SpacetimeEvent,
}

pub fn sys_update_observer(
    mut observer_query: Query<(&mut ObserverData, &Children)>,
    mut text: Query<&mut Text>,
    time: Res<Time>,
    settings: Res<RelativeSettings>,
) {
    let (mut observer, children) = observer_query.single_mut();

    let proper_time_delta = time.delta_seconds_f64();

    let c = settings.speed_of_light;
    let gamma = lorentz_factor_from_vel(observer.velocity, c);

    observer.proper_time += proper_time_delta;

    let time_delta = proper_time_delta * gamma;
    observer.coord = SpacetimeEvent {
        pos: observer.coord.pos + observer.velocity * time_delta,
        time: observer.coord.time + time_delta,
    };

    for child in children.iter() {
        if let Ok(mut text) = text.get_mut(*child) {
            text.sections[0].value = format!(
                "\
                t={:.3}\n\
                v={:.4}",
                observer.proper_time,
                observer.velocity.length(),
            );
        }
    }
}

pub fn sys_control_observer(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut observer_query: Query<&mut ObserverData>,
    time: Res<Time>,
    settings: Res<RelativeSettings>,
) {
    let dt = time.delta_seconds_f64();
    let c = settings.speed_of_light;

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

    let delta_velocity = velocity_dir.normalize() * dt;

    observer.velocity = velocity_to_new_rf(-observer.velocity, delta_velocity, c);
}

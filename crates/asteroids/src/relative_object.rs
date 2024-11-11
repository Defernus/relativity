use crate::*;
use bevy::math::DVec3;
use bevy::prelude::*;
use nonempty::NonEmpty;
use relativity::{lorentz_factor_from_vel, velocity_to_new_rf, SpacetimeEvent};

#[derive(Debug, Component)]
pub struct RelativeObject {
    pub id: u32,
    /// Last event data in current observer reference frame.
    pub world_line: NonEmpty<ObjectRFChange>,
}

impl RelativeObject {
    pub fn find_last_event(&self, time: f64, velocity: DVec3, c: f64) -> &ObjectRFChange {
        for event in self.world_line.iter().rev() {
            let coord = event.coord.to_reference_frame(velocity, c);

            if coord.time <= time {
                return event;
            }
        }

        self.world_line.first()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ObjectRFChange {
    /// Space time coordinate of the object in main reference frame.
    pub coord: SpacetimeEvent,

    /// Velocity of the object in main reference frame.
    pub velocity: DVec3,

    pub object_proper_time: f64,
}

impl RelativeObject {
    pub fn new(id: u32, pos: DVec3, velocity: DVec3) -> Self {
        Self {
            id,
            world_line: NonEmpty::new(ObjectRFChange {
                coord: SpacetimeEvent::new(pos),
                velocity,
                object_proper_time: 0.0,
            }),
        }
    }
}

#[derive(Debug, Resource)]
pub struct RelativeSettings {
    pub speed_of_light: f64,
}

pub fn sys_update_relative_objects(
    mut query: Query<(&RelativeObject, &mut Transform, &Children)>,
    mut text_query: Query<&mut Text>,
    settings: Res<RelativeSettings>,
    observer_query: Query<&ObserverData>,
) {
    let c = settings.speed_of_light;
    let observer = observer_query.single();

    for (object, mut transform, children) in query.iter_mut() {
        let last_event = object.find_last_event(observer.proper_time, observer.velocity, c);

        let relative_coord = last_event.coord.to_reference_frame(observer.velocity, c);
        let relative_velocity = velocity_to_new_rf(observer.velocity, last_event.velocity, c);

        let observer_delta_time = observer.proper_time - relative_coord.time;

        let offset_pos = observer.pos - observer.velocity * observer_delta_time;

        let new_pos = relative_coord.pos + relative_velocity * observer_delta_time - offset_pos;

        transform.translation.x = new_pos.x as f32;
        transform.translation.y = new_pos.y as f32;
        transform.translation.z = new_pos.z as f32;

        let object_gamma = lorentz_factor_from_vel(last_event.velocity, c);
        let observer_gamma = lorentz_factor_from_vel(observer.velocity, c);

        let proper_time =
            last_event.object_proper_time + observer_delta_time * observer_gamma / object_gamma;

        for child in children.iter() {
            if let Ok(mut text) = text_query.get_mut(*child) {
                text.sections[0].value = format!("[{}] {:.3}", object.id, proper_time);
            }
        }
    }
}

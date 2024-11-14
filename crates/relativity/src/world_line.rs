use crate::*;
use bevy::math::DVec3;
use nonempty::NonEmpty;

/// Event in object's world line.
#[derive(Debug, Copy, Clone)]
pub struct WorldLineEvent {
    /// Space time coordinate of the object in main reference frame.
    pub coord: SpacetimeEvent,

    /// Velocity of the object in main reference frame.
    pub velocity: DVec3,

    /// Proper time of the object when this event happened.
    pub object_proper_time: f64,
}

#[derive(Debug, Clone)]
pub struct WorldLine {
    pub events: NonEmpty<WorldLineEvent>,
}

impl WorldLine {
    pub fn new(init_event: WorldLineEvent) -> Self {
        Self {
            events: NonEmpty::new(init_event),
        }
    }

    /// Find event from the world lint right before the given `event_position` in the reference
    /// frame with the given `velocity`.
    ///
    /// # Parameters
    /// - `event_position`: Position of the event in the reference frame of the world line.
    /// - `rf_velocity`: Velocity relative to the reference frame of the world line.
    /// - `c`: Speed of light.
    pub fn find_last_event(
        &self,
        event_position: SpacetimeEvent,
        rf_velocity: DVec3,
        c: f64,
    ) -> Option<&WorldLineEvent> {
        for event in self.events.iter().rev() {
            let coord = event.coord - event_position;
            let coord = coord.to_reference_frame(rf_velocity, c);

            if coord.time <= 0.0 {
                return Some(event);
            }
        }

        None
    }

    /// Get simultaneous event from the world line to the given `event_position` in the reference
    /// frame with the given `velocity`.
    ///
    /// # Parameters
    /// - `event_position`: Position of the event in the reference frame of the world line.
    /// - `rf_velocity`: Velocity relative to the reference frame of the world line.
    /// - `c`: Speed of light.
    pub fn get_current_event(
        &self,
        event_position: SpacetimeEvent,
        rf_velocity: DVec3,
        c: f64,
    ) -> Option<SimultaneousWorldLineEvent> {
        let last_event = self.find_last_event(event_position, rf_velocity, c)?;

        let relative_coord = (last_event.coord - event_position).to_reference_frame(rf_velocity, c);
        let relative_velocity = velocity_to_new_rf(rf_velocity, last_event.velocity, c);

        let observer_delta_time = -relative_coord.time;

        let relative_position = relative_coord.pos + relative_velocity * observer_delta_time;

        // gamma factors relative to main reference frame
        let object_gamma = lorentz_factor_from_vel(last_event.velocity, c);
        let observer_gamma = lorentz_factor_from_vel(rf_velocity, c);

        let object_proper_time =
            last_event.object_proper_time + observer_delta_time * observer_gamma / object_gamma;

        Some(SimultaneousWorldLineEvent {
            relative_position,
            proper_time: object_proper_time,
            velocity: last_event.velocity,
            relative_velocity,
            coord: last_event.coord,
        })
    }

    /// Find last event from the world line which is inside the observer's past light cone.
    ///
    /// Returns `None` if the world line start is outside the observer's past light cone.
    pub fn find_last_visible_event(
        &self,
        event_position: SpacetimeEvent,
        c: f64,
    ) -> Option<&WorldLineEvent> {
        let mut prev_event = self.events.first();

        if !event_position.light_cone(c).past_contains(prev_event.coord) {
            // world line start is outside the observer's past light cone and can't be seen yet
            return None;
        }

        for next_event in self.events.iter().skip(1) {
            if !event_position.light_cone(c).past_contains(next_event.coord) {
                // if next event is outside past light cone then the trajectory between prev_event
                // and next_event intersects with the observer's past light cone
                break;
            }

            prev_event = next_event;
        }

        Some(prev_event)
    }

    /// Same as [`WorldLine::get_current_event`] but return event on world line which intersects
    /// with the observer's past light cone.
    ///
    /// # Parameters
    /// - `event_position`: Position of the event in the reference frame of the world line.
    /// - `rf_velocity`: Velocity relative to the reference frame of the world line.
    /// - `c`: Speed of light.
    pub fn get_visible_event(
        &self,
        event_position: SpacetimeEvent,
        rf_velocity: DVec3,
        c: f64,
    ) -> Option<VisibleWorldLineEvent> {
        let last_visible_event = self.find_last_visible_event(event_position, c)?;

        let relative_velocity = velocity_to_new_rf(rf_velocity, last_visible_event.velocity, c);

        // last visible event position in observer's reference frame
        let relative_last_event =
            (last_visible_event.coord - event_position).to_reference_frame(rf_velocity, c);

        // find time of first light cone intersection
        let t_intersect = calc_intersection_time(relative_last_event, relative_velocity, c);
        let Some(t_intersect) = t_intersect.into_iter().min_by(|a, b| a.total_cmp(b)) else {
            // TODO add proper warnings
            println!("!!!No intersection found!!! coord={relative_last_event:?}; vel={relative_velocity:?}");
            return None;
        };
        if t_intersect > 0.0 {
            // TODO add proper warnings
            println!("!!!Intersection is in the future!!! {t_intersect:?}");
            return None;
        }

        let delta_time = t_intersect - relative_last_event.time;
        let pos_intersect = relative_last_event.pos + relative_velocity * delta_time;

        let intersect_event = SpacetimeEvent::new(pos_intersect).with_time(t_intersect);

        let relative_gamma = lorentz_factor_from_vel(relative_velocity, c);
        let delta_proper_time = delta_time / relative_gamma;

        Some(VisibleWorldLineEvent {
            relative_coord: intersect_event,
            proper_time: last_visible_event.object_proper_time + delta_proper_time,
            velocity: last_visible_event.velocity,
            relative_velocity,
        })
    }
}

fn calc_intersection_time(cord: SpacetimeEvent, vel: DVec3, c: f64) -> Vec<f64> {
    let vx_sq = vel.x * vel.x;
    let vy_sq = vel.y * vel.y;
    let vz_sq = vel.z * vel.z;

    let v_sq = vx_sq + vy_sq + vz_sq;

    let a = c * c - v_sq;

    let b = -2.0
        * ((cord.pos.x * vel.x - vx_sq * cord.time)
            + (cord.pos.y * vel.y - vy_sq * cord.time)
            + (cord.pos.z * vel.z - vz_sq * cord.time));

    let c = -((cord.pos.x * cord.pos.x - 2.0 * cord.pos.x * vel.x * cord.time
        + vx_sq * cord.time * cord.time)
        + (cord.pos.y * cord.pos.y - 2.0 * cord.pos.y * vel.y * cord.time
            + vy_sq * cord.time * cord.time)
        + (cord.pos.z * cord.pos.z - 2.0 * cord.pos.z * vel.z * cord.time
            + vz_sq * cord.time * cord.time));

    let double_a = 2.0 * a;
    let discriminant = b * b - 2.0 * double_a * c;

    if discriminant < 0.0 {
        return vec![];
    }

    if discriminant == 0.0 {
        let t = -b / double_a;
        return vec![t];
    }

    let d_sqrt = discriminant.sqrt();

    vec![(-b + d_sqrt) / double_a, (-b - d_sqrt) / double_a]
}

/// World line event simultaneous to the observer's event in the observer's reference frame.
#[derive(Debug, Clone, Copy)]
pub struct SimultaneousWorldLineEvent {
    /// Position of the event in observer's reference frame.
    pub relative_position: DVec3,
    /// Velocity of the object in reference frame of the world line.
    pub velocity: DVec3,
    /// Velocity of the object in reference frame of the observer.
    pub relative_velocity: DVec3,
    /// Proper time of the object when this event happened.
    pub proper_time: f64,
    /// Position of the event in world line's reference frame.
    pub coord: SpacetimeEvent,
}

/// World line event visible to the observer.
#[derive(Debug, Clone, Copy)]
pub struct VisibleWorldLineEvent {
    /// Position of the event in observer's reference frame relative to the observer's event.
    pub relative_coord: SpacetimeEvent,
    /// Velocity of the object in reference frame of the world line.
    pub velocity: DVec3,
    /// Velocity of the object in reference frame of the observer.
    pub relative_velocity: DVec3,
    /// Proper time of the object when this event happened.
    pub proper_time: f64,
}

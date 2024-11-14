use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct LightCone {
    pub origin: SpacetimeEvent,
    pub light_speed: f64,
}

impl LightCone {
    pub fn new(origin: SpacetimeEvent, light_speed: f64) -> Self {
        Self {
            origin,
            light_speed,
        }
    }

    pub fn future_contains(self, event: SpacetimeEvent) -> bool {
        event.time > self.origin.time
            && event.get_separation(self.origin, self.light_speed) == SpacetimeSeparation::Timelike
    }

    pub fn past_contains(self, event: SpacetimeEvent) -> bool {
        event.time < self.origin.time
            && event.get_separation(self.origin, self.light_speed) == SpacetimeSeparation::Timelike
    }

    pub fn contains(self, event: SpacetimeEvent) -> bool {
        event.get_separation(self.origin, self.light_speed) == SpacetimeSeparation::Timelike
    }
}

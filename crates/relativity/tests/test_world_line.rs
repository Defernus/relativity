use bevy::math::DVec3;
use relativity::{SpacetimeEvent, WorldLine, WorldLineEvent};

#[test]
fn test_world_line() {
    let c = 1.0;

    let world_line = WorldLine::new(WorldLineEvent {
        velocity: DVec3::new(0.5 * c, 0.0, 0.0),
        coord: SpacetimeEvent::new(DVec3::new(2.0, 0.0, 0.0)),
        object_proper_time: 0.0,
    });

    let observer_velocity = DVec3::new(0.0, 0.0, 0.0);
    let observer_coord = SpacetimeEvent::new(DVec3::new(0.0, 0.0, 0.0)).with_time(4.0);

    let visible_event = world_line
        .get_visible_event(observer_coord, observer_velocity, c)
        .expect("visible event");

    let invariant = (visible_event.relative_coord.time * c).powi(2)
        - visible_event.relative_coord.pos.length_squared();
    assert!(
        invariant.abs() < 1e-8,
        "intersection event invariant should be zero, as it lies on the light cone",
    );
}

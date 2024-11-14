use bevy::math::DVec3;
use relativity::{SpacetimeEvent, SpacetimeSeparation};

#[test]
fn test_light_cone_timelike_separation() {
    let c = 1.0;

    let event_0 = SpacetimeEvent {
        pos: DVec3::new(0.0, 0.0, 0.0),
        time: 0.0,
    };

    let event_1 = SpacetimeEvent {
        pos: DVec3::new(0.0, 0.0, 0.0),
        time: 1.0,
    };

    assert_eq!(
        event_0
            .spacetime_interval_squared(event_1, c)
            .partial_cmp(&0.0),
        Some(std::cmp::Ordering::Greater)
    );

    assert_eq!(
        event_0.get_separation(event_1, c),
        SpacetimeSeparation::Timelike
    );

    assert!(event_1.light_cone(c).past_contains(event_0));
    assert!(!event_0.light_cone(c).past_contains(event_1));

    assert!(!event_1.light_cone(c).future_contains(event_0));
    assert!(event_0.light_cone(c).future_contains(event_1));
}

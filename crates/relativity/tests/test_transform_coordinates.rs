use bevy::math::DVec3;
use relativity::*;

const EPSILON: f64 = 1e-10;

#[test]
fn test_transform_coordinates_target_position_is_zero_in_target_frame() {
    let c = 1.0;
    let velocity_len = 0.5 * c;
    let velocity = DVec3::new(1.0, 2.0, 0.0).normalize() * velocity_len;
    let time = 10.0;

    let transformed_event = SpacetimeEvent {
        pos: velocity * time,
        time,
    }
    .to_reference_frame(velocity, c);

    let rev_gamma = (1.0 - velocity_len.powi(2) / c.powi(2)).sqrt();
    let expected_time = time * rev_gamma;

    assert!(transformed_event.pos.length_squared() < EPSILON);
    assert!((transformed_event.time - expected_time).abs() < EPSILON);
}

use crate::lorentz_factor_from_vel;
use bevy::math::DVec3;

const EPSILON: f64 = 1e-10;

/// Transform velocity from old reference frame(rf) to new reference frame.
///
/// # Parameters
/// - `rf_delta_v`: Velocity change of new reference frame relative to old reference frame (in old
///   reference frame).
/// - `v`: Velocity of the object in old reference frame.
///
/// # Returns
/// Velocity of the object in new reference frame.
pub fn velocity_to_new_rf(rf_delta_v: DVec3, v: DVec3, c: f64) -> DVec3 {
    if rf_delta_v.length_squared() < EPSILON {
        return v;
    }

    let c_sq = c.powi(2);

    let dv_unit = rf_delta_v.normalize();

    let dv_par = v.dot(dv_unit) * dv_unit;
    let dv_perp = v - dv_par;

    let gamma = lorentz_factor_from_vel(rf_delta_v, c);

    let new_v_par = (dv_par - rf_delta_v) / (1.0 - dv_par.dot(rf_delta_v) / c_sq);
    let new_v_perp = dv_perp / (gamma * (1.0 - dv_par.dot(rf_delta_v) / c_sq));

    new_v_par + new_v_perp
}

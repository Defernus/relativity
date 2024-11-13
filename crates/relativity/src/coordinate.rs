use bevy::math::{DMat4, DVec3, DVec4};

#[inline(always)]
pub fn lorentz_factor(beta_squared: f64) -> f64 {
    1.0 / (1.0 - beta_squared).sqrt()
}

#[inline(always)]
pub fn lorentz_factor_from_vel(v: DVec3, c: f64) -> f64 {
    let beta = v / c;
    lorentz_factor(beta.length_squared())
}

#[inline(always)]
pub fn time_to_reference_frame(delta_time: f64, gamma: f64) -> f64 {
    delta_time / gamma
}

#[inline(always)]
pub fn lorentz_transform_matrix(beta: DVec3) -> DMat4 {
    let gamma = lorentz_factor(beta.length_squared());
    let squared_gamma = gamma.powi(2);
    let gamma_plus_one = gamma + 1.0;

    DMat4::from_cols(
        DVec4::new(gamma, -beta.x * gamma, -beta.y * gamma, -beta.z * gamma),
        DVec4::new(
            -beta.x * gamma,
            1.0 + squared_gamma * beta.x.powi(2) / gamma_plus_one,
            squared_gamma * beta.x * beta.y / gamma_plus_one,
            squared_gamma * beta.x * beta.z / gamma_plus_one,
        ),
        DVec4::new(
            -beta.y * gamma,
            squared_gamma * beta.y * beta.x / gamma_plus_one,
            1.0 + squared_gamma * beta.y.powi(2) / gamma_plus_one,
            squared_gamma * beta.y * beta.z / gamma_plus_one,
        ),
        DVec4::new(
            -beta.z * gamma,
            squared_gamma * beta.z * beta.x / gamma_plus_one,
            squared_gamma * beta.z * beta.y / gamma_plus_one,
            1.0 + squared_gamma * beta.z.powi(2) / gamma_plus_one,
        ),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacetimeEvent {
    pub pos: DVec3,
    pub time: f64,
}

impl SpacetimeEvent {
    pub const ZERO: Self = Self {
        pos: DVec3::ZERO,
        time: 0.0,
    };

    #[inline(always)]
    pub fn new(pos: DVec3) -> Self {
        Self { pos, time: 0.0 }
    }

    #[inline(always)]
    pub fn with_time(mut self, time: f64) -> Self {
        self.time = time;

        self
    }

    pub fn to_reference_frame(self, v: DVec3, c: f64) -> Self {
        let beta = v / c;

        let matrix = lorentz_transform_matrix(beta);

        let vec = DVec4::new(self.time * c, self.pos.x, self.pos.y, self.pos.z);

        let transformed = matrix * vec;

        Self {
            pos: DVec3::new(transformed.y, transformed.z, transformed.w),
            time: transformed.x / c,
        }
    }
}

impl std::ops::Sub for SpacetimeEvent {
    type Output = SpacetimeEvent;

    fn sub(self, rhs: Self) -> Self::Output {
        SpacetimeEvent {
            pos: self.pos - rhs.pos,
            time: self.time - rhs.time,
        }
    }
}

impl std::ops::Add for SpacetimeEvent {
    type Output = SpacetimeEvent;

    fn add(self, rhs: Self) -> Self::Output {
        SpacetimeEvent {
            pos: self.pos + rhs.pos,
            time: self.time + rhs.time,
        }
    }
}

impl std::ops::AddAssign for SpacetimeEvent {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for SpacetimeEvent {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

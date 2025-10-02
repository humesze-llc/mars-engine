use glam::{self, Mat4 as GMat4, Vec2 as GVec2, Vec3 as GVec3};
use std::ops::{Add, Sub, Mul, Div};

macro_rules! impl_vec_ops {
    ($T:ty, $Inner:ty) => {
        impl Add for $T {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self(self.0 + rhs.0)
            }
        }
        impl Sub for $T {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                Self(self.0 - rhs.0)
            }
        }
        impl Mul<f32> for $T {
            type Output = Self;
            fn mul(self, rhs: f32) -> Self {
                Self(self.0 * rhs)
            }
        }
        impl Div<f32> for $T {
            type Output = Self;
            fn div(self, rhs: f32) -> Self {
                Self(self.0 / rhs)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2(pub GVec2);

impl Vec2 {
    pub const ZERO: Self = Self(GVec2::ZERO);
    pub const ONE: Self = Self(GVec2::ONE);
    pub const X: Self = Self(GVec2::X);
    pub const Y: Self = Self(GVec2::Y);

    pub fn new(x: f32, y: f32) -> Self {
        Self(GVec2::new(x, y))
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.0.dot(rhs.0)
    }

    pub fn length(self) -> f32 {
        self.0.length()
    }

    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }
}
impl_vec_ops!(Vec2, GVec2);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub GVec3);

impl Vec3 {
    pub const ZERO: Self = Self(GVec3::ZERO);
    pub const ONE: Self = Self(GVec3::ONE);
    pub const X: Self = Self(GVec3::X);
    pub const Y: Self = Self(GVec3::Y);
    pub const Z: Self = Self(GVec3::Z);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(GVec3::new(x, y, z))
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.0.dot(rhs.0)
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(self.0.cross(rhs.0))
    }

    pub fn length(self) -> f32 {
        self.0.length()
    }

    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }
}
impl_vec_ops!(Vec3, GVec3);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4(pub GMat4);

impl Mat4 {
    pub const IDENTITY: Self = Self(GMat4::IDENTITY);

    pub fn perspective_rh_gl(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self(GMat4::perspective_rh_gl(fov_y, aspect, near, far))
    }

    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Self(GMat4::look_at_rh(eye.0, center.0, up.0))
    }

    pub fn inverse(self) -> Self {
        Self(self.0.inverse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_addition() {
        assert_eq!(Vec3::X + Vec3::Y, Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn vec3_dot_and_cross() {
        assert_eq!(Vec3::X.dot(Vec3::Y), 0.0);
        assert_eq!(Vec3::X.cross(Vec3::Y), Vec3::Z);
    }

    #[test]
    fn vec3_normalization() {
        let v = Vec3::new(3.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Vec3::X);
    }

    #[test]
    fn mat4_identity_inverse() {
        assert_eq!(Mat4::IDENTITY.inverse(), Mat4::IDENTITY);
    }
}
use super::{IVector2, Vector2};
use glam::UVec2;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// u32

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct UVector2 {
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for UVector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UVector2({}, {})", self.x, self.y)
    }
}

impl UVector2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const ONE: Self = Self { x: 1, y: 1 };

    #[inline]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn to_array(self) -> [u32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn to_tuple(self) -> (u32, u32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn as_vector2(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    // Converts to `IVector2` by clamping values above `i32::MAX`.
    #[inline]
    pub fn as_ivector2_saturating(self) -> IVector2 {
        IVector2::new(u32_to_i32_saturating(self.x), u32_to_i32_saturating(self.y))
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> u64 {
        self.x as u64 * rhs.x as u64 + self.y as u64 * rhs.y as u64
    }

    #[inline]
    pub fn length_squared(self) -> u64 {
        self.dot(self)
    }

    // Component-wise minimum
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    // Component-wise maximum
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }

    // Component-wise clamp
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    // Returns a wrapping-negated copy
    #[inline]
    pub fn negated(self) -> Self {
        Self::new(self.x.wrapping_neg(), self.y.wrapping_neg())
    }

    // Wrapping-negates this vector in place
    #[inline]
    pub fn negate(&mut self) -> &mut Self {
        *self = self.negated();
        self
    }

    // Returns a copy stepped toward `to` by at most `step` per component
    #[inline]
    pub fn stepped(self, to: Self, step: u32) -> Self {
        Self::new(
            step_u32_toward(self.x, to.x, step),
            step_u32_toward(self.y, to.y, step),
        )
    }

    // Steps this vector toward `to` by at most `step` per component in place
    #[inline]
    pub fn step(&mut self, to: Self, step: u32) -> &mut Self {
        *self = self.stepped(to, step);
        self
    }
}

impl From<UVector2> for UVec2 {
    #[inline]
    fn from(v: UVector2) -> Self {
        UVec2::new(v.x, v.y)
    }
}

impl From<UVec2> for UVector2 {
    #[inline]
    fn from(v: UVec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<[u32; 2]> for UVector2 {
    #[inline]
    fn from(v: [u32; 2]) -> Self {
        Self::new(v[0], v[1])
    }
}

impl From<UVector2> for [u32; 2] {
    #[inline]
    fn from(v: UVector2) -> Self {
        v.to_array()
    }
}

impl From<(u32, u32)> for UVector2 {
    #[inline]
    fn from(v: (u32, u32)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<UVector2> for (u32, u32) {
    #[inline]
    fn from(v: UVector2) -> Self {
        v.to_tuple()
    }
}

impl From<UVector2> for Vector2 {
    #[inline]
    fn from(v: UVector2) -> Self {
        v.as_vector2()
    }
}

impl From<Vector2> for UVector2 {
    #[inline]
    fn from(v: Vector2) -> Self {
        v.as_uvector2_saturating()
    }
}

impl Add for UVector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[inline]
fn step_u32_toward(current: u32, target: u32, step: u32) -> u32 {
    if current < target {
        current.saturating_add(step).min(target)
    } else if current > target {
        current.saturating_sub(step).max(target)
    } else {
        current
    }
}

#[inline]
fn u32_to_i32_saturating(value: u32) -> i32 {
    value.min(i32::MAX as u32) as i32
}

impl AddAssign for UVector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for UVector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for UVector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for UVector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for UVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<u32> for UVector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<u32> for UVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for UVector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for UVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<u32> for UVector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<u32> for UVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

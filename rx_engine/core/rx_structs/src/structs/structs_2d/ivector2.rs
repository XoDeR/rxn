use super::{UVector2, Vector2};
use glam::IVec2;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// i32

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct IVector2 {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for IVector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IVector2({}, {})", self.x, self.y)
    }
}

impl IVector2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const ONE: Self = Self { x: 1, y: 1 };

    pub const NEG_ONE: Self = Self { x: -1, y: -1 };

    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn to_array(self) -> [i32; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn to_tuple(self) -> (i32, i32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn as_vector2(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }

    // Converts to `UVector2` by clamping negative values to zero
    #[inline]
    pub fn as_uvector2_saturating(self) -> UVector2 {
        UVector2::new(i32_to_u32_saturating(self.x), i32_to_u32_saturating(self.y))
    }

    #[inline]
    pub fn dot(self, rhs: Self) -> i64 {
        self.x as i64 * rhs.x as i64 + self.y as i64 * rhs.y as i64
    }

    #[inline]
    pub fn length_squared(self) -> i64 {
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

    // Returns a negated copy
    #[inline]
    pub fn negated(self) -> Self {
        Self::new(self.x.saturating_neg(), self.y.saturating_neg())
    }

    // Negates this vector in place
    #[inline]
    pub fn negate(&mut self) -> &mut Self {
        *self = self.negated();
        self
    }

    // Component-wise absolute value, saturating `i32::MIN` to `i32::MAX`
    #[inline]
    pub fn abs(self) -> Self {
        Self::new(self.x.saturating_abs(), self.y.saturating_abs())
    }

    // Component-wise sign (-1, 0, or 1)
    #[inline]
    pub fn signum(self) -> Self {
        Self::new(self.x.signum(), self.y.signum())
    }

    // Returns a copy stepped toward `to` by at most `step` per component
    #[inline]
    pub fn stepped(self, to: Self, step: u32) -> Self {
        Self::new(
            step_i32_toward(self.x, to.x, step),
            step_i32_toward(self.y, to.y, step),
        )
    }

    // Steps this vector toward `to` by at most `step` per component in place
    #[inline]
    pub fn step(&mut self, to: Self, step: u32) -> &mut Self {
        *self = self.stepped(to, step);
        self
    }
}

impl From<IVector2> for IVec2 {
    #[inline]
    fn from(v: IVector2) -> Self {
        IVec2::new(v.x, v.y)
    }
}

impl From<IVec2> for IVector2 {
    #[inline]
    fn from(v: IVec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<[i32; 2]> for IVector2 {
    #[inline]
    fn from(v: [i32; 2]) -> Self {
        Self::new(v[0], v[1])
    }
}

impl From<IVector2> for [i32; 2] {
    #[inline]
    fn from(v: IVector2) -> Self {
        v.to_array()
    }
}

impl From<(i32, i32)> for IVector2 {
    #[inline]
    fn from(v: (i32, i32)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<IVector2> for (i32, i32) {
    #[inline]
    fn from(v: IVector2) -> Self {
        v.to_tuple()
    }
}

impl From<IVector2> for Vector2 {
    #[inline]
    fn from(v: IVector2) -> Self {
        v.as_vector2()
    }
}

impl From<Vector2> for IVector2 {
    #[inline]
    fn from(v: Vector2) -> Self {
        v.as_ivector2_saturating()
    }
}

impl From<IVector2> for UVector2 {
    #[inline]
    fn from(v: IVector2) -> Self {
        v.as_uvector2_saturating()
    }
}

impl From<UVector2> for IVector2 {
    #[inline]
    fn from(v: UVector2) -> Self {
        v.as_ivector2_saturating()
    }
}

impl Add for IVector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for IVector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for IVector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for IVector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for IVector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for IVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<i32> for IVector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for IVector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for IVector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl DivAssign for IVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<i32> for IVector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for IVector2 {
    #[inline]
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for IVector2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

#[inline]
fn i32_to_u32_saturating(value: i32) -> u32 {
    if value <= 0 { 0 } else { value as u32 }
}

#[inline]
fn step_i32_toward(current: i32, target: i32, step: u32) -> i32 {
    if current < target {
        current
            .saturating_add(step.min(i32::MAX as u32) as i32)
            .min(target)
    } else if current > target {
        current
            .saturating_sub(step.min(i32::MAX as u32) as i32)
            .max(target)
    } else {
        current
    }
}

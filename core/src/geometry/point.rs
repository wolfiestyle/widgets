use crate::geometry::{Rect, Size};
use num_traits::{AsPrimitive, Float, NumCast, PrimInt};
use std::ops;

/// Defines a position in 2D cartesian coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Point<T> {
    /// Distance from the left edge.
    pub x: T,
    /// Distance from the top edge.
    pub y: T,
}

impl<T> Point<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    #[inline]
    pub fn with_x(self, x: T) -> Self {
        Point { x, y: self.y }
    }

    #[inline]
    pub fn with_y(self, y: T) -> Self {
        Point { x: self.x, y }
    }

    #[inline]
    pub fn offset(self, dx: T, dy: T) -> Self
    where
        T: ops::Add<Output = T>,
    {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    #[inline]
    pub fn inside(self, rect: Rect) -> bool
    where
        T: NumCast,
    {
        if let Some(p) = self.cast_checked::<i32>() {
            p.x >= rect.x() && p.x <= rect.end_x() && p.y >= rect.y() && p.y <= rect.end_y()
        } else {
            false
        }
    }

    #[inline]
    pub fn map<F, R>(self, mut f: F) -> Point<R>
    where
        F: FnMut(T) -> R,
    {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }

    #[inline]
    pub fn map2<F, U, R>(self, other: Point<U>, mut f: F) -> Point<R>
    where
        F: FnMut(T, U) -> R,
    {
        Point {
            x: f(self.x, other.x),
            y: f(self.y, other.y),
        }
    }

    #[inline]
    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        f(&mut self.x);
        f(&mut self.y);
    }

    #[inline]
    pub fn map2_mut<F, U>(&mut self, other: Point<U>, mut f: F)
    where
        F: FnMut(&mut T, U),
    {
        f(&mut self.x, other.x);
        f(&mut self.y, other.y);
    }

    #[inline]
    pub fn cast<R>(self) -> Point<R>
    where
        T: AsPrimitive<R>,
        R: Copy + 'static,
    {
        Point {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }

    #[inline]
    pub fn cast_checked<R>(self) -> Option<Point<R>>
    where
        T: NumCast,
        R: NumCast,
    {
        Some(Point {
            x: num_traits::cast(self.x)?,
            y: num_traits::cast(self.y)?,
        })
    }
}

impl<T: PrimInt + AsPrimitive<u32>> Point<T> {
    #[inline]
    pub fn as_size(self) -> Size {
        Size {
            w: self.x.max(T::zero()).as_(),
            h: self.y.max(T::zero()).as_(),
        }
    }
}

impl<T: Float> Point<T> {
    /// Creates a new point from radial coordinates.
    #[inline]
    pub fn new_radial(radius: T, angle: T) -> Self {
        Point {
            x: radius * angle.cos(),
            y: radius * angle.sin(),
        }
    }

    /// Calculates the distance between two points.
    #[inline]
    pub fn distance_to(self, other: Self) -> T {
        let d = other - self;
        d.x.hypot(d.y)
    }

    /// Rotate this point around the origin.
    #[inline]
    pub fn rotate_origin(self, angle: T) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Point {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Rotate this point around another point.
    #[inline]
    pub fn rotate(self, center: Self, angle: T) -> Self {
        (self - center).rotate_origin(angle) + center
    }

    /// Interpolate between two points.
    #[inline]
    pub fn interpolate(self, other: Self, a: T) -> Self {
        self * (T::one() - a) + other * a
    }
}

impl<T: Copy + 'static, S: AsPrimitive<T>> From<[S; 2]> for Point<T> {
    #[inline]
    fn from([x, y]: [S; 2]) -> Self {
        Self { x: x.as_(), y: y.as_() }
    }
}

impl<T: Copy + 'static, S: AsPrimitive<T>> From<(S, S)> for Point<T> {
    #[inline]
    fn from((x, y): (S, S)) -> Self {
        Self { x: x.as_(), y: y.as_() }
    }
}

impl<T> From<Point<T>> for [T; 2] {
    #[inline]
    fn from(p: Point<T>) -> Self {
        [p.x, p.y]
    }
}

impl<T> From<Point<T>> for (T, T) {
    #[inline]
    fn from(p: Point<T>) -> Self {
        (p.x, p.y)
    }
}

impl<T: Default> From<()> for Point<T> {
    #[inline]
    fn from(_: ()) -> Self {
        Default::default()
    }
}

impl<T> ops::Add for Point<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Add::add)
    }
}

impl<T> ops::Sub for Point<T>
where
    T: ops::Sub<Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Sub::sub)
    }
}

impl<T> ops::Mul<T> for Point<T>
where
    T: ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.map(|a| a * rhs)
    }
}

impl<T> ops::Div<T> for Point<T>
where
    T: ops::Div<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.map(|a| a / rhs)
    }
}

impl<T> ops::Div for Point<T>
where
    T: ops::Div<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.map2(rhs, ops::Div::div)
    }
}

impl<T> ops::Rem<T> for Point<T>
where
    T: ops::Rem<Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: T) -> Self::Output {
        self.map(|a| a % rhs)
    }
}

impl<T> ops::AddAssign for Point<T>
where
    T: ops::AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, std::ops::AddAssign::add_assign)
    }
}

impl<T> ops::SubAssign for Point<T>
where
    T: ops::SubAssign,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.map2_mut(rhs, std::ops::SubAssign::sub_assign)
    }
}

impl<T> ops::MulAssign<T> for Point<T>
where
    T: ops::MulAssign + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.map_mut(|a| *a *= rhs);
    }
}

impl<T> ops::DivAssign<T> for Point<T>
where
    T: ops::DivAssign + Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.map_mut(|a| *a /= rhs);
    }
}

impl<T> ops::RemAssign<T> for Point<T>
where
    T: ops::RemAssign + Copy,
{
    #[inline]
    fn rem_assign(&mut self, rhs: T) {
        self.map_mut(|a| *a %= rhs);
    }
}

impl<T> ops::Neg for Point<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y }
    }
}

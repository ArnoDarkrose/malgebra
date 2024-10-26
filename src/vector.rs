use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::simd::prelude::*;

use paste::paste;

use crate::{One, Zero};

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Vec2 {
    buf: f32x2,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vec2::from_slice(&[x, y])
    }

    pub const fn from_slice(val: &[f32; 2]) -> Self {
        Vec2 {
            buf: f32x2::from_slice(val),
        }
    }

    pub const fn splat(val: f32) -> Self {
        Vec2::from_slice(&[val, val])
    }

    pub const fn as_array(&self) -> &[f32; 2] {
        self.buf.as_array()
    }

    pub const fn x(&self) -> f32 {
        self.as_array()[0]
    }

    pub const fn y(&self) -> f32 {
        self.as_array()[1]
    }

    pub const fn with_x(self, x: f32) -> Self {
        Self::new(x, self.y())
    }

    pub const fn with_y(self, y: f32) -> Self {
        Self::new(self.x(), y)
    }
}

macro_rules! impl_binop {
    ($(($name:ident, $op:tt));*) => {
        $(
            impl $name for Vec2 {
                type Output = Self;

                paste!{
                    fn [<$name:lower>] (self, rhs: Self) -> Self::Output {
                        let buf = self.buf $op rhs.buf;

                        Self {buf}
                    }
                }
            }

            impl $name for &Vec2 {
                type Output = Vec2;

                paste!{
                    fn [<$name:snake>] (self, rhs: Self) -> Self::Output {
                        let buf = self.buf $op rhs.buf;

                        Vec2 {buf}
                    }
                }
            }

            impl $name<f32> for Vec2 {
                type Output = Vec2;

                paste!{
                    fn [<$name:snake>] (self, rhs: f32) -> Self::Output {
                        let rhs = Self::splat(rhs);

                        self $op rhs
                    }
                }
            }

            impl $name<&f32> for &Vec2 {
                type Output = Vec2;

                paste!{
                    fn [<$name:snake>] (self, rhs: &f32) -> Self::Output {
                        let rhs = Vec2::splat(*rhs);

                        *self $op rhs
                    }
                }
            }

            impl $name<Vec2> for f32 {
                type Output = Vec2;

                paste! {
                    fn [<$name:snake>] (self, rhs: Vec2) -> Self::Output {
                        rhs $op self
                    }
                }
            }
        )*
    };
}

impl_binop! {(Mul, *); (Add, +); (Sub, -); (Div, /); (Rem, %)}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        let buf = -self.buf;

        Self { buf }
    }
}

macro_rules! impl_binop_assign {
    ($(($name:ident, $op:tt));*) => {
        $(
            impl $name for Vec2 {
                paste!{
                    fn [<$name:snake>] (&mut self, rhs: Self) {
                        self.buf $op rhs.buf;
                    }
                }
            }

            impl $name<f32> for Vec2 {
                paste!{
                    fn [<$name:snake>] (&mut self, rhs: f32) {
                        let rhs = f32x2::from_slice(&[rhs, rhs]);

                        self.buf $op rhs;
                    }
                }
            }
        )*
    }
}

impl_binop_assign! {(AddAssign, +=); (SubAssign, -=); (MulAssign, *=); (DivAssign, /=); (RemAssign, %=)}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array()[index]
    }
}

impl Product for Vec2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec2>,
    {
        iter.fold(Vec2::ZERO, |acc, cur| acc * cur)
    }
}

impl Sum for Vec2 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec2>,
    {
        iter.fold(Vec2::ZERO, |acc, cur| acc + cur)
    }
}

impl Zero for Vec2 {
    const ZERO: Self = Vec2::splat(0.0);
}

impl One for Vec2 {
    const ONE: Self = Vec2::splat(1.0);
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    #[test]
    fn test_mul() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        let c = a * b;

        let d = f32x2::from_slice(&[5.0, 6.0]);

        let e = f32x2::from_slice(&[7.0, 8.0]);

        let g = f32x2::from_slice(&[1.0, 3.0]);

        let h = 4.0 + Vec2::new(1.0, 3.0);

        dbg!(h);
    }
}

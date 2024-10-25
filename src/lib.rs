pub mod reducible;

pub trait CheckGcd: Sized + Checked {
    fn gcd(&self, rhs: &Self) -> Option<Self>;
}

pub trait Checked {}
pub trait Gcd: Sized {
    fn gcd(&self, rhs: &Self) -> Self;
}

pub trait Zero {
    const ZERO: Self;

    fn is_zero(&self) -> bool
    where
        Self: PartialEq + Sized,
    {
        *self == Self::ZERO
    }
}

pub trait One {
    const ONE: Self;

    fn is_one(&self) -> bool
    where
        Self: PartialEq + Sized,
    {
        *self == Self::ONE
    }

    fn non_zero() -> Self
    where
        Self: Sized,
    {
        Self::ONE
    }
}

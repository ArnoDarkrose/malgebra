use crate::{CheckGcd, Checked};
use paste::paste;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

pub trait IntoCheck<U> {
    fn safe(self) -> U;
}

macro_rules! impl_ops {
    (shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident));*) => {
        $(
            impl $op<CheckU32> for $name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name(self, rhs: CheckU32) -> Self::Output {
                        if let Some(val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some(Self(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };

    (without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for $name {
                type Output = Self;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    Self(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };

    ($typ:ty, $name:ident: $(($op:ident, $fn_name:ident));*) => {
        $(
            impl $op for $name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name (self, rhs:Self) -> Self::Output {
                        if let Some (val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some($name(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };

    (ref - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident(rhs: $rhs_typ:ty)));*) => {
        $(
            impl $op for &$name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name (self, rhs: &$rhs_typ) -> Self::Output {
                        if let Some(val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some($name(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };


    (ref mut - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident(rhs: $rhs_typ:ty)));*) => {
        $(
            impl $op for &mut $name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name (self, rhs: &mut $rhs_typ) -> Self::Output {
                        if let Some(val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some($name(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };

    (ref shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident));*) => {
        $(
            impl $op<&CheckU32> for &$name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name (self, rhs: &CheckU32) -> Self::Output {
                        if let Some(val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some($name(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };

    (ref without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for &$name {
                type Output = $name;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    $name(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };

    (ref mut shift - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident));*) => {
        $(
            impl $op<&mut CheckU32> for &mut $name {
                type Output = Option<$name>;

                paste!{
                    fn $fn_name (self, rhs: &mut CheckU32) -> Self::Output {
                        if let Some(val) = self.0.[<checked_$fn_name>](rhs.0) {
                            Some($name(val))
                        } else {
                            None
                        }
                    }
                }
            }
        )*
    };

    (ref mut without check - $typ:ty, $name:ident: $(($op:ident, $fn_name:ident($($param:ident: $param_typ:ty),*)));*) => {
        $(
            impl $op for &mut $name {
                type Output = $name;

                fn $fn_name(self $(, $param: $param_typ)*) -> Self::Output{
                    $name(self.0.$fn_name($($param.0),*))
                }
            }
        )*
    };
}

macro_rules! impl_method {
    (self. $(($name:ident ($($param_name:ident : $typ:ty),*) -> $ret_typ:ident));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> $ret_typ {
                $ret_typ(self.0.$name($($param_name.0),*))
            }
        )*
    };

    (with Output as Self: self. $(($name:ident ($($param_name:ident : $typ:ty),* ) -> $ret_typ:ty));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> $ret_typ {
                Self(self.0.$name($($param_name.0),*))
            }
        )*
    };

    (return Option: self. $(($name:ident, $meth_name:ident ($($param_name:ident : $typ:ty),* ) -> Option<$ret_typ: ty>));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> Option<$ret_typ> {
                if let Some(val) = self.0.$meth_name($($param_name.0),*) {
                    Some(<$ret_typ>::new(val))
                } else {
                    None
                }
            }
        )*
    };

    (return [Check_u8; 4]: self. $(($name:ident($($param_name:ident : $typ:ty),* )));*) => {
        $(
            // TODO: unconst map issue
        )*
    };

    (to_bytes - self. $(($name:ident($($param_name:ident : $typ:ty),* ) -> [u8; $size:expr]));*) => {
        $(
            pub const fn $name (self $(, $param_name: $typ)*) -> [u8; $size] {
                self.0.$name($($param_name),*)
            }
        )*
    };

    (from_bytes - $(($typ:ty, $name:ident(bytes: [u8; $size:expr]) -> Self));*) => {
        $(
            pub const fn $name(bytes: [u8; $size]) -> Self {
                Self(<$typ>::$name(bytes))
            }
        )*
    }
}

macro_rules! implFormatTrait {
    ($name:ident: $($trait:ident),*) => {
        $(
            impl std::fmt::$trait for $name {
                fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
                    self.0.fmt(f)
                }
            }
        )*
    };
}

macro_rules! define {
    ($($typ:ident);*) => {
        $(
            paste!{
                define_!($typ, [<Check$typ:camel>]);
            }
        )*
    };
}

macro_rules! define_ {
    ($typ:ty, $name:ident) => {
        /// Main struct for working with checked primitives.
        /// It is very close to std primitives in its functionality
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name ($typ);

        impl $name {
            #[allow(dead_code)]
            pub const fn new(val: $typ) -> $name {
                $name (val)
            }

            pub const MIN: Self = Self(<$typ>::MIN);
            pub const MAX: Self = Self(<$typ>::MAX);
            pub const BITS: u32 = <$typ>::BITS;

            impl_method!(self.
                (count_zeros() -> CheckU32); (leading_zeros() -> CheckU32); (count_ones() -> CheckU32); (trailing_zeros() -> CheckU32);
                (leading_ones() -> CheckU32); (trailing_ones() -> CheckU32)
            );

            impl_method!(with Output as Self: self.
                (rotate_left(n: CheckU32) -> Self); (rotate_right(n: CheckU32) -> Self); (swap_bytes() -> Self); (reverse_bits() -> Self);
                (to_be() -> Self); (to_le() -> Self)
            );

            impl_method!(return Option: self.
                (div_euclid, checked_div_euclid(rhs: Self) -> Option<Self>); (rem_euclid, checked_rem_euclid(rhs: Self) -> Option<Self>);
                (pow, checked_pow(exp: CheckU32) -> Option<Self>); (ilog, checked_ilog(base: Self) -> Option<CheckU32>);
                (ilog2, checked_ilog2() -> Option<CheckU32>); (ilog10, checked_ilog10() -> Option<CheckU32>)
            );


            pub fn from_str_radix(src: &str, radix: CheckU32) -> Result<Self, std::num::ParseIntError> {
                let res = <$typ>::from_str_radix(src, radix.0);
                match res {
                    Ok(val) => Ok(Self(val)),
                    Err(val) => Err(val),
                }
            }

            pub const fn from_be(x: Self) -> Self {
                Self(<$typ>::from_be(x.0))
            }

            pub const fn from_le(x: Self) -> Self {
                Self(<$typ>::from_le(x.0))
            }

            //TODO change this functions to return array of CheckU8 while remaining const
            impl_method!(to_bytes - self.
                (to_be_bytes() -> [u8; (0 as $typ).to_be_bytes().len()]);
                (to_le_bytes() -> [u8; (0 as $typ).to_le_bytes().len()]);
                (to_ne_bytes() -> [u8; (0 as $typ).to_ne_bytes().len()])
            );

            // TODO: Similar to the above
            impl_method!(from_bytes -
                ($typ, from_be_bytes(bytes: [u8; (0 as $typ).to_be_bytes().len()]) -> Self);
                ($typ, from_le_bytes(bytes: [u8; (0 as $typ).to_le_bytes().len()]) -> Self);
                ($typ, from_ne_bytes(bytes: [u8; (0 as $typ).to_ne_bytes().len()]) -> Self)
            );

            fn gcd_sign_unsafe(&self, rhs: &Self) -> Option<Self> {
                let rem = (self % rhs)?;

                if rem == Self::try_from(0).unwrap() {
                    return Some(rhs.clone())
                }

                rhs.gcd_sign_unsafe(&rem)
            }
        }

        impl IntoCheck<$name> for $typ {
            fn safe(self) -> $name {
                $name(self)
            }
        }

        impl_ops!($typ, $name:
            (Add, add); (Sub, sub); (Div, div); (Mul, mul); (Rem, rem)
        );

        impl_ops!(shift - $typ, $name: (Shl, shl); (Shr, shr));

        impl Neg for $name {
            type Output = Option<$name>;

            fn neg(self) -> Self::Output{
                if let Some(val) = self.0.checked_neg() {
                    Some($name(val))
                } else {
                    None
                }
            }
        }

        implFormatTrait!($name: Display, Binary, LowerExp, LowerHex, Octal, UpperExp, UpperHex);

        impl std::str::FromStr for $name {
            type Err = std::num::ParseIntError;

            fn from_str(src: &str) -> Result<Self, std::num::ParseIntError> {
                match <$typ>::from_str(src) {
                    Ok(val) => Ok(Self(val)),
                    Err(e) => Err(e)
                }
            }
        }

        impl_ops!(without check - $typ, $name:
            (BitAnd, bitand(rhs: $name)); (BitOr, bitor(rhs: $name)); (BitXor, bitxor(rhs: $name)); (Not, not())
        );

        impl_ops!(ref - $typ, $name:
            (Add, add(rhs: $name)); (Sub, sub(rhs: $name)); (Mul, mul(rhs: $name));
            (Div, div(rhs: $name)); (Rem, rem(rhs: $name))
        );

        impl_ops!(ref without check - $typ, $name:
            (BitAnd, bitand(rhs: &$name)); (BitOr, bitor(rhs: &$name)); (BitXor, bitxor(rhs: &$name)); (Not, not())
        );

        impl_ops!(ref shift - $typ, $name:
            (Shl, shl); (Shr, shr)
        );

        impl Neg for &$name {
            type Output = Option<$name>;

            fn neg(self) -> Self::Output {
                if let Some(val) = self.0.checked_neg() {
                    Some($name(val))
                } else {
                    None
                }
            }
        }

        impl_ops!(ref mut - $typ, $name:
            (Add, add(rhs: $name)); (Sub, sub(rhs: $name)); (Mul, mul(rhs: $name));
            (Div, div(rhs: $name)); (Rem, rem(rhs: $name))
        );

        impl_ops!(ref mut shift - $typ, $name:
            (Shl, shl); (Shr, shr)
        );

        impl_ops!(ref mut without check - $typ, $name:
            (BitAnd, bitand(rhs: &mut $name)); (BitOr, bitor(rhs: &mut $name)); (BitXor, bitxor(rhs: &mut $name)); (Not, not())
        );

        impl Neg for &mut $name {
            type Output = Option<$name>;

            fn neg(self) -> Self::Output {
                if let Some(val) = self.0.checked_neg() {
                    Some($name(val))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! define_unsigned {
    ($($typ:ident);*) => {
        $(
            paste!{
                define_unsigned_!{$typ, [<Check$typ:camel>]}
            }
        )*
    }
}
macro_rules! define_unsigned_ {
    ($($typ:ty, $name:ident);*) => {
        $(
            impl $name {
                #[allow(dead_code)]
                pub const fn div_ceil(self, rhs: Self) -> Self {
                    Self(self.0.div_ceil(rhs.0))
                }

                pub const fn next_multiple_of(self, rhs: Self) -> Option<Self> {
                    if let Some(val) = self.0.checked_next_multiple_of(rhs.0) {
                        Some(Self(val))
                    } else {
                        None
                    }
                }

                pub const fn is_power_of_two(self) -> bool {
                    self.0.is_power_of_two()
                }

                pub const fn next_power_of_two(self) -> Option<Self> {
                    if let Some(val) = self.0.checked_next_power_of_two() {
                        Some(Self(val))
                    } else {
                        None
                    }
                }

                pub const fn abs_diff(self, other: Self) -> Self {
                    Self(self.0.abs_diff(other.0))
                }
            }

            impl Checked for $name{}

            impl CheckGcd for $name {
                fn gcd(&self, rhs: &Self) -> Option<Self> {
                    self.gcd_sign_unsafe(rhs)
                }
            }
        )*
    };
}

macro_rules! define_signed {
    ($($typ:ident, unsigned - $utyp:ident);*) => {
        paste!{
            $(
                define_signed_!{$typ, [<Check$typ:camel>] - unsigned: $utyp, [<Check$utyp:camel>]}
            )*
        }
    };
}
macro_rules! define_signed_ {
    ($typ:ty, $name:ident - unsigned: $utyp:ty, $uname:ident) => {
        impl $name {
            #[allow(dead_code)]
            pub const fn is_positive(self) -> bool {
                self.0.is_positive()
            }

            #[allow(dead_code)]
            pub const fn is_negative(self) -> bool {
                self.0.is_negative()
            }

            pub const fn abs_diff(self, other: Self) -> $uname {
                $uname(self.0.abs_diff(other.0))
            }

            pub const fn abs(self) -> Option<Self> {
                if let Some(val) = self.0.checked_abs() {
                    Some(Self(val))
                } else {
                    None
                }
            }
        }

        impl Checked for $name {}

        impl CheckGcd for $name {
            fn gcd(&self, rhs: &Self) -> Option<Self> {
                self.abs()?.gcd_sign_unsafe(&rhs.abs()?)
            }
        }
    };
}

macro_rules! defineFromInt {
    ($($typ:ident: ($($from_typ:ident);*));*) => {
        paste!{
            $(
                defineFromInt_!{$typ, [<Check$typ:camel>]: ($($from_typ, [<Check$from_typ:camel>]);*)}
            )*
        }
    };
}
macro_rules! defineFromInt_ {
    ($($typ:ty, $name:ident: ($($from_typ:ty, $from_name:ident);*));*) =>  {
        $(
            $(
                impl From<$from_name> for $name {
                    fn from(value: $from_name) -> Self {
                        Self(value.0 as $typ)
                    }
                }

                impl From<$from_typ> for $name { fn from(value: $from_typ) -> Self { Self(value as $typ) } }
            )*
        )*
    }
}

macro_rules! defineFrom {
    ($($typ:ident: ($($directly_from:ty),*));*) => {
        paste!{
            $(
                defineFrom_!{$typ, [<Check$typ:camel>]: ($($directly_from),*)}
            )*
        }
    };
}
macro_rules! defineFrom_ {
    ($typ:ty, $name:ident: ($($directly_from:ty),*)) => {
        $(
            impl From<$directly_from> for $name {
                fn from(value: $directly_from) -> Self {
                    Self(<$typ>::from(value))
                }
            }
        )*
    };
}

macro_rules! defineTryFromCheck {
    ($($typ:ident: $($from_typ:ident),*);*) => {
        paste!{
            $(
                defineTryFromCheck_!{$typ, [<Check$typ:camel>]: $(([<Check$from_typ:camel>], $from_typ)),*}
            )*
        }
    };
}
macro_rules! defineTryFromCheck_ {
    ($($typ:ty, $name:ident: $(($from_name:ident, $from_typ:ty)),*);*) => {
        $(
            $(
                impl TryFrom<$from_name> for $name {
                    type Error = <$typ as TryFrom<$from_typ>>::Error;

                    fn try_from(u: $from_name) -> Result<$name, Self::Error> {
                        match <$typ>::try_from(u.0) {
                            Ok(val) => Ok(Self(val)),
                            Err(e) => Err(e)
                        }
                    }
                }
            )*
        )*
    }
}

macro_rules! defineTryFromInt {
    ($($typ:ty: $($from_typ:ty),*);*) => {
        paste!{
            $(
                defineTryFromInt_!{$typ, [<Check$typ:camel>]: $($from_typ),*}
            )*
        }
    };
}

macro_rules! defineTryFromInt_ {
    ($($typ:ty, $name:ident: $($from_typ:ty),*);*) => {
        $(
            $(
                impl TryFrom<$from_typ> for $name {
                    type Error = <$typ as TryFrom<$from_typ>>::Error;

                    fn try_from(u: $from_typ) -> Result<$name, Self::Error> {
                        match <$typ>::try_from(u) {
                            Ok(val) => Ok(Self(val)),
                            Err(e) => Err(e)
                        }
                    }
                }
            )*
        )*
    }
}

macro_rules! defineFromSelf_typ {
    ($($typ:ty);*) => {
        $(
            paste!{
                impl From<$typ> for [<Check$typ:camel>] {
                    fn from(value: $typ) -> Self {
                        Self(value)
                    }
                }
            }
        )*
    };
}

define! {u32; i8; i16; i32; i64; i128; u8; u16; u64; u128; usize; isize}

define_unsigned! {u8; u16; u32; u64; u128; usize}

define_signed! {
    i8, unsigned - u8;
    i16, unsigned - u16;
    i32, unsigned - u32;
    i64, unsigned - u64;
    i128, unsigned - u128;
    isize, unsigned - usize
}

defineFromInt! {
    u16: (u8);
    i16: (i8);
    u32: (u8; u16);
    i32: (i8; i16);
    u64: (u8; u16; u32);
    i64: (i8; i16; i32);
    u128: (u8; u16; u32; u64);
    i128: (i8; i16; i32; i64);
    usize: (u8; u16);
    isize: (i8; u8; i16)
}

defineFrom! {
    u8: (std::num::NonZeroU8, bool);
    i8: (std::num::NonZeroI8, bool);
    u16: (std::num::NonZeroU16, bool);
    i16: (std::num::NonZeroI16, bool);
    u32: (std::num::NonZeroU32, bool, std::net::Ipv4Addr, char);
    i32: (std::num::NonZeroI32, bool);
    u64: (std::num::NonZeroU64, bool, char);
    i64: (std::num::NonZeroI64, bool);
    u128: (std::num::NonZeroU128, bool, std::net::Ipv6Addr, char);
    i128: (std::num::NonZeroI128, bool);
    usize: (std::num::NonZeroUsize, bool);
    isize: (std::num::NonZeroIsize, bool)
}

defineTryFromInt! {
    u8: i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    i8: u8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    u16: i8, i16, u32, i32, u64, i64, u128, i128, usize, isize;
    i16: u8, u16, u32, i32, u64, i64, u128, i128, usize, isize;
    u32: i8, i16, i32, u64, i64, u128, i128, usize, isize;
    i32: u8, u16, u32, u64, i64, u128, i128, usize, isize;
    u64: i8, i16, i32, i64, u128, i128, usize, isize;
    i64: u8, u16, u32, u64, u128, i128, usize, isize;
    u128: i8, i16, i32, i64, i128, usize, isize;
    i128: u8, u16, u32, u64, u128, usize, isize;
    usize: i8, i16, u32, i32, u64, i64, i128, u128, isize;
    isize: u16, u32, i32, u64, i64, u128, i128, usize
}

defineTryFromCheck! {
    u8: i8, i16, i32, i64, i128, u16, u32, u64, u128, usize, isize;
    i8: u8, u16, u32, u64, u128, i16, i32, i64, i128, usize, isize;
    u16: i8, i16, i32, i64, i128, u32, u64, u128, usize, isize;
    i16: u8, u16, u32, u64, u128, i32, i64, i128, usize, isize;
    u32: i8, i16, i32, i64, i128, u64, u128, usize, isize;
    i32: u8, u16, u32, u64, u128, i64, i128, usize, isize;
    u64: i8, i16, i32, i64, i128, u128, usize, isize;
    i64: u8, u16, u32, u64, u128, i128, usize, isize;
    u128: i8, i16, i32, i64, i128, usize, isize;
    i128: u8, u16, u32, u64, u128, usize, isize;
    usize: i8, i16, i32, i64, i128, u32, u64, u128, isize;
    isize: u16, u32, u64, u128, i32, i64, i128, usize
}

defineFromSelf_typ! {
    u8; u16; u32; u64; u128; usize;
    i8; i16; i32; i64; i128; isize
}

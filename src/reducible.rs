pub mod checked_reducible {
    use crate::{CheckGcd, Zero, One, Checked};

    use std::{
        fmt,
        ops::{Add, Div, Mul, Neg, Sub},
    };

    #[derive(Debug, Clone)]
    pub struct CheckRdc<T: CheckGcd + Zero + One + PartialEq> {
        num: T,
        denom: T,
    }

    impl<T: CheckGcd + Zero + One + PartialEq> CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>,
    {
        pub fn new(num: T, denom: T) -> CheckRdc<T> {
            if denom.is_zero() {
                panic!("Zero denominator");
            }

            let mut res = CheckRdc { num, denom };

            res.simplify();

            res
        }

        pub fn num(&self) -> &T {
            &self.num
        }

        pub fn denom(&self) -> &T {
            &self.denom
        }

        pub fn simplify(&mut self) -> Option<()> {
            let gcd = self.num().gcd(self.denom())?;

            self.num = (self.num() / &gcd).expect("Never fails");

            self.denom = (self.denom() / &gcd).expect("Never fails");

            Some(())
        }
    }

    #[macro_export]
    macro_rules! chrdc {
        ($num:expr, $denom:expr) => {
            CheckRdc::new($num, $denom)
        };

        ($typ:ty) => {
            CheckRdc::new(<$typ>::non_zero(), <$typ>::non_zero())
        };

        ($num_denom:expr) => {
            CheckRdc::new($num_denom.0, $num_denom.1)
        };
    }

    impl<T: CheckGcd + Zero + One + PartialEq> std::default::Default for CheckRdc<T> {
        fn default() -> Self {
            CheckRdc {
                num: T::non_zero(),
                denom: T::non_zero(),
            }
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> PartialEq for CheckRdc<T>
    where
        for<'a> &'a T: Mul<&'a T, Output = Option<T>> + Div<&'a T, Output = Option<T>>,
    {
        fn eq(&self, other: &Self) -> bool {
            let mut overflowed = false;

            let lhs = match &self.num * &other.denom {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return lhs == rhs;
            }

            let mut new_self = self.clone();
            new_self.simplify();

            let mut other = other.clone();
            other.simplify();

            let denom_gcd = new_self.denom().gcd(other.denom()).unwrap();

            let lhs = match new_self.num() * &((other.denom() / &denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions"),
            };

            let rhs = match other.num() * &((new_self.denom() / &denom_gcd).expect("Never fails")) {
                Some(val) => val,
                None => panic!("Failed to compare fractions"),
            };

            lhs == rhs
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Eq for CheckRdc<T> where
        for<'a> &'a T: Mul<&'a T, Output = Option<T>> + Div<&'a T, Output = Option<T>>
    {
    }

    impl<T: CheckGcd + Zero + One + PartialEq + PartialOrd + Clone> PartialOrd for CheckRdc<T>
    where
        for<'a> &'a T: Mul<&'a T, Output = Option<T>> + Div<&'a T, Output = Option<T>>,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            let mut overflowed = false;

            let lhs = match &self.num * &other.denom {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let rhs = match &self.denom * &other.num {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return lhs.partial_cmp(&rhs);
            }

            let mut new_self = self.clone();
            new_self.simplify();

            let mut other = other.clone();
            other.simplify();

            let denom_gcd = self.denom().gcd(other.denom())?;

            let lhs = (new_self.num() * &((other.denom() / &denom_gcd).expect("Never fails")))?;

            let rhs = (other.num() * &((new_self.denom() / &denom_gcd).expect("Never fails")))?;

            lhs.partial_cmp(&rhs)
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Mul<Self> for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return Some(CheckRdc { num, denom });
            }

            self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom())?;
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num())?;

            self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (self.num() * rhs.num())?;

            let denom = (self.denom() * rhs.denom())?;

            Some(CheckRdc { num, denom })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Mul<Self> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn mul(self, rhs: Self) -> Self::Output {
            let mut overflowed = false;

            let num = match self.num() * rhs.num() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return Some(CheckRdc { num, denom });
            }

            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_denom_gcd = self.num().gcd(rhs.denom())?;
            let s_denom_r_num_gcd = self.denom().gcd(rhs.num())?;

            new_self.num = (self.num() / &s_num_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_num_r_denom_gcd).expect("Never fails");

            new_self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_denom_r_num_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.num())?;

            let denom = (new_self.denom() * rhs.denom())?;

            Some(CheckRdc { num, denom })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Mul<&T> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn mul(self, rhs: &T) -> Self::Output {
            match self.num() * rhs {
                Some(val) => Some(CheckRdc {
                    num: val,
                    denom: self.denom.clone(),
                }),
                None => {
                    let mut new_self = self.clone();
                    let mut rhs = rhs.clone();
                    new_self.simplify();

                    let s_denom_r_num_gcd = self.denom().gcd(&rhs)?;

                    new_self.denom = (new_self.denom() / &s_denom_r_num_gcd).expect("Never fails");
                    rhs = (&rhs / &s_denom_r_num_gcd).expect("Never failes");

                    let num = (new_self.num() * &rhs)?;

                    Some(CheckRdc {
                        num,
                        denom: new_self.denom,
                    })
                }
            }
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Mul<&mut T> for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn mul(self, rhs: &mut T) -> Self::Output {
            match self.num() * rhs {
                Some(val) => Some(CheckRdc {
                    num: val,
                    denom: self.denom.clone(),
                }),
                None => {
                    let mut rhs = rhs.clone();
                    self.simplify();

                    let s_denom_r_num_gcd = self.denom().gcd(&rhs)?;

                    self.denom = (self.denom() / &s_denom_r_num_gcd).expect("Never fails");
                    rhs = (&rhs / &s_denom_r_num_gcd).expect("Never failes");

                    let num = (self.num() * &rhs)?;

                    Some(CheckRdc {
                        num,
                        denom: self.denom.clone(),
                    })
                }
            }
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Div<&T> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn div(self, rhs: &T) -> Self::Output {
            if rhs.is_zero() {
                panic!("dividing by zero")
            }

            match self.denom() * rhs {
                Some(val) => Some(CheckRdc {
                    num: self.num.clone(),
                    denom: val,
                }),
                None => {
                    let mut new_self = self.clone();
                    let mut rhs = rhs.clone();
                    new_self.simplify();

                    let s_num_r_num_gcd = self.num().gcd(&rhs)?;

                    new_self.num = (new_self.num() / &s_num_r_num_gcd).expect("never fails");
                    rhs = (&rhs / &s_num_r_num_gcd).expect("never failes");

                    let denom = (new_self.denom() * &rhs)?;

                    Some(CheckRdc {
                        num: new_self.num.clone(),
                        denom,
                    })
                }
            }
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Add<Self> for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Add<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            // HACK: here and a few lines later i declare a reference that'll be mainly used in calcualtions
            // but when overflow occurs i change multipliers. So second variable
            // is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;

            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult)?
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (self_mult * self.num())?
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (rhs_mult * rhs.num())?
                }
            };

            let new_num = match &new_num_part1 + &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (&(self_mult * self.num())? + &(rhs_mult * rhs.num())?)?
                }
            };

            Some(CheckRdc {
                num: new_num,
                denom: new_denom,
            })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Add<Self> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Add<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn add(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            &mut new_self + &mut rhs
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Sub<Self> for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Sub<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut already_simplified = false;

            let mut denom_gcd;

            // HACK: here and a few lines later i declare a reference that'll be mainly used in calcualtions
            // but when overflow occurs i change multipliers. So the second variable
            // is for storing those values and the first is for taking references to them
            let mut self_mult = rhs.denom();
            let mut self_mult_val;

            let mut rhs_mult = self.denom();
            let mut rhs_mult_val;

            let mut new_denom = match self.denom() * rhs.denom() {
                Some(val) => val,
                None => {
                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    (self.denom() * self_mult)?
                }
            };

            let new_num_part1 = match self.num() * self_mult {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (self_mult * self.num())?
                }
            };

            let new_num_part2 = match rhs_mult * rhs.num() {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    already_simplified = true;

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (rhs_mult * rhs.num())?
                }
            };

            let new_num = match &new_num_part1 - &new_num_part2 {
                Some(val) => val,
                None => {
                    if already_simplified {
                        return None;
                    }

                    self.simplify();
                    rhs.simplify();

                    denom_gcd = self.denom().gcd(rhs.denom())?;

                    self_mult_val = (rhs.denom() / &denom_gcd).expect("Never fails");
                    self_mult = &self_mult_val;

                    rhs_mult_val = (self.denom() / &denom_gcd).expect("Never fails");
                    rhs_mult = &rhs_mult_val;

                    new_denom = (self.denom() * self_mult)?;

                    (&(self_mult * self.num())? - &(rhs_mult * rhs.num())?)?
                }
            };

            Some(CheckRdc {
                num: new_num,
                denom: new_denom,
            })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Sub<Self> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Sub<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn sub(self, rhs: Self) -> Self::Output {
            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            &mut new_self - &mut rhs
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Div<Self> for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return Some(CheckRdc { num, denom });
            }

            self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = self.num().gcd(rhs.num())?;
            let s_denom_r_denom_gcd = self.denom().gcd(rhs.denom())?;

            self.num = (self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            self.denom = (self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (self.num() * rhs.denom())?;

            let denom = (self.denom() * rhs.num())?;

            Some(CheckRdc { num, denom })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq + Clone> Div<Self> for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn div(self, rhs: Self) -> Self::Output {
            if rhs.num.is_zero() {
                panic!("Dividing by zero");
            }

            let mut overflowed = false;

            let num = match self.num() * rhs.denom() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            let denom = match self.denom() * rhs.num() {
                Some(val) => val,
                None => {
                    overflowed = true;
                    T::non_zero()
                }
            };

            if !overflowed {
                return Some(CheckRdc { num, denom });
            }

            let mut new_self = self.clone();
            let mut rhs = rhs.clone();

            new_self.simplify();
            rhs.simplify();

            let s_num_r_num_gcd = new_self.num().gcd(rhs.num())?;
            let s_denom_r_denom_gcd = new_self.denom().gcd(rhs.denom())?;

            new_self.num = (new_self.num() / &s_num_r_num_gcd).expect("Never fails");
            rhs.num = (rhs.num() / &s_num_r_num_gcd).expect("Never fails");

            new_self.denom = (new_self.denom() / &s_denom_r_denom_gcd).expect("Never fails");
            rhs.denom = (rhs.denom() / &s_denom_r_denom_gcd).expect("Never fails");

            let num = (new_self.num() * rhs.denom())?;

            let denom = (new_self.denom() * rhs.num())?;

            Some(CheckRdc { num, denom })
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Neg for &mut CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Sub<&'a T, Output = Option<T>>,
        for<'a> &'a T: Div<&'a T, Output = Option<T>> + Mul<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn neg(self) -> Self::Output {
            &mut CheckRdc {
                num: T::ZERO,
                denom: T::non_zero(),
            } - self
        }
    }

    impl<T: CheckGcd + Zero + One + Clone + PartialEq> Neg for &CheckRdc<T>
    where
        for<'a> &'a T: Div<&'a T, Output = Option<T>>
            + Mul<&'a T, Output = Option<T>>
            + Sub<&'a T, Output = Option<T>>,
    {
        type Output = Option<CheckRdc<T>>;

        fn neg(self) -> Self::Output {
            &CheckRdc::<T>::ZERO - self
        }
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Zero for CheckRdc<T> {
        const ZERO: Self = CheckRdc {
            num: T::ZERO,
            denom: T::ONE,
        };
    }

    impl<T: CheckGcd + Zero + One + PartialEq> Checked for CheckRdc<T> {}

    impl<T: CheckGcd + Zero + One + fmt::Display + PartialEq> fmt::Display for CheckRdc<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({})/({})", self.num, self.denom)
        }
    }
}

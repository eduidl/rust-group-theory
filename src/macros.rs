#[macro_export]
macro_rules! impl_add_group {
    ($T: ty, $AddGroup: ty) => {
        use std::ops::{Add, Neg, Sub};

        impl Neg for $T {
            type Output = Self;

            fn neg(self) -> Self {
                <$AddGroup>::inverse(self)
            }
        }

        impl Add for $T {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                <$AddGroup>::call(self, other)
            }
        }

        impl Sub for $T {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                <$AddGroup>::call(self, <$AddGroup>::inverse(other))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_ring {
    ($T: ty, $RingT: ty) => {
        use std::ops::{Add, Mul, Neg, Sub};

        impl Neg for $T {
            type Output = Self;

            fn neg(self) -> Self {
                <$RingT>::minus(self)
            }
        }

        impl Add for $T {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                <$RingT>::add(self, other)
            }
        }

        impl Sub for $T {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                <$RingT>::sub(self, other)
            }
        }

        impl Mul for $T {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                <$RingT>::mul(self, other)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_field {
    ($T: ty, $FieldT: ty) => {
        use std::ops::Div;

        crate::impl_ring!($T, $FieldT);

        impl Div for $T {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                <$FieldT>::div(self, other).unwrap()
            }
        }
    };
}

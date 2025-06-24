use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A type which can be used as the backing storage for a bitfield.
pub trait Storage: BitSized + private::Sealed {
    const EMPTY: Self;
    const MAX_BITS: Self::Bits;

    fn extract<T>(&self, offset: u32) -> T
    where
        T: BitSized,
        T::Bits: Widen<Self::Bits>,
    {
        check_bits_and_offset::<Self, T>(offset);

        let shifted = self.into_bits().wrapping_shr(offset);
        let narrowed = T::Bits::narrow(shifted);

        T::from_bits(narrowed)
    }

    fn insert<T>(&mut self, offset: u32, value: T)
    where
        T: BitSized,
        T::Bits: Widen<Self::Bits>,
    {
        check_bits_and_offset::<Self, T>(offset);

        let mask = Self::Bits::MAX
            .wrapping_shr((Self::BITS - T::BITS) as u32)
            .wrapping_shl(offset);

        let widened = value.into_bits().widen();
        let shifted = widened.wrapping_shl(offset);

        let combined = (self.into_bits() & !mask) | (shifted & mask);
        *self = Self::from_bits(combined);
    }
}

fn check_bits_and_offset<S: Storage, T: BitSized>(offset: u32) {
    assert!(S::BITS <= u32::MAX as usize);

    assert!(T::BITS < S::BITS);
    assert!(offset <= (S::BITS - T::BITS) as u32);
}

/// A type with bit-grained size.
pub trait BitSized: Copy {
    type Bits: Bits;

    const BITS: usize;

    fn from_bits(bits: Self::Bits) -> Self;
    fn into_bits(self) -> Self::Bits;
}

/// Bits which can be operated on.
///
/// (Byte grained.)
pub trait Bits:
    Copy
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + Not<Output = Self>
    + private::Sealed
{
    const ZERO: Self;
    const MAX: Self;

    #[must_use]
    fn wrapping_shl(self, n: u32) -> Self;

    #[must_use]
    fn wrapping_shr(self, n: u32) -> Self;
}

pub trait Widen<Wide> {
    fn widen(self) -> Wide;
    fn narrow(wide: Wide) -> Self;
}

macro_rules! impl_uint {
    ($($n:ident)*) => {
        $(
            impl Bits for $n {
                const ZERO: Self = 0;
                const MAX: Self = Self::MAX;

                fn wrapping_shl(self, n: u32) -> Self {
                    self.wrapping_shl(n)
                }

                fn wrapping_shr(self, n: u32) -> Self {
                    self.wrapping_shr(n)
                }
            }

            impl BitSized for $n {
                type Bits = $n;

                const BITS: usize = Self::BITS as usize;

                #[inline]
                fn into_bits(self) -> Self::Bits {
                    self
                }

                #[inline]
                fn from_bits(bits: Self::Bits) -> Self {
                    bits
                }
            }

            impl<const BITS: usize> BitSized for arbitrary_int::UInt<$n, BITS> {
                type Bits = $n;

                const BITS: usize = BITS;

                #[inline]
                fn into_bits(self) -> Self::Bits {
                    self.value()
                }

                #[inline]
                fn from_bits(bits: Self::Bits) -> Self {
                    <Self as arbitrary_int::Number>::masked_new(bits)
                }
            }

            impl Storage for $n {
                const EMPTY: Self = 0;
                const MAX_BITS: Self::Bits = $n::MAX;
            }
            impl private::Sealed for $n {}

            impl<const BITS: usize> Storage for arbitrary_int::UInt<$n, BITS> {
                const EMPTY: Self = Self::new(0);
                const MAX_BITS: Self::Bits = <Self as arbitrary_int::Number>::MAX.value();
            }

            impl<const BITS: usize> private::Sealed for arbitrary_int::UInt<$n, BITS> {}
        )*
    };
}

impl_uint!(u8 u16 u32 u64 u128);

impl BitSized for bool {
    type Bits = u8;

    const BITS: usize = 1;

    #[inline]
    fn from_bits(bits: Self::Bits) -> Self {
        (bits & 1) != 0
    }

    #[inline]
    fn into_bits(self) -> Self::Bits {
        self as u8
    }
}

macro_rules! impl_widen {
    ($narrow:ident $($wide:ident)*) => {
        $(
        impl Widen<$wide> for $narrow {
            #[inline]
            fn widen(self) -> $wide {
                self as $wide
            }

            #[inline]
            fn narrow(wide: $wide) -> Self {
                wide as Self
            }
        }
        )*

        impl_widen!($($wide)*);
    };

    () => {};
}

impl_widen!(u8 u16 u32 u64 u128);

impl<T> Widen<T> for T {
    #[inline]
    fn widen(self) -> T {
        self
    }

    #[inline]
    fn narrow(wide: T) -> Self {
        wide
    }
}

mod private {
    pub trait Sealed {}
}

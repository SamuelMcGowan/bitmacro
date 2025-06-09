use std::ops::{BitAnd, BitOr, BitXor, Not};

/// A type which can be used as the backing storage for a bitfield.
pub trait Storage: BitSized + private::Sealed {
    fn extract<T>(&self, offset: u32) -> T
    where
        T: BitSized,
        T::Inner: Widen<Self::Inner>,
    {
        check_bits_and_offset::<Self, T>(offset);

        let shifted = self.into_inner().wrapping_shr(offset);
        let narrowed = T::Inner::narrow(shifted);

        T::from_inner(narrowed)
    }

    fn insert<T>(&mut self, offset: u32, value: T)
    where
        T: BitSized,
        T::Inner: Widen<Self::Inner>,
    {
        check_bits_and_offset::<Self, T>(offset);

        let mask = Self::Inner::MAX
            .wrapping_shr((Self::BITS - T::BITS) as u32)
            .wrapping_shl(offset);

        let widened = value.into_inner().widen();
        let shifted = widened.wrapping_shl(offset);

        let combined = (self.into_inner() & !mask) | (shifted & mask);
        *self = Self::from_inner(combined);
    }
}

fn check_bits_and_offset<S: Storage, T: BitSized>(offset: u32) {
    assert!(S::BITS <= u32::MAX as usize);

    assert!(T::BITS < S::BITS);
    assert!(offset <= (S::BITS - T::BITS) as u32);
}

/// A type with bit-grained size.
pub trait BitSized: Copy {
    type Inner: Bits;

    const BITS: usize;

    fn from_inner(inner: Self::Inner) -> Self;
    fn into_inner(self) -> Self::Inner;
}

/// Bits which can be operated on.
///
/// (Byte grained.)
pub trait Bits:
    Copy + BitOr<Output = Self> + BitAnd<Output = Self> + BitXor<Output = Self> + Not<Output = Self>
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

mod private {
    pub trait Sealed {}
}

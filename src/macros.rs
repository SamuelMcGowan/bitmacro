#[macro_export]
macro_rules! bitfield {
    (
        $vis:vis struct $ident:ident : $storage:ty {
            $(
                $( $field_vis:vis $field:ident : $field_ty:ty ),+
                $(,)?
            )?
        }
    ) => {
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        $vis struct $ident($storage);

        impl $ident {
            #[inline]
            pub const fn empty() -> Self {
                Self(<$storage as $crate::internal::Storage>::EMPTY)
            }
        }

        impl $crate::internal::BitSized for $ident
        where
            $storage: $crate::internal::Storage
        {
            type Bits = <$storage as $crate::internal::BitSized>::Bits;

            const BITS: usize = 0 $($( + <$field_ty as $crate::internal::BitSized>::BITS )*)?;

            #[inline]
            fn from_bits(bits: Self::Bits) -> Self {
                Self(<$storage as $crate::internal::BitSized>::from_bits(bits))
            }

            #[inline]
            fn into_bits(self) -> Self::Bits {
                <$storage as $crate::internal::BitSized>::into_bits(self.0)
            }
        }
    };
}

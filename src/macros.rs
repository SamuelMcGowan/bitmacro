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

        impl $ident
        where
            $storage: $crate::internal::Storage
        {
            #[inline]
            pub const fn empty() -> Self {
                Self(<$storage as $crate::internal::Storage>::EMPTY)
            }

            $( $crate::bitfield_accessors! { 0u32; $( $field_vis $field : $field_ty, )* } )?
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

        const _: () = {
            if <$ident as $crate::internal::BitSized>::BITS < <$storage as $crate::internal::BitSized>::BITS {
                panic!("fields  add up to less than bitfield size");
            } else if <$ident as $crate::internal::BitSized>::BITS > <$storage as $crate::internal::BitSized>::BITS {
                panic!("fields  add up to more than bitfield size");
            }
        };

        impl ::core::fmt::Debug for $ident
        where
            $storage: $crate::internal::Storage,
            $( $( $field_ty: ::core::fmt::Debug, )* )?
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut f = f.debug_struct(stringify!($ident));
                $( $( f.field(stringify!($field), &self.$field()); )* )?
                f.finish()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! bitfield_accessors {
    (
        $offset:expr;
        $field_vis:vis $field:ident : $field_ty:ty,
        $($rest:tt)*
    ) => {
        $crate::paste! {
            #[inline]
            $field_vis fn $field(&self) -> $field_ty {
                $crate::internal::Storage::extract(&self.0, $offset)
            }

            #[inline]
            $field_vis fn [< set_ $field >] (&mut self, value: $field_ty) {
                $crate::internal::Storage::insert(&mut self.0, $offset, value);
            }

            #[inline]
            $field_vis fn [< with_ $field >] (mut self, value: $field_ty) -> Self {
                self. [< set_ $field >] (value);
                self
            }
        }

        $crate::bitfield_accessors! {
            $offset + <$field_ty as $crate::internal::BitSized>::BITS as u32;
            $($rest)*
        }
    };

    ($offset:expr;) => {};
}

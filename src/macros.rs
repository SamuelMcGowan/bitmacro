#[macro_export]
macro_rules! bitfield {
    (
        $(#[$attr:meta])*
        $vis:vis struct $ident:ident : $storage:ty {
            $(
                $(
                    $(#[$field_attr:meta])*
                    $field_vis:vis $field:ident : $field_ty:ty
                ),+
                $(,)?
            )?
        }
    ) => {
        $(#[$attr])*
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        $vis struct $ident($storage);

        impl $ident
        where
            $storage: $crate::internal::Storage
        {
            #[doc=concat!("Create an empty [`", stringify!($ident), "`].\n\n")]
            #[inline]
            pub const fn empty() -> Self {
                Self(<$storage as $crate::internal::Storage>::EMPTY)
            }

            $( $crate::bitfield_accessors! {
                0u32;
                $(
                    $(#[$field_attr])*
                    $field_vis $field : $field_ty,
                )*
            } )?
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
        $(#[$field_attr:meta])*
        $field_vis:vis $field:ident : $field_ty:ty,
        $($rest:tt)*
    ) => {
        $crate::paste! {
            $(#[$field_attr])*
            #[inline]
            $field_vis fn $field(&self) -> $field_ty {
                $crate::internal::Storage::extract(&self.0, $offset)
            }

            #[doc=concat!("Set [`Self::", stringify!($field), "`].\n\n")]
            $(#[$field_attr])*
            #[inline]
            $field_vis fn [< set_ $field >] (&mut self, value: $field_ty) {
                $crate::internal::Storage::insert(&mut self.0, $offset, value);
            }

            #[doc=concat!("Return this bitfield with [`Self::", stringify!($field), "`] set.\n\n")]
            $(#[$field_attr])*
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

#[macro_export]
macro_rules! bitfield_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $ident:ident : $storage:ty {
            $(
                $(
                    $(#[$variant_attr:meta])*
                    $variant:ident = $bits:expr
                ),+
                $(,)?
            )?
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy)]
        $vis enum $ident {
            $(
                $(
                    $(#[$variant_attr])*
                    $variant = $bits,
                )*
            )?
        }

        // Exhaustiveness check.
        const _: () = {
            const NUM_VARIANTS: <$storage as $crate::internal::BitSized>::Bits = $crate::count!(0; $($($variant)*)?);

            if NUM_VARIANTS == 0 || NUM_VARIANTS - 1 < <$storage as $crate::internal::Storage>::MAX_BITS {
                panic!("non-exhaustive - all possible discriminants should be covered");
            }
        };

        impl $crate::internal::BitSized for $ident {
            type Bits = <$storage as $crate::internal::BitSized>::Bits;

            const BITS: usize = <$storage as $crate::internal::BitSized>::BITS;

            #[inline]
            #[allow(non_upper_case_globals)]
            fn from_bits(bits: Self::Bits) -> Self {
                $crate::paste! {
                    $( $(
                        const [< $variant >]: <$storage as $crate::internal::BitSized>::Bits = $bits;
                    )* )?

                    let bits = $crate::internal::BitSized::into_bits(bits);


                    match bits {
                        $( $(
                            $variant => Self::$variant,
                        )* )?

                        // Variants are exhastive.
                        _ => unreachable!(),
                    }
                }
            }

            #[inline]
            fn into_bits(self) -> Self::Bits {
                match self {
                    $( $(
                            Self::$variant => $bits,
                    )* )?
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    ($acc:expr; $tt:tt $($rest:tt)*) => {
        $crate::count!($acc + 1; $($rest)*)
    };
    ($acc:expr;) => { $acc };
}

/// Generates a brand-typed scalar id newtype around a primitive integer,
/// together with the trait impls shared by every id width.
///
/// The impls are written by hand (rather than derived) so that no `TBrand:
/// Trait` bound leaks onto the generated type, and forward to the primitive's
/// own specialized impls. Type-specific items (cross-width conversions,
/// `offset`, range converters, `IdSliceIndex`) are added by the caller in a
/// separate `impl` block.
///
/// Parameters: `$id` is the type name, `$prim` the primitive integer, `$from`
/// the constructor name, and `$to` the accessor name.
macro_rules! scalar_id {
    ($id:ident, $prim:ty, $from:ident, $to:ident) => {
        /// A brand-typed integer id. The `TBrand` type parameter makes ids
        /// built for different domains distinct types, so they cannot be mixed
        /// even though they share an integer representation.
        ///
        /// The `to_*_id` methods convert between id widths with an `as` cast,
        /// which truncates or sign-reinterprets values that do not fit the
        /// target width.
        #[repr(transparent)]
        pub struct $id<TBrand: ?Sized> {
            phantom: ::std::marker::PhantomData<TBrand>,
            repr: $prim,
        }

        impl<TBrand: ?Sized> $id<TBrand> {
            fn fmt_helper(
                self,
                fmt_repr: impl FnOnce(&$prim, &mut ::std::fmt::Formatter) -> ::std::fmt::Result,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::fmt::Result {
                use ::std::fmt::Write as _;
                $crate::internal::fmt_brand_name::<TBrand>(f)?;
                f.write_char('(')?;
                fmt_repr(&self.$to(), f)?;
                f.write_char(')')
            }

            pub const fn $from(repr: $prim) -> Self {
                Self {
                    phantom: ::std::marker::PhantomData,
                    repr,
                }
            }

            pub const fn $to(self) -> $prim {
                self.repr
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::Binary for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Binary::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::clone::Clone for $id<TBrand> {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<TBrand: ?Sized> ::std::marker::Copy for $id<TBrand> {}

        impl<TBrand: ?Sized> ::std::fmt::Debug for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Debug::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::Display for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Display::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Eq for $id<TBrand> {}

        impl<TBrand: ?Sized> ::std::convert::From<$prim> for $id<TBrand> {
            fn from(val: $prim) -> Self {
                Self::$from(val)
            }
        }

        impl<TBrand: ?Sized> ::std::str::FromStr for $id<TBrand> {
            type Err = <$prim as ::std::str::FromStr>::Err;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::$from(<$prim as ::std::str::FromStr>::from_str(s)?))
            }
        }

        impl<TBrand: ?Sized> ::std::hash::Hash for $id<TBrand> {
            fn hash<H>(&self, state: &mut H)
            where
                H: ::std::hash::Hasher,
            {
                ::std::hash::Hash::hash(&self.$to(), state)
            }

            fn hash_slice<H>(data: &[Self], state: &mut H)
            where
                H: ::std::hash::Hasher,
            {
                // SAFETY: $id is #[repr(transparent)] over $prim, so &[$id] and
                // &[$prim] share a layout.
                let data = unsafe { ::std::mem::transmute::<&[$id<TBrand>], &[$prim]>(data) };
                <$prim as ::std::hash::Hash>::hash_slice(data, state)
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::LowerExp for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::LowerExp::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::LowerHex for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::LowerHex::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::Octal for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Octal::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Ord for $id<TBrand> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                ::std::cmp::Ord::cmp(&self.$to(), &other.$to())
            }

            fn max(self, other: Self) -> Self
            where
                Self: Sized,
            {
                Self::$from(::std::cmp::Ord::max(self.$to(), other.$to()))
            }

            fn min(self, other: Self) -> Self
            where
                Self: Sized,
            {
                Self::$from(::std::cmp::Ord::min(self.$to(), other.$to()))
            }

            fn clamp(self, min: Self, max: Self) -> Self
            where
                Self: Sized,
                Self: ::std::cmp::PartialOrd,
            {
                Self::$from(::std::cmp::Ord::clamp(self.$to(), min.$to(), max.$to()))
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::PartialEq for $id<TBrand> {
            fn eq(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::eq(&self.$to(), &other.$to())
            }

            #[allow(clippy::partialeq_ne_impl)]
            fn ne(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::ne(&self.$to(), &other.$to())
            }
        }

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl<TBrand: ?Sized> ::std::cmp::PartialOrd for $id<TBrand> {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                ::std::cmp::PartialOrd::partial_cmp(&self.$to(), &other.$to())
            }

            fn lt(&self, other: &Self) -> bool {
                ::std::cmp::PartialOrd::lt(&self.$to(), &other.$to())
            }

            fn le(&self, other: &Self) -> bool {
                ::std::cmp::PartialOrd::le(&self.$to(), &other.$to())
            }

            fn gt(&self, other: &Self) -> bool {
                ::std::cmp::PartialOrd::gt(&self.$to(), &other.$to())
            }

            fn ge(&self, other: &Self) -> bool {
                ::std::cmp::PartialOrd::ge(&self.$to(), &other.$to())
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::UpperExp for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::UpperExp::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::UpperHex for $id<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::UpperHex::fmt, f)
            }
        }
    };
}

pub(crate) use scalar_id;

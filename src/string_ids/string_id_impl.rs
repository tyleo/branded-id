/// Generates a brand-typed borrowed/owned string id pair together with the
/// trait impls they share, mirroring the standard `str`/`String` relationship.
///
/// The impls are written by hand (rather than derived) so that no `TBrand:
/// Trait` bound leaks onto the generated types, and forward to the inner
/// string type's own impls. Kind-specific items (`Display`, `FromStr`, extra
/// `From` conversions, `extends` casts) are added by the caller in separate
/// `impl` blocks, because the inner types do not all support them.
///
/// Unlike the integer ids, string ids are opaque branded keys rather than
/// indices: they do not implement [`Id`](crate::Id) or [`Scalar`](crate::Scalar)
/// and cannot index a container.
///
/// Parameters: `$bid`, `$binner`, `$bfrom`, `$bas` are the borrowed id, its
/// unsized inner type, its constructor, and its accessor; `$oid`, `$oinner`,
/// `$ofrom`, `$ointo` are the owned id, its inner type, its constructor, and
/// its consuming accessor.
macro_rules! string_id_impl {
    (
        $bid:ident, $binner:ty, $bfrom:ident, $bas:ident,
        $oid:ident, $oinner:ty, $ofrom:ident, $ointo:ident $(,)?
    ) => {
        // ===================================================================
        // Borrowed half
        // ===================================================================

        /// A brand-typed, borrowed string id. The `TBrand` type parameter makes
        /// ids built for different domains distinct types, so they cannot be
        /// mixed even though they share a representation.
        ///
        /// This is the borrowed half of the owned id, related to it as `str` is
        /// to `String`: an owned id derefs and borrows to this type, so a map
        /// keyed by the owned id can be looked up by a borrowed id without
        /// allocating.
        #[repr(transparent)]
        pub struct $bid<TBrand: ?Sized> {
            phantom: ::std::marker::PhantomData<TBrand>,
            repr: $binner,
        }

        impl<TBrand: ?Sized> $bid<TBrand> {
            fn fmt_helper(
                &self,
                fmt_repr: impl FnOnce(&$binner, &mut ::std::fmt::Formatter) -> ::std::fmt::Result,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::fmt::Result {
                use ::std::fmt::Write as _;
                $crate::internal::fmt_brand_name::<TBrand>(f)?;
                f.write_char('(')?;
                fmt_repr(&self.repr, f)?;
                f.write_char(')')
            }

            /// Wraps a borrowed string slice as a branded id.
            pub const fn $bfrom(value: &$binner) -> &$bid<TBrand> {
                // SAFETY: $bid is #[repr(transparent)] over $binner and the
                // brand is a zero-sized PhantomData, so a reference to $binner
                // has the same layout (data pointer and metadata) as a
                // reference to $bid.
                unsafe { ::std::mem::transmute::<&$binner, &$bid<TBrand>>(value) }
            }

            /// Returns the underlying borrowed string slice.
            pub const fn $bas(&self) -> &$binner {
                &self.repr
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::Debug for $bid<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Debug::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::PartialEq for $bid<TBrand> {
            fn eq(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::eq(&self.repr, &other.repr)
            }

            #[allow(clippy::partialeq_ne_impl)]
            fn ne(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::ne(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Eq for $bid<TBrand> {}

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl<TBrand: ?Sized> ::std::cmp::PartialOrd for $bid<TBrand> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                ::std::cmp::PartialOrd::partial_cmp(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Ord for $bid<TBrand> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                ::std::cmp::Ord::cmp(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::hash::Hash for $bid<TBrand> {
            fn hash<H>(&self, state: &mut H)
            where
                H: ::std::hash::Hasher,
            {
                ::std::hash::Hash::hash(&self.repr, state)
            }
        }

        impl<TBrand: ?Sized> ::std::convert::AsRef<$binner> for $bid<TBrand> {
            fn as_ref(&self) -> &$binner {
                &self.repr
            }
        }

        impl<TBrand: ?Sized> ::std::borrow::ToOwned for $bid<TBrand> {
            type Owned = $oid<TBrand>;

            fn to_owned(&self) -> $oid<TBrand> {
                $oid::$ofrom(::std::borrow::ToOwned::to_owned(&self.repr))
            }
        }

        impl<'a, TBrand: ?Sized> ::std::convert::From<&'a $binner> for &'a $bid<TBrand> {
            fn from(value: &'a $binner) -> &'a $bid<TBrand> {
                $bid::$bfrom(value)
            }
        }

        // ===================================================================
        // Owned half
        // ===================================================================

        /// A brand-typed, owned string id. The owning half of the borrowed id,
        /// related to it as `String` is to `str`; it derefs and borrows to the
        /// borrowed id so it can key a map looked up by a borrowed id without
        /// allocating.
        #[repr(transparent)]
        pub struct $oid<TBrand: ?Sized> {
            phantom: ::std::marker::PhantomData<TBrand>,
            repr: $oinner,
        }

        impl<TBrand: ?Sized> $oid<TBrand> {
            fn fmt_helper(
                &self,
                fmt_repr: impl FnOnce(&$oinner, &mut ::std::fmt::Formatter) -> ::std::fmt::Result,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::fmt::Result {
                use ::std::fmt::Write as _;
                $crate::internal::fmt_brand_name::<TBrand>(f)?;
                f.write_char('(')?;
                fmt_repr(&self.repr, f)?;
                f.write_char(')')
            }

            /// Wraps an owned string as a branded id.
            pub const fn $ofrom(value: $oinner) -> $oid<TBrand> {
                $oid {
                    phantom: ::std::marker::PhantomData,
                    repr: value,
                }
            }

            /// Consumes the id and returns the underlying owned string.
            pub fn $ointo(self) -> $oinner {
                self.repr
            }
        }

        impl<TBrand: ?Sized> ::std::clone::Clone for $oid<TBrand> {
            fn clone(&self) -> Self {
                $oid::$ofrom(::std::clone::Clone::clone(&self.repr))
            }
        }

        impl<TBrand: ?Sized> ::std::fmt::Debug for $oid<TBrand> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.fmt_helper(::std::fmt::Debug::fmt, f)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::PartialEq for $oid<TBrand> {
            fn eq(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::eq(&self.repr, &other.repr)
            }

            #[allow(clippy::partialeq_ne_impl)]
            fn ne(&self, other: &Self) -> bool {
                ::std::cmp::PartialEq::ne(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Eq for $oid<TBrand> {}

        #[allow(clippy::non_canonical_partial_ord_impl)]
        impl<TBrand: ?Sized> ::std::cmp::PartialOrd for $oid<TBrand> {
            fn partial_cmp(&self, other: &Self) -> ::std::option::Option<::std::cmp::Ordering> {
                ::std::cmp::PartialOrd::partial_cmp(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::cmp::Ord for $oid<TBrand> {
            fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
                ::std::cmp::Ord::cmp(&self.repr, &other.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::hash::Hash for $oid<TBrand> {
            fn hash<H>(&self, state: &mut H)
            where
                H: ::std::hash::Hasher,
            {
                ::std::hash::Hash::hash(&self.repr, state)
            }
        }

        impl<TBrand: ?Sized> ::std::ops::Deref for $oid<TBrand> {
            type Target = $bid<TBrand>;

            fn deref(&self) -> &$bid<TBrand> {
                $bid::$bfrom(::std::convert::AsRef::<$binner>::as_ref(&self.repr))
            }
        }

        impl<TBrand: ?Sized> ::std::borrow::Borrow<$bid<TBrand>> for $oid<TBrand> {
            fn borrow(&self) -> &$bid<TBrand> {
                self
            }
        }

        impl<TBrand: ?Sized> ::std::convert::AsRef<$bid<TBrand>> for $oid<TBrand> {
            fn as_ref(&self) -> &$bid<TBrand> {
                self
            }
        }

        impl<TBrand: ?Sized> ::std::convert::AsRef<$binner> for $oid<TBrand> {
            fn as_ref(&self) -> &$binner {
                ::std::convert::AsRef::<$binner>::as_ref(&self.repr)
            }
        }

        impl<TBrand: ?Sized> ::std::convert::From<$oinner> for $oid<TBrand> {
            fn from(value: $oinner) -> $oid<TBrand> {
                $oid::$ofrom(value)
            }
        }
    };
}

pub(crate) use string_id_impl;

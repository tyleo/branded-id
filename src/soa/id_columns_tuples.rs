use crate::{
    Scalar, UsizeId,
    internal::Sealed,
    soa::{IdColumns, IdColumnsMut, IdRemap, IdStruct},
};

macro_rules! impl_id_columns_tuple {
    ($($column:ident $index:tt),+) => {
        impl<$($column: Sealed),+> Sealed for ($($column,)+) {}

        impl<TBrand: ?Sized, $($column: IdColumns<TBrand>),+> IdColumns<TBrand>
            for ($($column,)+)
        {
            type Ref<'r>
                = ($($column::Ref<'r>,)+)
            where
                Self: 'r;

            type Mut<'r>
                = ($($column::Mut<'r>,)+)
            where
                Self: 'r;

            type RawRef = ($($column::RawRef,)+);

            type RawMut = ($($column::RawMut,)+);

            fn raw_ref(&self) -> Self::RawRef {
                ($(self.$index.raw_ref(),)+)
            }

            fn raw_mut(&mut self) -> Self::RawMut {
                ($(self.$index.raw_mut(),)+)
            }

            unsafe fn row_ref<'r>(raw: Self::RawRef, id: UsizeId<TBrand>) -> Self::Ref<'r>
            where
                Self: 'r,
            {
                // SAFETY: forwarded to the caller, column by column.
                unsafe { ($($column::row_ref(raw.$index, id),)+) }
            }

            unsafe fn row_mut<'r>(raw: Self::RawMut, id: UsizeId<TBrand>) -> Self::Mut<'r>
            where
                Self: 'r,
            {
                // SAFETY: forwarded to the caller, column by column. The
                // columns are distinct fields, so their rows never alias.
                unsafe { ($($column::row_mut(raw.$index, id),)+) }
            }
        }

        impl<TBrand: ?Sized, $($column: IdColumnsMut<TBrand>),+> IdColumnsMut<TBrand>
            for ($($column,)+)
        {
            type Values = ($($column::Values,)+);

            fn retain(&mut self, id: UsizeId<TBrand>, values: Self::Values) {
                $(self.$index.retain(id, values.$index);)+
            }

            unsafe fn take(&mut self, id: UsizeId<TBrand>) -> Self::Values {
                // SAFETY: forwarded to the caller, column by column.
                unsafe { ($(self.$index.take(id),)+) }
            }

            unsafe fn take_zeroed(&mut self, id: UsizeId<TBrand>) -> Self::Values {
                // SAFETY: forwarded to the caller, column by column.
                unsafe { ($(self.$index.take_zeroed(id),)+) }
            }

            unsafe fn gc<TNum: Scalar>(&mut self, remap: &IdRemap<TBrand, TNum>) {
                // SAFETY: forwarded to the caller, column by column.
                $(unsafe { self.$index.gc(remap) };)+
            }

            unsafe fn clear<TNum: Scalar>(&mut self, ids: &IdStruct<TBrand, TNum>) {
                // SAFETY: forwarded to the caller, column by column.
                $(unsafe { self.$index.clear(ids) };)+
            }
        }
    };
}

impl_id_columns_tuple!(C0 0, C1 1);
impl_id_columns_tuple!(C0 0, C1 1, C2 2);
impl_id_columns_tuple!(C0 0, C1 1, C2 2, C3 3);
impl_id_columns_tuple!(C0 0, C1 1, C2 2, C3 3, C4 4);
impl_id_columns_tuple!(C0 0, C1 1, C2 2, C3 3, C4 4, C5 5);
impl_id_columns_tuple!(C0 0, C1 1, C2 2, C3 3, C4 4, C5 5, C6 6);
impl_id_columns_tuple!(C0 0, C1 1, C2 2, C3 3, C4 4, C5 5, C6 6, C7 7);

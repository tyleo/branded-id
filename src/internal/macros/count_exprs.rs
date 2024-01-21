macro_rules! count_exprs {
    () => { 0usize };
    ($head: expr) => { 1usize };
    ($head: expr, $($tail: expr),*) => {1usize + $crate::internal::macros::count_exprs!($($tail),*)};
}

pub(crate) use count_exprs;

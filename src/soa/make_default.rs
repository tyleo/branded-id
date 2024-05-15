pub trait MakeDefault<T> {
    fn make_default() -> T;
}

impl<T> MakeDefault<T> for T
where
    T: Default,
{
    fn make_default() -> T {
        T::default()
    }
}

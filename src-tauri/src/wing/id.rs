use crate::wing::error::WingError;

pub trait WingId: Sized {
    type Id: Ord + Into<u32>;

    const MIN_ID: Self::Id;
    const MAX_ID: Self::Id;

    fn unchecked_new(id: Self::Id) -> Self;
    fn new(value: Self::Id) -> Result<Self, WingError> {
        if value >= Self::MIN_ID && value <= Self::MAX_ID {
            Ok(Self::unchecked_new(value))
        } else {
            Err(WingError::id_out_of_bounds(
                value,
                Self::MIN_ID,
                Self::MAX_ID,
            ))
        }
    }

    fn value(&self) -> Self::Id;

    fn display(&self) -> impl std::fmt::Display + '_
    where
        Self::Id: std::fmt::Display,
    {
        struct Helper<'a, T: WingId>(&'a T);

        impl<'a, T: WingId> std::fmt::Display for Helper<'a, T>
        where
            T::Id: std::fmt::Display,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.value().fmt(f)
            }
        }

        Helper(self)
    }
}

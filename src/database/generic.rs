#[cfg(not(feature = "use_hecs"))]
pub trait Get {
    type Key: ToString;

    fn get<T>(&self, key: &Self::Key) -> Option<&T>;
}

#[cfg(feature = "use_hecs")]
pub trait Get {
    type Key: ToString;

    fn get<'a, T: Sync + Send + 'a>(&'a self, key: &Self::Key) -> Option<&'a T>;
}

pub mod generic;

#[cfg(feature = "use_hecs")]
pub mod hecs_adaptor;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no such '{0}' entry found")]
    NoSuchEntry(String),
}

impl<'a, K: ToString> From<&'a K> for Error {
    fn from(key: &'a K) -> Self {
        Self::NoSuchEntry(key.to_string())
    }
}

pub trait Like {
    type Key: ToString;
    type Value;

    fn insert(&mut self, key: Self::Key, val: Self::Value);

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    fn contains(&self, key: &Self::Key) -> bool;

    fn must_get(&self, key: &Self::Key) -> Result<&Self::Value, Error> {
        self.get(&key).ok_or(key.into())
    }

    fn must_get_mut(&mut self, key: &Self::Key) -> Result<&mut Self::Value, Error> {
        self.get_mut(&key).ok_or(key.into())
    }

    fn must_remove(&mut self, key: &Self::Key) -> Result<Self::Value, Error> {
        self.remove(&key).ok_or(key.into())
    }
}

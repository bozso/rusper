use std::{
    fmt::{Debug, Display},
};

pub mod generic;

#[derive(thiserror::Error, Debug)]
pub enum Error<Key: Debug + Display> {
    #[error("no such '{0}' entry found")]
    NoSuchEntry(Key),
}

pub trait Like {
    type Key: Display + Debug;
    type Value;

    fn insert(&mut self, key: Self::Key, val: Self::Value);

    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    fn contains(&self, key: &Self::Key) -> bool;

    fn must_get(&self, key: Self::Key) -> Result<&Self::Value, Error<Self::Key>> {
        self.get(&key).ok_or(Error::NoSuchEntry(key))
    }

    fn must_get_mut(&mut self, key: Self::Key) -> Result<&mut Self::Value, Error<Self::Key>> {
        self.get_mut(&key).ok_or(Error::NoSuchEntry(key))
    }

    fn must_remove(&mut self, key: Self::Key) -> Result<Self::Value, Error<Self::Key>> {
        self.remove(&key).ok_or(Error::NoSuchEntry(key))
    }
}


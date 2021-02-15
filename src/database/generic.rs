use std::{
    fmt::{Debug, Display},
};

use crate::database as db;

pub trait Get<T> {
    type Key: Debug + Display;

    fn get(&self, key: Self::Key) -> Option<&T>;

}

impl<T, V> db::Like for T
where
    T: Get<V>
{
    type Key = T::Key;
    type Value = V;

}

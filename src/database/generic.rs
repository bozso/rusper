use crate::database as db;

pub trait Get {
    type Key: ToString;

    fn get<T>(&self, key: Self::Key) -> Option<&T>;

}
/*
impl<V, T> db::Like for T
where
    T: Get
{
    type Key = T::Key;
    type Value = V;

    fn get(&self, key: Self::Key) -> Option<&V> {
    }
}
*/

use std::ops::Deref;
use hecs;

use crate::database as db;

struct Wrap(hecs::Entity);

impl ToString for Wrap {
    fn to_string(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl db::generic::Get for hecs::World {
    type Key = Wrap;

    fn get<'a, T>(&'a self, key: &Self::Key) -> Option<&'a T>
    where
        T: Sync + Send + 'a,
    {
        self.get::<T>(key.0).ok().map(|v| v.deref())

    }
}

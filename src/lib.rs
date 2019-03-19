use std::cmp::Ord;

mod sort;
pub mod structure;

pub trait Pair {
    type Key: Ord;
    type Value;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn set_value(&mut self, value: Self::Value);
}

impl<K: Ord, V> Pair for (K, V) {
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        &self.0
    }

    fn value(&self) -> &Self::Value {
        &self.1
    }

    fn set_value(&mut self, value: Self::Value) {
        self.1 = value
    }
}

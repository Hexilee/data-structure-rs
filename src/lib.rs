use std::cmp::{Ord, Ordering};

mod sort;
mod structure;

pub trait Pair {
    type Key: Ord;
    type Value;
    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn set_value(&mut self);
}

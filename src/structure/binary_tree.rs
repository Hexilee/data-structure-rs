use crate::Pair;
use std::cmp::max;
use std::cmp::Ordering::*;

type OptionalNode<T> = Option<Box<Node<T>>>;

pub struct BinaryTree<T: Pair> {
    root: OptionalNode<T>,
}

struct Node<T: Pair> {
    data: T,
    left: OptionalNode<T>,
    right: OptionalNode<T>,
}

impl<T: Pair> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: T) {
        match self.root {
            Some(ref mut node) => node.insert(data),
            None => self.root = Some(Node::boxed(data)),
        }
    }

    pub fn find(&self, key: &T::Key) -> Option<&T::Value> {
        match self.root {
            Some(ref node) => node.find(key),
            None => None,
        }
    }
}

impl<T: Pair> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }

    fn boxed(data: T) -> Box<Self> {
        Box::new(Self::new(data))
    }

    fn insert(&mut self, data: T) {
        match self.data.key().cmp(&data.key()) {
            Equal => self.data = data,
            Less => match self.left {
                Some(ref mut node) => node.insert(data),
                None => self.left = Some(Node::boxed(data)),
            },
            Greater => match self.right {
                Some(ref mut node) => node.insert(data),
                None => self.right = Some(Node::boxed(data)),
            },
        }
    }

    fn find(&self, key: &T::Key) -> Option<&T::Value> {
        match self.data.key().cmp(key) {
            Equal => Some(self.data.value()),
            Less => match self.left {
                Some(ref node) => node.find(key),
                None => None,
            },
            Greater => match self.right {
                Some(ref node) => node.find(key),
                None => None,
            },
        }
    }
}

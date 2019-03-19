use std::cmp::Ord;
use std::cmp::Ordering::*;

type OptionalNode<T> = Option<Box<Node<T>>>;

pub struct BinaryTree<T: Ord> {
    root: OptionalNode<T>
}

struct Node<T: Ord> {
    data: T,
    left: OptionalNode<T>,
    right: OptionalNode<T>,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, data: T) {
        match self.root {
            Some(ref mut node) => node.insert(data),
            None => self.root = Some(Node::boxed(data))
        }
    }
}

impl<T: Ord> Node<T> {
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
        match self.data.cmp(&data) {
            Equal => self.data = data,
            Less => match self.left {
                Some(ref mut node) => node.insert(data),
                None => self.left = Some(Node::boxed(data))
            },
            Greater => match self.right {
                Some(ref mut node) => node.insert(data),
                None => self.right = Some(Node::boxed(data))
            }
        }
    }
}
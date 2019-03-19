use crate::Pair;
use std::cmp::Ordering::*;

type OptionalNode<T> = Option<Box<Node<T>>>;

pub struct BinarySearchTree<T: Pair> {
    root: OptionalNode<T>,
}

struct Node<T: Pair> {
    data: T,
    left: OptionalNode<T>,
    right: OptionalNode<T>,
}

impl<T: Pair> BinarySearchTree<T> {
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

#[cfg(test)]
mod tests {
    use super::BinarySearchTree;

    #[test]
    fn find() {
        let mut tree = BinarySearchTree::new();
        tree.insert(("c", 0));
        tree.insert(("cpp", 1));
        tree.insert(("rust", 2));
        assert_eq!(&0, tree.find(&"c").unwrap());
        assert_eq!(&1, tree.find(&"cpp").unwrap());
        assert_eq!(&2, tree.find(&"rust").unwrap());
        assert_eq!(None, tree.find(&"go"));
        tree.insert(("rust", 3));
        assert_eq!(&3, tree.find(&"rust").unwrap());
    }
}

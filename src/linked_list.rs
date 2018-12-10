use std::ops::{Index, IndexMut};
use std::cmp::PartialEq;
use std::mem;

#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Box<Node<T>>,
}

#[derive(Debug)]
enum Node<T> {
    Cons(T, Box<Node<T>>),
    Nil,
}

impl<T> Node<T> {
    fn index_node(&mut self, index: usize) -> &mut Box<Node<T>> {
        match self {
            Node::Cons(_, next) => {
                match index {
                    0 => next,
                    _ => next.index_node(index - 1)
                }
            }
            Node::Nil => panic!("index out of bounds")
        }
    }

    fn index_value(&self, index: usize) -> &T {
        match self {
            Node::Cons(value, next) => {
                match index {
                    0 => value,
                    _ => next.index_value(index - 1)
                }
            }
            Node::Nil => panic!("index out of bounds")
        }
    }

    fn index_mut_value(&mut self, index: usize) -> &mut T {
        match self {
            Node::Cons(value, next) => {
                match index {
                    0 => value,
                    _ => next.index_mut_value(index - 1)
                }
            }
            Node::Nil => panic!("index out of bounds")
        }
    }

    fn last(&mut self) -> &mut Node<T> {
        match self {
            Node::Nil => self,
            Node::Cons(_, next) => next.last()
        }
    }

    fn len(&self, len: usize) -> usize {
        match self {
            Node::Nil => len,
            Node::Cons(_, next) => next.len(len + 1)
        }
    }
}

impl<T> LinkedList<T> {
    fn _index_node(&mut self, index: usize) -> &mut Box<Node<T>> {
        match index {
            0 => &mut self.head,
            _ => self.head.index_node(index - 1)
        }
    }

    fn _append(&mut self, other: Node<T>) {
        let last = self.head.last();
        *last = other
    }

    fn _insert(&mut self, index: usize, other: Box<Node<T>>) {
        let current = self._index_node(index);
        let former = mem::replace(current, other);
        self._append(*former)
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: Box::new(Node::Nil) }
    }

    pub fn len(&self) -> usize {
        self.head.len(0)
    }

    pub fn push_front(&mut self, data: T) {
        self.insert(0, data)
    }

    pub fn push_back(&mut self, data: T) {
        self._append(Node::Cons(data, Box::new(Node::Nil)))
    }

    pub fn append(&mut self, other: LinkedList<T>) {
        self._append(*other.head)
    }

    pub fn insert(&mut self, index: usize, data: T) {
        let new_node = Node::Cons(data, Box::new(Node::Nil));
        self._insert(index, Box::new(new_node))
    }

    pub fn insert_list(&mut self, index: usize, list: LinkedList<T>) {
        self._insert(index, list.head)
    }

    pub fn is_empty(&self) -> bool {
        if let Node::Nil = *self.head {
            return true;
        }
        false
    }
}

impl<T> Index<usize> for LinkedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.head.index_value(index)
    }
}

impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.head.index_mut_value(index)
    }
}

impl<T: Clone> From<&[T]> for LinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut result = LinkedList::new();
        for data in slice {
            result.push_front(data.clone())
        }
        result
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        if let Node::Nil = self {
            if let Node::Nil = other {
                return true;
            }
        }
        if let Node::Cons(data, next) = self {
            if let Node::Cons(other_data, other_next) = other {
                if data == other_data {
                    return next.eq(other_next);
                }
            }
        }
        false
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: PartialEq> PartialEq<[T]> for Node<T> {
    fn eq(&self, other: &[T]) -> bool {
        if let Node::Nil = self {
            return other.len() == 0;
        }
        if let Node::Cons(data, next) = self {
            if other.len() > 0 {
                if data == &other[0] {
                    return <Self as PartialEq<[T]>>::eq(next, &other[1..]);
                }
            }
        }
        false
    }

    fn ne(&self, other: &[T]) -> bool {
        !self.eq(other)
    }
}
//
//#[cfg(test)]
//mod tests {
//    use super::LinkedList;
//
//    #[test]
//    fn new() {
//        assert_eq!(LinkedList::<u8>::new(), LinkedList::Nil)
//    }
//
//    #[test]
//    fn default() {
//        let default_list: LinkedList<u8> = Default::default();
//        assert_eq!(default_list, LinkedList::Nil)
//    }
//
//    #[test]
//    fn push() {
//        let cons_list = LinkedList::new().push(2usize).push(1).push(0);
//        for i in 0..3 {
//            assert_eq!(i, cons_list[i]);
//        }
//    }
//
//    #[test]
//    fn len() {
//        assert_eq!(0, LinkedList::<u8>::new().len());
//        assert_eq!(3, LinkedList::new().push(2).push(1).push(0).len());
//    }
//
//    #[test]
//    fn eq_with_slice() {
//        let empty_slice: &[u8] = &[];
//        assert_eq!(&LinkedList::<u8>::new(), empty_slice);
//        let cons_list = LinkedList::new().push(3u8).push(2u8).push(1u8);
//        assert_eq!(&cons_list, &[1u8, 2, 3] as &[u8])
//    }
//
//    #[test]
//    fn from_slice() {
//        let list = LinkedList::from(&[1, 2, 3] as &[i32]);
//        assert_eq!(&list, &[3, 2, 1] as &[i32])
//    }
//
//    #[test]
//    fn is_empty() {
//        assert!(LinkedList::<u8>::new().is_empty())
//    }
//
//    #[test]
//    fn index() {
//        let cons_list = LinkedList::from(&[2usize, 1, 0] as &[usize]);
//        for i in 0..3 {
//            assert_eq!(i, cons_list[i]);
//        }
//    }
//
//    #[test]
//    fn push_back() {
//        let mut cons_list = LinkedList::new();
//        cons_list.push_back(1u8);
//        cons_list.push_back(2u8);
//        cons_list.push_back(3u8);
//        assert_eq!(&cons_list, &[1u8, 2, 3] as &[u8])
//    }
//}
//

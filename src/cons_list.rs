use std::ops::{Index, IndexMut};
use std::cmp::PartialEq;
use std::mem;

#[derive(Debug)]
pub enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    Nil,
}

impl<T> ConsList<T> {
    fn _len(&self, current: usize) -> usize {
        match self {
            ConsList::Cons(_, tail) => tail._len(current + 1),
            ConsList::Nil => current
        }
    }

    fn _index_node(&mut self, index: usize) -> (&mut T, &mut Box<ConsList<T>>) {
        match self {
            ConsList::Cons(data, next) => {
                match index {
                    0 => (data, next),
                    _ => next._index_node(index - 1)
                }
            }
            ConsList::Nil => panic!("index out of bounds")
        }
    }

    fn _append(&mut self, other: Box<ConsList<T>>) {
        let (_, next) = self._index_node(self.len() - 1);
        *next = other
    }

    fn _insert(&mut self, index: usize, other: ConsList<T>) {
        match index {
            0 => {
                let former = mem::replace(self, other);
                self.append(former)
            }
            _ => {
                let (_, current) = self._index_node(index - 1);
                let former = mem::replace(current, Box::new(other));
                current._append(former)
            }
        }
    }
}

impl<T> ConsList<T> {
    pub fn new() -> Self {
        ConsList::Nil
    }

    pub fn len(&self) -> usize {
        self._len(0)
    }

    pub fn push(self, data: T) -> Self {
        ConsList::Cons(data, Box::new(self))
    }

    pub fn push_back(&mut self, data: T) {
        self.append(ConsList::new().push(data))
    }

    pub fn append(&mut self, other: ConsList<T>) {
        self._append(Box::new(other))
    }

    pub fn insert(&mut self, index: usize, data: T) {
        self._insert(index, ConsList::new().push(data))
    }

    pub fn insert_list(&mut self, index: usize, list: ConsList<T>) {
        self._insert(index, list)
    }

    pub fn is_empty(&self) -> bool {
        if let ConsList::Nil = self {
            return true;
        }
        false
    }
}

impl<T> Index<usize> for ConsList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match self {
            ConsList::Cons(data, next) => {
                match index {
                    0 => data,
                    _ => next.index(index - 1)
                }
            }
            ConsList::Nil => panic!("index out of bounds")
        }
    }
}

impl<T> IndexMut<usize> for ConsList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (data, _) = self._index_node(index);
        data
    }
}

impl<T: Clone> From<&[T]> for ConsList<T> {
    fn from(slice: &[T]) -> Self {
        let mut result = ConsList::new();
        for data in slice {
            result = result.push(data.clone())
        }
        result
    }
}

impl<T> Default for ConsList<T> {
    fn default() -> Self {
        ConsList::Nil
    }
}

impl<T: PartialEq> PartialEq for ConsList<T> {
    fn eq(&self, other: &Self) -> bool {
        if let ConsList::Nil = self {
            if let ConsList::Nil = other {
                return true;
            }
        }
        if let ConsList::Cons(data, next) = self {
            if let ConsList::Cons(other_data, other_next) = other {
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

impl<T: PartialEq> PartialEq<[T]> for ConsList<T> {
    fn eq(&self, other: &[T]) -> bool {
        if let ConsList::Nil = self {
            return other.len() == 0;
        }
        if let ConsList::Cons(data, next) = self {
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

#[cfg(test)]
mod tests {
    use super::ConsList;

    #[test]
    fn new() {
        assert_eq!(ConsList::<u8>::new(), ConsList::Nil)
    }

    #[test]
    fn default() {
        let default_list: ConsList<u8> = Default::default();
        assert_eq!(default_list, ConsList::Nil)
    }

    #[test]
    fn push() {
        let cons_list = ConsList::new().push(2usize).push(1).push(0);
        for i in 0..3 {
            assert_eq!(i, cons_list[i]);
        }
    }

    #[test]
    fn len() {
        assert_eq!(0, ConsList::<u8>::new().len());
        assert_eq!(3, ConsList::new().push(2).push(1).push(0).len());
    }

    #[test]
    fn eq_with_slice() {
        let empty_slice: &[u8] = &[];
        assert_eq!(&ConsList::<u8>::new(), empty_slice);
        let cons_list = ConsList::new().push(3u8).push(2u8).push(1u8);
        assert_eq!(&cons_list, &[1u8, 2, 3] as &[u8])
    }

    #[test]
    fn from_slice() {
        let list = ConsList::from(&[1, 2, 3] as &[i32]);
        assert_eq!(&list, &[3, 2, 1] as &[i32])
    }

    #[test]
    fn is_empty() {
        assert!(ConsList::<u8>::new().is_empty())
    }

    #[test]
    fn index() {
        let cons_list = ConsList::from(&[2usize, 1, 0] as &[usize]);
        for i in 0..3 {
            assert_eq!(i, cons_list[i]);
        }
    }

    #[test]
    fn push_back() {
        let mut cons_list = ConsList::new();
        cons_list.push_back(1u8);
        cons_list.push_back(2u8);
        cons_list.push_back(3u8);
        assert_eq!(&cons_list, &[1u8, 2, 3] as &[u8])
    }
}


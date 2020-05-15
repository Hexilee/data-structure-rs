#[derive(Debug)]
pub struct PriorityQueue<T>(Vec<T>);

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        let former_mut = &mut self.0[i1] as *mut T;
        unsafe { std::mem::swap(&mut *former_mut, &mut self.0[i2]) }
    }

    fn sink(&mut self) {
        let mut k = 1;
        while 2 * k <= self.len() {
            let next = if 2 * k == self.len() {
                2 * k
            } else if self.0[2 * k - 1] < self.0[2 * k] {
                2 * k
            } else {
                2 * k + 1
            };
            if self.0[next - 1] >= self.0[k - 1] {
                break;
            }
            self.swap(k - 1, next - 1);
            k = next;
        }
    }

    fn swim(&mut self) {
        let mut k = self.len();
        while k / 2 >= 1 {
            if self.0[k - 1] >= self.0[k / 2 - 1] {
                break;
            }
            self.swap(k - 1, k / 2 - 1);
            k /= 2;
        }
    }

    pub fn insert(&mut self, ele: T) {
        self.0.push(ele);
        self.swim();
    }

    pub fn pop(&mut self) -> Option<T> {
        let length = self.len();
        if length <= 1 {
            self.0.pop()
        } else {
            self.swap(0, length - 1);
            let ret = self.0.pop();
            self.sink();
            ret
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Clone> Clone for PriorityQueue<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Ord> Iterator for PriorityQueue<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::PriorityQueue;
    use test_case::test_case;

    #[test]
    fn basic() {
        let mut queue = PriorityQueue::new();
        queue.insert(3);
        queue.insert(2);
        queue.insert(1);
        assert_eq!(3, queue.len());
        let min = queue.pop();
        assert!(min.is_some());
        assert_eq!(1, min.unwrap());
    }

    #[test_case(&[0; 0], &[])]
    #[test_case(&[1], &[1])]
    #[test_case(&[2, 3, 1], &[1, 2, 3])]
    #[test_case(&[4, 2, 3, 1], &[1, 2, 3, 4])]
    #[test_case(&[4, 3, 2, 2, 3, 5, 1, 5], &[1, 2, 2, 3, 3, 4, 5, 5])]
    fn iter(slice: &[i32], expect: &[i32]) {
        let mut queue = PriorityQueue::new();
        for &i in slice {
            queue.insert(i);
        }
        let ordered: Vec<_> = queue.collect();
        assert_eq!(expect, ordered.as_slice());
    }
}

#[derive(Debug)]
pub struct PriorityQueue<T>(Vec<T>);

impl<T: Ord> PriorityQueue<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        use std::mem::swap;
        let former_mut = &mut self.0[i1] as *mut T;
        unsafe { swap(&mut *former_mut, &mut self.0[i2]) }
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

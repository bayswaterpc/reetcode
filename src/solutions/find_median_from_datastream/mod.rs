use std::collections::BinaryHeap;

struct MedianFinder {
    hi: BinaryHeap<i32>,
    lo: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            hi: BinaryHeap::default(),
            lo: BinaryHeap::default(),
        }
    }

    fn add_num(&self, num: i32) {
        if self.hi.is_empty() || num > *self.hi.peek().unwrap() {
            self.hi.push(-num);
        } else {
            self.lo.push(num);
        }
        if self.hi.len() - self.lo.len() == 2 {
            self.lo.push(-self.hi.pop().unwrap());
        }
        if self.lo.len() > self.hi.len() {
            self.hi.push(-self.lo.pop().unwrap());
        }
    }

    fn find_median(&self) -> f64 {
        1.0
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

mod test {
    //#[test]
    fn unit() {
        todo!()
    }
}

// submission codes start here
use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut me = KthLargest {
            heap: BinaryHeap::with_capacity(k as usize),
        };

        for i in nums.into_iter() {
            me.add(i);
        }

        me
    }

    fn add(&mut self, val: i32) -> i32 {
        if (self.heap.len() < self.heap.capacity()) {
            self.heap.push(Reverse(val));
            (*self.heap.peek().unwrap()).0
        } else {
            let peak = (*self.heap.peek().unwrap()).0;
            if (val <= peak) {
                peak
            } else {
                self.heap.pop();
                self.heap.push(Reverse(val));
                (*self.heap.peek().unwrap()).0
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut k = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(4, k.add(3));
        assert_eq!(5, k.add(5));
        assert_eq!(5, k.add(10));
        assert_eq!(8, k.add(9));
        assert_eq!(8, k.add(4));
    }
}

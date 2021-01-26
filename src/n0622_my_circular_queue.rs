// submission codes start

struct MyCircularQueue {
    front: usize,
    rear: usize,
    size: usize,
    q: Vec<i32>,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            front: 0,
            rear: 0,
            size: 0,
            q: vec![0; k as usize],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.q[self.rear] = value;
            self.rear = (self.rear + 1) % self.q.len();
            self.size += 1;
            true
        } else {
            false
        }
    }

    fn de_queue(&mut self) -> bool {
        if !self.is_empty() {
            self.front = (self.front + 1) % self.q.len();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            let rear_index = (self.rear + self.q.len() - 1) % self.q.len();
            self.q[rear_index]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.q.capacity()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(1));
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(q.en_queue(4) == false);
        assert_eq!(3, q.rear());
        assert!(q.is_full());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(4, q.rear());
    }
}

// submission codes start

struct MyCircularDeque {
    front: usize,
    rear: usize,
    size: usize,
    q: Vec<i32>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        MyCircularDeque {
            front: 0,
            rear: 0,
            size: 0,
            q: vec![0; k as usize],
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.front = (self.front + self.q.len() - 1) % self.q.len();
            self.q[self.front] = value;

            self.size += 1;
            true
        } else {
            false
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.q[self.rear] = value;
            self.rear = (self.rear + 1) % self.q.len();

            self.size += 1;
            true
        } else {
            false
        }
    }

    fn delete_front(&mut self) -> bool {
        if !self.is_empty() {
            self.front = (self.front + 1) % self.q.len();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    fn delete_last(&mut self) -> bool {
        if !self.is_empty() {
            self.rear = (self.rear + self.q.len() - 1) % self.q.len();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.front]
        }
    }

    fn get_rear(&self) -> i32 {
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
        let mut q = MyCircularDeque::new(3);
        assert!(q.insert_last(1));
        assert!(q.insert_last(2));
        assert!(q.insert_front(3));
        assert_eq!(false, q.insert_front(4));
        assert_eq!(2, q.get_rear());
        assert!(q.is_full());
        assert!(q.delete_last());
        assert!(q.insert_front(4));
        assert_eq!(4, q.get_front());
    }
}

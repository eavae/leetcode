/**
 * [232] Implement Queue using Stacks
 *
 * Implement the following operations of a queue using stacks.
 *
 *
 * 	push(x) -- Push element x to the back of queue.
 * 	pop() -- Removes the element from in front of queue.
 * 	peek() -- Get the front element.
 * 	empty() -- Return whether the queue is empty.
 *
 *
 * Example:
 *
 *
 * MyQueue queue = new MyQueue();
 *
 * queue.push(1);
 * queue.push(2);
 * queue.peek();  // returns 1
 * queue.pop();   // returns 1
 * queue.empty(); // returns false
 *
 * Notes:
 *
 *
 * 	You must use only standard operations of a stack -- which means only push to top, peek/pop from top, size, and is empty operations are valid.
 * 	Depending on your language, stack may not be supported natively. You may simulate a stack by using a list or deque (double-ended queue), as long as you use only standard operations of a stack.
 * 	You may assume that all operations are valid (for example, no pop or peek operations will be called on an empty queue).
 *
 *
 */
pub struct Solution {}

// submission codes start here

struct MyQueue {
    vec1: Vec<i32>,
    vec2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            vec1: Vec::new(),
            vec2: Vec::new(),
        }
    }

    fn in_to_out(&mut self) {
        if (self.vec2.is_empty()) {
            while let Some(v) = self.vec1.last() {
                self.vec2.push(self.vec1.pop().unwrap());
            }
        }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.vec1.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        self.in_to_out();
        self.vec2.pop().unwrap()
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        self.in_to_out();
        *self.vec2.last().unwrap()
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.vec1.is_empty() && self.vec2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_232() {
        let mut queue = MyQueue::new();

        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1); // returns 1
        assert_eq!(queue.pop(), 1); // returns 1
        assert_eq!(queue.empty(), false); // returns false
    }
}

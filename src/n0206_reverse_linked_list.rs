/**
 * [206] Reverse Linked List
 *
 * Reverse a singly linked list.
 *
 * Example:
 *
 *
 * Input: 1->2->3->4->5->NULL
 * Output: 5->4->3->2->1->NULL
 *
 *
 * Follow up:
 *
 * A linked list can be reversed either iteratively or recursively. Could you implement both?
 *
 */
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};

// submission codes start here

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut now = head;
        let mut prev = None;
        while let Some(mut cur) = now {
            now = cur.next.take();

            cur.next = prev;
            prev = Some(cur);
        }
        prev
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 3, 4, 5]),
            linked![5, 4, 3, 2, 1]
        );
    }
}

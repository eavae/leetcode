/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative
 * integers. The digits are stored in reverse order and each of their nodes
 * contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 *
 * Example:
 *
 *
 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 */
pub struct Solution {}
use super::util::linked_list::{to_list, ListNode};

// submission codes start here

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two(l1, l2, false)
    }

    fn get_node_by_two(l1: Box<ListNode>, l2: Box<ListNode>, sup: bool) -> Option<Box<ListNode>> {
        let value = if sup {
            l1.val + l2.val + 1
        } else {
            l1.val + l2.val
        };
        let sup = value / 10 > 0;
        let value = value % 10;
        let mut node = ListNode::new(value);
        node.next = Solution::add_two(l1.next, l2.next, sup);
        Some(Box::new(node))
    }

    fn get_node_by_one(l: Box<ListNode>, sup: bool) -> Option<Box<ListNode>> {
        let value = if sup { l.val + 1 } else { l.val };
        let sup = value / 10 > 0;
        let value = value % 10;
        let mut node = ListNode::new(value);
        node.next = Solution::add_two(l.next, None, sup);
        Some(Box::new(node))
    }

    fn add_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        sup: bool,
    ) -> Option<Box<ListNode>> {
        match (l1, l2, sup) {
            (Some(l1), Some(l2), _) => Solution::get_node_by_two(l1, l2, sup),
            (Some(l1), None, _) => Solution::get_node_by_one(l1, sup),
            (None, Some(l2), _) => Solution::get_node_by_one(l2, sup),
            (None, None, true) => Some(Box::new(ListNode::new(1))),
            (None, None, false) => None,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}

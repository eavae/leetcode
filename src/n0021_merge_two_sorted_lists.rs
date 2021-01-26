/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 *
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
 *
 *
 */
pub struct Solution {}

use super::util::linked_list::{to_list, ListNode};

// submission codes start here

use std::mem::replace;
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
        let mut prev = &mut dummy_head;
        let mut lh = &l1;
        let mut rh = &l2;

        while lh.is_some() || rh.is_some() {
            let cur = match (lh, rh) {
                (Some(a), Some(b)) => {
                    if a.val > b.val {
                        replace(&mut rh, &b.next);
                        Some(ListNode::new(b.val))
                    } else {
                        replace(&mut lh, &a.next);
                        Some(ListNode::new(a.val))
                    }
                }
                (Some(a), None) => {
                    replace(&mut lh, &a.next);
                    Some(ListNode::new(a.val))
                }
                (None, Some(b)) => {
                    replace(&mut rh, &b.next);
                    Some(ListNode::new(b.val))
                }
                (_, _) => None,
            };

            if let Some(prev_box) = prev {
                if let Some(cur) = cur {
                    replace(&mut prev_box.next, Some(Box::new(cur)));
                }
                prev = &mut prev_box.next;
            }
        }

        dummy_head.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }
}

use super::util::tree::{to_tree, TreeNode};
pub struct Solution {}

// submission codes start here

use std::cell::RefCell;
use std::{ops::RangeInclusive, rc::Rc};

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        // let left = root.unwrap().borrow().left.as_ref().unwrap().borrow().val;
        Solution::traverse(root.as_ref(), &low, &high)
    }

    pub fn traverse(root: Option<&Rc<RefCell<TreeNode>>>, low: &i32, high: &i32) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let value = node.val;
            if value > *high {
                return Solution::traverse(node.left.as_ref(), low, high);
            } else if value < *low {
                return Solution::traverse(node.right.as_ref(), low, high);
            } else {
                return value
                    + Solution::traverse(node.left.as_ref(), low, high)
                    + Solution::traverse(node.right.as_ref(), low, high);
            }
        }
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            23,
            Solution::range_sum_bst(tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6], 6, 10)
        )
    }
}

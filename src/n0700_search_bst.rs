use super::util::tree::{to_tree, TreeNode};
pub struct Solution {}

// submission codes start here

use std::cell::RefCell;
use std::{ops::RangeInclusive, rc::Rc};

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::search(root.as_ref(), val)
    }

    pub fn search(root: Option<&Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let data = node.borrow();
            let cur = data.val;
            if (val > cur) {
                return Solution::search(data.right.as_ref(), val);
            } else if (val < cur) {
                return Solution::search(data.left.as_ref(), val);
            } else {
                return Some(Rc::clone(root.unwrap()));
            }
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;
}

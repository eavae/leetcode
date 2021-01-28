/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given a binary tree, return the preorder traversal of its nodes' values.
 *
 * Example:
 *
 *
 * Input: [1,null,2,3]
 *    1
 *     \
 *      2
 *     /
 *    3
 *
 * Output: [1,2,3]
 *
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */
pub struct Solution {}
use super::util::tree::{to_tree, TreeNode};

// submission codes start here

// use recursion
// use std::cell::RefCell;
// use std::rc::Rc;
// impl Solution {
//     pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//         let mut res = Vec::new();
//         Solution::helper(root.as_ref(), &mut res);
//         res
//     }

//     fn helper(root: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
//         if let Some(node) = root {
//             vec.push(node.borrow().val);
//             Solution::helper(node.borrow().left.as_ref(), vec);
//             Solution::helper(node.borrow().right.as_ref(), vec);
//         }
//     }
// }

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        let mut stack = Vec::new();
        stack.push(root);

        while !stack.is_empty() {
            if let Some(Some(node)) = stack.pop() {
                res.push(node.borrow().val);
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
            }
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_144() {
        assert_eq!(
            Solution::preorder_traversal(tree![1, null, 2, 3]),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_144_2() {
        assert_eq!(Solution::preorder_traversal(tree![3, 1, 2]), vec![3, 1, 2]);
    }
}

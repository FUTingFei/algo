use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            Solution::invert_tree(node.borrow().left.clone());
            Solution::invert_tree(node.borrow().right.clone());
        } 
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test226() {
        let tree = btree![4,2,7,1,3,6,9];
        let res = btree![4,7,2,9,6,3,1];
        assert_eq!(res, Solution::invert_tree(tree));
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            let ln = root.as_ref().unwrap().borrow().left.clone();
            let rn = root.as_ref().unwrap().borrow().right.clone();

            match (ln, rn) {
                (None, None) => 1,
                (node, None) | (None, node) => 1 + Self::min_depth(node),
                (ln, rn) => 1 + std::cmp::min(Self::min_depth(ln), Self::min_depth(rn)),
            }
        }
    }  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test111() {
        let tree1 = btree![3,9,20,null,null,15,7];
        let res = 2;
        assert_eq!(res, Solution::min_depth(tree1));
    }
}
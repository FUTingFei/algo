use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            false
        } else {
            Self::helper(root.as_ref(), sum)
        }
    }

    fn helper(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(c) = root {
            let rest = sum - c.borrow().val;
            if c.borrow().left.is_none() && c.borrow().right.is_none() {
                rest == 0
            } else {
                Solution::helper(c.borrow().left.as_ref(), rest) ||
                Solution::helper(c.borrow().right.as_ref(), rest)
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test112  () {
        let tree1 = btree![5,4,8,11,null,13,4,7,2,null,null,null,1];
        let num = 22;
        assert_eq!(true, Solution::has_path_sum(tree1,num));
    }
}
use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        let mut res = 0;
        Solution::helper2(root,sum,&mut res);
        res
    }

    fn helper2(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, res:&mut i32) {
        let mut target = 0;
        Solution::helper(root.clone(), sum, &mut target, res);

        if let Some(node) = root.clone() {
            
        }
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, sum:i32, target: &mut i32, count:&mut i32) {
        if let Some(node) = root.clone() {
            *target += node.borrow().val;
            if *target == sum {
                *count += 1;
            }

            if let Some(l) = node.borrow().left.clone() {
                Solution::helper(Some(l), sum, target, count);
            }

            if let Some(r) = node.borrow().right.clone() {
                Solution::helper(Some(r), sum, target, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test437() {
        let root = btree![10,5,-3,3,2,null,11,3,-2,null,1];
        assert_eq!(3, Solution::path_sum(root, 8));
    }
}
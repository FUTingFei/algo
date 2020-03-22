use std::rc::Rc;
use std::cell::RefCell;
use leetcode_prelude::btree;
use leetcode_prelude::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut resv: Vec<i32> = Vec::new();
        
        Solution::helper(root, &mut resv, false);

        resv.iter().sum()
    }

    fn helper(root: Option<Rc<RefCell<TreeNode>>>, resv:&mut Vec<i32>, left: bool) {
        
        if let Some(node) = root.clone() {

            match (node.borrow().left.clone(), node.borrow().right.clone()) {
                (Some(l), Some(r)) => {
                    Solution::helper(Some(l), resv, true);
                    Solution::helper(Some(r), resv, false);
                },
                (Some(l), _) => {
                    Solution::helper(Some(l), resv, true);
                },
                (_, Some(r)) => {
                    Solution::helper(Some(r), resv, false);
                },
                _ => {
                    if left {
                        resv.push(node.borrow().val);
                    }
                }
            }
        
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test404() {
        let tree = btree![1,2];
        assert_eq!(2, Solution::sum_of_left_leaves(tree));
    }
}
pub use leetcode_prelude::TreeNode;
pub use leetcode_prelude::btree;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        if let Some(r) = root {
          if let Ok(v) = Rc::try_unwrap(r) {
            let s = v.into_inner();

            let left_node = s.left;
            let right_node = s.right;

            let left = Solution::max_depth(left_node);
            let right = Solution::max_depth(right_node);

            if left > right {
              return left + 1;
            } else {
              return right + 1;
            }
            
          }
        }

        0
    }
 
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
      Solution::valid(root, None, None)
    }

    fn valid(root: Option<Rc<RefCell<TreeNode>>>, lo: Option<i32>, hi: Option<i32>) -> bool {
      
      if root.is_none() {
        return true;
      }

      let node = root.unwrap();
      match (lo, hi) {
        (None, None) => {},
        (None, Some(ref x)) => {
          if node.as_ref().borrow().val >= *x {
            return false;
          }
        },
        (Some(ref x), None) => {
          if node.as_ref().borrow().val <= *x {
            return false;
          }
        },
        (Some(ref l), Some(ref h)) => {
          if !(node.as_ref().borrow().val > *l && node.as_ref().borrow().val < *h) {
            return false;
          }
        }
      }

      if !Solution::valid(
        node.as_ref().borrow().left.clone(), 
        lo, 
        Some(node.as_ref().borrow().val)) {
          return false;
        }

      if !Solution::valid(
        node.as_ref().borrow().right.clone(), 
        Some(node.as_ref().borrow().val),
        hi) {
          return false;
        }

      true
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
          return false;
        }
        let mut mid_vec: Vec<i32> = Vec::new();
        let temp = mid_vec.clone();
        Solution::get_mid(root, &mut mid_vec);
        mid_vec.reverse();
        let n = mid_vec.len();
        for i in 0..n {
          if mid_vec[i] != temp[i] {
            return false;
          }
        }
        true
    }

    fn get_mid(root: Option<Rc<RefCell<TreeNode>>>, mid_vec: &mut Vec<i32>) {
      if let Some(r) = root {
        Solution::get_mid(r.as_ref().borrow().left.clone(), mid_vec);
        mid_vec.push(r.as_ref().borrow().val);
        Solution::get_mid(r.as_ref().borrow().right.clone(), mid_vec);
      }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_max_depth() {
      let tree = btree![3,9,20,null,null,15,7];
      assert_eq!(Solution::max_depth(tree), 3);
    }

    #[test]
    fn test_is_valid_bst() {
      let true_bst = btree![10,5,15,null,null,6,20];
      assert_eq!(Solution::is_valid_bst(true_bst), true);
    }

    #[test]
    fn test_symmetric() {
      let symmetric = btree![1,2,2,3,4,4,3];
    }
}
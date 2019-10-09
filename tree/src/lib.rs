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

      fn get_node(ori: Option<Rc<RefCell<TreeNode>>>) -> Option<TreeNode> {
        let mut s = None;
        if let Some(r) = ori {
          if let Ok(v) = Rc::try_unwrap(r) {
            s = Some(v.into_inner());
          }
        }
        s
      }

      let c_node = get_node(root);
      if c_node != None {
        let c_node = c_node.unwrap();
        let l_node = get_node(c_node.left);
        let r_node = get_node(c_node.right);
      }

      // if !( (l_val < c_val) && (r_val > c_val) ) {
      //   return false;
      // }
      
      // Solution::is_valid_bst(Some(Rc::new(RefCell::new())));
      // Solution::is_valid_bst(Some(Rc::new(RefCell::new(l_node))));

      true
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
      let true_bst = btree![0];
      assert_eq!(Solution::is_valid_bst(true_bst), true);

      // let false_bst = btree![5,1,4,null,null,3,6];
      // assert_eq!(Solution::is_valid_bst(false_bst), false);
    }
}
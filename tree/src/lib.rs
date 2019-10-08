pub use leetcode_prelude::TreeNode;
pub use leetcode_prelude::btree;

use std::rc::Rc;
use std::cell::RefCell;


struct Solution {}

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
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_max_depth() {
      let tree = btree![3,9,20,null,null,15,7];
      assert_eq!(Solution::max_depth(tree), 3)
    }
}
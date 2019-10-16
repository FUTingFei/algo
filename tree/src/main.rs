use tree::Solution;
pub use leetcode_prelude::TreeNode;
pub use leetcode_prelude::btree;

fn main() {
    let bst = btree![1,2,2,2,null,2];
    let res = Solution::is_symmetric(bst);
    println!("{:?}", res);
}
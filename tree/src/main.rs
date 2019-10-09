use tree::Solution;
pub use leetcode_prelude::TreeNode;
pub use leetcode_prelude::btree;


fn main() {
    let bst = btree![2,1,3];
    let res = Solution::is_valid_bst(bst);
    println!("{:?}", res);
}
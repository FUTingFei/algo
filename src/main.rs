use algo;
pub use leetcode_prelude::ListNode;
pub use leetcode_prelude::linkedlist;
pub use leetcode_prelude;

fn main() {
    let list = linkedlist![1,1,1,2];
    algo::n083delete_duplicates::Solution::delete_duplicates(list);
}
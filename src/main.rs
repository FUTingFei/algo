use algo;
use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;
mod n203remove_elements;


fn main() {
    let list = linkedlist![1,2,6,3,4,5,6];
    let r = algo::n203remove_elements::Solution::remove_elements(list, 6);
}
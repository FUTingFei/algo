use algo;
use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;
mod n443compress;


fn main() {
    let mut chars = vec!['a','a','b','b','c','c','c'];
    algo::n443compress::Solution::compress(&mut chars);
}
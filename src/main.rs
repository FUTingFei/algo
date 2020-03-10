use algo;
use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;
pub mod n002add_two_numbers;


fn main() {
    let list1 = linkedlist![2,4,3];
    let list2 = linkedlist![5,6,4];
    let res = algo::n002add_two_numbers::Solution::add_two_numbers(list1,list2);
    println!("{:?}", res);
}
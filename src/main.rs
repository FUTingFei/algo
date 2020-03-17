use algo;
use leetcode_prelude::ListNode;
use leetcode_prelude::linkedlist;
use leetcode_prelude;
mod n383can_construct;


fn main() {
    assert_eq!(false, algo::n383can_construct::Solution::can_construct("a".to_owned(), "b".to_owned()));
    assert_eq!(false, algo::n383can_construct::Solution::can_construct("aa".to_owned(), "ab".to_owned()));
    assert_eq!(true, algo::n383can_construct::Solution::can_construct("aa".to_owned(), "aab".to_owned()));
}
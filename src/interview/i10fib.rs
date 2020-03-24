pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut v:Vec<i128> = Vec::new();
        v.push(0);
        v.push(1);
        for i in 2..n+1 {
            v.push(v[i as usize -1] + v[i as usize - 2]);
        }
        if n == 0 {
            return 0;
        }
        (v.pop().unwrap() % 1000000007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i10() {
        assert_eq!(807526948, Solution::fib(48));
        assert_eq!(5, Solution::fib(5));
    }   
}
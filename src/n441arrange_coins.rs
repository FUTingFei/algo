pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut res = 0;
        let mut n = n;
        let mut i = 0;
        loop {
            i += 1;
            n = n - i;
            if n < 0 {
                res = i - 1;
                break;
            } else if n == 0 {
                res = i;
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test441() {
        assert_eq!(2, Solution::arrange_coins(5));
        assert_eq!(3, Solution::arrange_coins(8));
    }   
}
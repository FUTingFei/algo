pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;

        while n > 0 {
            n /= 5;
            count += n;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test172() {
        assert_eq!(0, Solution::trailing_zeroes(3));
        assert_eq!(1, Solution::trailing_zeroes(5));
    }
}
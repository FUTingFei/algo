pub struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;

        while num >= 10 {
            num = Solution::get_sum(num);
        }

        num
    }

    fn get_sum(num: i32) -> i32 {
        let mut num = num;
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num = num / 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test258() {
        assert_eq!(5, Solution::add_digits(32));
    }
}
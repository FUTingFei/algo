pub struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
       let mut num = num;

       while num != 0 && num % 4 == 0 {
           num /= 4;
       }

       num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test342() {
        assert_eq!(true, Solution::is_power_of_four(16));
        assert_eq!(false, Solution::is_power_of_four(5));
    }   
}
pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        let mut n = n;

        if n <= 0 { return false };

        while n != 0 {
            if n == 1 {
                return true;
            } else {
                if n % 2 == 1 {
                    return false;
                } else {
                    n = n / 2;
                }
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test231() {
        assert_eq!(true, Solution::is_power_of_two(2));
        assert_eq!(true, Solution::is_power_of_two(16));
        assert_eq!(false, Solution::is_power_of_two(218));
    }
}
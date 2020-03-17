pub struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num < 2 {
            return true;
        }

        let mut left = 2;
        let mut right = num/2;

        while left <= right {
            let mid = left + (right - left)/2;
            let x:i128 = mid as i128 * mid as i128;

            if num as i128 == x {
                return true;
            }
            
            if num as i128 > x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }

        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test367() {
        assert_eq!(true, Solution::is_perfect_square(808201));
        assert_eq!(false, Solution::is_perfect_square(14));
    }   
}
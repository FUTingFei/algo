pub struct Solution {}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        let mut num = num;
        if num <= 0 { return false; }
        if num == 1 { return true; }

        while num != 1 {
            if (num/2)*2 == num {
                num = num/2;
            } else if (num/3)*3 == num  {
                num = num/3;
            } else if (num/5)*5 == num {
                num = num/5;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test263() {
        assert_eq!(true, Solution::is_ugly(1));
        assert_eq!(true, Solution::is_ugly(8));
        assert_eq!(false, Solution::is_ugly(14));
    }
}
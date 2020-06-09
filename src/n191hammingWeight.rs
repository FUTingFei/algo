pub struct Solution {}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut count = 0;
        let mut flag: u32 = 1;
        
        while flag != 0 {
            if n & flag != 0 {
                count += 1;
            }

            flag = flag << 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test191() {
        assert_eq!(2, Solution::hammingWeight(9));
    }
}
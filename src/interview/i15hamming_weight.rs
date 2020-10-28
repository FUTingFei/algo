pub struct Solution {}

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut count:i32 = 0;
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

    pub fn i15() {
        assert_eq!(2, Solution::hamming_weight(9));
    }
}
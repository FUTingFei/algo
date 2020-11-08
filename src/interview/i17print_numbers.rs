pub struct Solution {}

impl Solution {
    pub fn print_numbers(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for i in 1..((10 as i32).pow(n as u32)) {
            res.push(i);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i17() {
        let answer = vec![1,2,3,4,5,6,7,8,9];
        assert_eq!(Solution::print_numbers(1), answer);
    }
}
pub struct Solution {}

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let mut numbers = numbers;
        numbers.sort();
        numbers[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i11() {
        assert_eq!(1, Solution::min_array(vec![3,4,5,1,2]));
        assert_eq!(0, Solution::min_array(vec![2,2,2,0,1]));
    }   
}
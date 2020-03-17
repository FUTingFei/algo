struct NumArray {
    n: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        NumArray { n:nums }
    }
    
    fn sum_range(&mut self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        let nums = &self.n;
        nums[i..=j].iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test303() {
        let nums = vec![1,2,3,4,5];
        let mut obj = NumArray::new(nums);
        assert_eq!(10, obj.sum_range(0, 3));
    }
}

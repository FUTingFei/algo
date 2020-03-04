pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![];
        
        for i in 0..=row_index as usize {
            res.push(1);
            for j in (1..i as usize).rev() {
                res[j] += res[j-1];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test119() {
        let res = vec![1,3,3,1];
        assert_eq!(res, Solution::get_row(3));
    }
}
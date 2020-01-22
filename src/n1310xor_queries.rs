pub struct Solution {}
    
impl Solution {
    // 1310
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let n = queries.len();

        for i in 0..n {
            let s = queries[i as usize][0];
            let e = queries[i as usize][1];
            
            let mut a = 0;
            for j in s..=e {
                a = a ^ arr[j as usize];
            }
            
            result.push(a);
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr: Vec<i32> = vec![1,3,4,8];
        let queries:Vec<Vec<i32>> = vec![vec![0,1],vec![1,2],vec![0,3],vec![3,3]];
        let res = Solution::xor_queries(arr, queries);
        let c:Vec<i32> = vec![2,7,14,8];
        assert_eq!(c, res);
    }
}
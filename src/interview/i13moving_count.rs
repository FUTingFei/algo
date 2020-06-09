pub struct Solution {}

impl Solution {
    // to be continued
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        if m <= 0 || n <= 0 || k < 0 {
            return 0;
        }

        let mut visited: Vec<Vec<bool>> = vec![vec![false; n as usize]; m as usize];

        Solution::core(m,n,0,0,k,&mut visited)
    }

    fn core(m: i32, n: i32, i: i32, j: i32, k: i32, visited: &mut Vec<Vec<bool>>) -> i32 {
        let mut res = 0;

        if Solution::check(m,n,i,j,k,visited) {
            visited[i as usize][j as usize] = true;

            res = 1 + Solution::core(m,n,i + 1,j,k,visited)
                    + Solution::core(m,n,i - 1,j,k,visited)
                    + Solution::core(m,n,i,j + 1,k,visited)
                    + Solution::core(m,n,i,j - 1,k,visited);
        }

        res
    }

    fn check(m: i32, n: i32, i: i32, j: i32, k: i32, visited: &mut Vec<Vec<bool>>) -> bool {
        
        if i >= 0 && i < m && j >= 0 && j < n 
        && Solution::get_digit_sum(i) + Solution::get_digit_sum(j) <= k  
        && !visited[i as usize][j as usize]{
            return true;
        }
           
        false
    }

    fn get_digit_sum(num: i32) -> i32 {
        let mut num = num;
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i13() {
        assert_eq!(0, Solution::moving_count(1, 2, 1));
        // assert_eq!(1, Solution::moving_count(3, 1, 0));
    }   
}
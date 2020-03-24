pub struct Solution {}

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        if n == 0 {
            return false;
        }
        let m = matrix[0].len();
        if m == 0 {
            return false;
        }

        for i in 0..n {
            if target >= matrix[i][0] && target <=  matrix[i][m-1] {
                let mut l = 0;
                let mut r = m-1;
                while l <= r {
                    let mid = (l+r)/2;
                    if matrix[i][mid] > target {
                        r = mid - 1;
                    } else if matrix[i][mid] < target {
                        l = mid + 1;
                    } else {
                        return true;
                    }
                }
            }
        }        

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i04() {
        let matrix = vec![
                        vec![1,  4,  7,  11, 15],
                        vec![2,  5,  8,  12, 19],
                        vec![3,  6,  9,  16, 22],
                        vec![10, 13, 14, 17, 24],
                        vec![18, 21, 23, 26, 30]
                    ];

        assert_eq!(true, Solution::find_number_in2_d_array(matrix,5));

        let matrix = vec![
                        vec![]
                    ];
        assert_eq!(false, Solution::find_number_in2_d_array(matrix,1));
    }   
}
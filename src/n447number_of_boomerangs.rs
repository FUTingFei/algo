pub struct Solution {}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
        
        let len = points.len();
        for i in 0..len {
            for j in i+1..len {
                let d = Solution::get_sqr(&points[i], &points[j]);
                if map.contains_key(&d) {
                    map.get_mut(&d).unwrap().push(vec![i as i32, j as i32]);
                } else {
                    map.insert(d, vec![vec![i as i32, j as i32]]);
                }
            }
        }

        let mut res = 0;
        for (_, vv) in map.iter() {
            for i in 0..vv.len() {
                for j in i+1..vv.len() {
                    if vv[i][0] == vv[j][0] 
                    || vv[i][0] == vv[j][1]
                    || vv[i][1] == vv[j][0]
                    || vv[i][1] == vv[j][1] {
                        res += 2;
                    }
                }
            }
        }

        res
    }

    fn get_sqr(a:& Vec<i32>, b: & Vec<i32>) -> i32 {
        let mut sum:i32 = 0;
        
        for i in 0..2 {
            sum += ((a[i]-b[i]).abs())*((a[i]-b[i]).abs());
        }
        
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test447() {
        let points = vec![vec![1,1],vec![2,2],vec![3,3]];
        assert_eq!(2, Solution::number_of_boomerangs(points));
    }   
}
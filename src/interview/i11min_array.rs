pub struct Solution {}

impl Solution {
    pub fn min_array(numbers: Vec<i32>) -> i32 {
        let mut s = 0;
        let mut e = numbers.len() - 1;
        let mut mid = s;
        
        while numbers[s] >= numbers[e] {

            if e - s == 1 {
                mid = e;
                break;
            }

            mid = (s + e)/2;

            if numbers[s] == numbers[e] && numbers[s] == numbers[mid] {
                return Solution::in_order(&numbers, s, e);
            }

            if numbers[mid] >= numbers[s] {
                s = mid;
            } else if numbers[mid] <= numbers[e] {
                e = mid;
            }

        }

        numbers[mid]
    }

    fn in_order(v: &Vec<i32>, s: usize, e: usize) -> i32 {
        let mut res = v[s];
        for item in v.iter() {
            if res > *item {
                res = *item;
            }
        }
        res
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
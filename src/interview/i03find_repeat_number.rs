pub struct Solution {}

impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        
        for i in 0..nums.len() {
            if nums[i] != i as i32 {
                if nums[i] == nums[nums[i] as usize] {
                    return nums[i];
                } else {
                    let j = nums[i];
                    nums.swap(i,  j as usize);
                }
            }
        }

        0
    }

    
    // time: O(nlogn) , space: O(1), 不修改原数组
    // pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
    //     let mut s = 1;
    //     let mut e = nums.len() - 1;

    //     while e >= s {
    //         let mid = ((e-s)>>1) + s;
    //         let c = Solution::count(&nums, s, mid);

    //         if e == s {
    //             if c > 1 {
    //                 return s as i32;
    //             } else {
    //                 break;
    //             } 
    //         }

    //         if c > (mid - s + 1) as i32 {
    //             e = mid;
    //         } else {
    //             s = mid + 1;
    //         }

    //     }

    //     -1

    // }

    // fn count(nums: &Vec<i32>, s: usize, e: usize) -> i32 {
    //     let mut res = 0;

    //     for elem in nums {
    //         if *elem >= s as i32 && *elem <= e as i32 {
    //             res += 1;
    //         }
    //     }

    //     res
    // }



    // time: O(n) , space: O(1)
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn i03() {
        assert_eq!(2, Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]));
    }   
}
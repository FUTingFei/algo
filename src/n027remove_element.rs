pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    
    let len = nums.len();
    let mut count = 0;
    let mut i = 0;

    loop {
        if i == nums.len() {
            break;
        }

        if nums[i] == val {
            count += 1;
            nums.remove(i);
            i -= 1;
        }
        
        i += 1;
    }

    len as i32 - count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums2 = vec![0,1,2,2,3,0,4,2];
        let val2 = 2;
        let len2 = remove_element(&mut nums2, val2);
        assert_eq!(5, len2);
    }
}
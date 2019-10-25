fn main() {
    let nums = vec![1];
    let res = missing_number(nums);
    println!("{}", res);
}

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let len = nums.len();
    let mut flag = nums[len as usize - 1];
    let mut i = len;
    if nums[0] != 0 {
        return 0;
    }
    while i > 0 {
        if nums[i as usize - 1] != flag {
            return flag;
        } else {
            i -= 1;
            flag -= 1;
        }
    }
    nums[len as usize - 1] + 1
}
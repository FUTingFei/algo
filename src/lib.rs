pub mod quicksort {
    pub fn quicksort_lomuto(arr: &mut [i32]) {
        let hi = arr.len() as isize - 1;
        quicksort_helper(arr, 0, hi);
    }

    fn quicksort_helper(arr: &mut [i32], lo: isize, hi: isize) {
        if lo <= hi {
            let pivot = partition(arr, lo, hi);
            quicksort_helper(arr, lo, pivot - 1);
            quicksort_helper(arr, pivot + 1, hi);
        }
    }

    fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
        
        let pivot = arr[hi as usize];
        let mut i = lo;

        for j in lo..hi {
            if arr[j as usize] < pivot {
                arr.swap(i as usize, j as usize);
                i += 1;
            }
        }

        arr.swap(i as usize, hi as usize);
        i
    }
}

pub mod array {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        
        let old_len = nums.len();
        nums.retain(|x| *x != 0);
        let new_len = nums.len();
        let s = old_len - new_len;

        let mut i = 0;

        while i < s {
            nums.push(0);
            i += 1;
        }

    }

    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut nums = digits.clone();
        let len = nums.len();
        let mut i = len - 1;

        while i >= 0 {
            if nums[i] < 9 {
                nums[i] = nums[i] + 1;
                return nums;
            }
            nums[i] = 0;
            if i > 0{
                i = i - 1;
            } else {
                break;
            }
        }

        nums.insert(0, 1);

        nums
    }

    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut result: Vec<i32> = Vec::new();

        let mut i = 0;
        let mut j = 0;
        
        let mut n1 = nums1.clone();
        let mut n2 = nums2.clone();
        
        n1.sort();
        n2.sort();

        while i < len1 && j < len2 {
            if n1[i] == n2[j] {
                result.push(n1[i]);
                i += 1;
                j += 1;
            } else if n1[i] > n2[j] {
                j += 1;
            } else {
                i += 1;
            }
        }

        result   
    }

    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut output = 0;

        for val in nums {
            output = output ^ val;
        }

        output        
    }

    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut i = 0;
        for val in &nums {
            let j = i + 1;
            for res in &nums[j..] {
                if val == res {
                    return true;
                }
            }
            i += 1;
        }
        false
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32){
        let mut i = 0;

        while i < k {
            let out = nums.pop().unwrap();
            nums.insert(0, out);
            i += 1;
        }
    }
    
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 0 {
            return 0;
        }
        let mut money = 0;
        let mut buy = prices[0];

        for price in prices {
            if buy < price {
                money += price - buy;
            }
            buy = price;
        }

        money
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i = 0;

        while i < nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}

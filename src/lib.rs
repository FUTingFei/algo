pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m;
    let mut n = n;

    if m == 0 {
        let temp = nums2.clone();
        nums1.clear();
        for t in temp {
            nums1.push(t);
        }
        return;
    }

    while n > 0 {
        if m == 0 {
            nums1[(m + n) as usize - 1] = nums2[n as usize - 1];
            n -= 1;
        } else {
            if nums1[m as usize - 1] >= nums2[n as usize - 1] {
                nums1[(m + n) as usize - 1] = nums1[m as usize - 1];
                m -= 1;
            } else {
                nums1[(m + n) as usize - 1] = nums2[n as usize - 1];
                n -= 1;
            }
        }
        
    }
    
}
pub fn binary_search( arr: &[i32], v: i32 ) -> i32 {
    let mut lo = 0;
    let mut hi = arr.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) >> 1; 
        if arr[mid] == v {
            return mid as i32;
        } else if arr[mid] < v {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1
}
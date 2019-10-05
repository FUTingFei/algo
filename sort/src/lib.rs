pub fn bubble_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len <= 1 {
        println!("The arr don't need to be sorted!");
    }

    for i in 0..len {
        let mut flag = false;
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = true;
            }
        }
        if !flag {
            break;
        }
    }
}

pub fn insert_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len <= 1 {
        println!("The arr don't need to be sorted!");
    }

    for i in 1..len {
        let value = arr[i];
        let mut j = (i - 1) as isize;

        while j >= 0 {
            if arr[j as usize] > value {
                arr[j as usize +1] = arr[j as usize];
            } else {
                break;
            }
            j -= 1;
        }

        arr[(j+1) as usize] = value;
    }
}

pub fn select_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    if len <= 1 {
        println!("The arr don't need to be sorted!");
    }

    for i in 0..len {
        let mut res = arr[i];
        let mut r = i;

        for j in i..len {
            if arr[j] < res {
                res = arr[j];
                r = j;
            }
        }

        arr.swap(i, r);
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let hi = arr.len() as isize - 1;
    quick_sort_helper(arr, 0, hi);
}

fn quick_sort_helper(arr: &mut [i32], lo: isize, hi: isize) {
    if lo < hi {
        let pivot = partition(arr, lo, hi);
        quick_sort_helper(arr, lo, pivot - 1);
        quick_sort_helper(arr, pivot + 1, hi);
    }
}

fn partition(arr: &mut [i32], lo: isize, hi: isize) -> isize {
    let pivot_val = arr[hi as usize];
    let mut i = lo;

    for j in lo..hi {
        if arr[j as usize] < pivot_val {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }

    arr.swap(i as usize, hi as usize);
    i
}
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

    
}
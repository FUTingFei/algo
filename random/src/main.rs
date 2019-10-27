fn main() {
    let res = consecutive_numbers_sum(15);
    println!("{}", res);
}

pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut res = 1;

    let mut i = 1;
    while i < n/2 + 1 {
        if is_count(i, i, n) {
            res += 1;
        }
        i += 1;
    }

    res
}

fn is_count(a: i32, b: i32, n: i32) -> bool {
    let g = go_next(a, b);

    if g == n {
        return true;
    } else if g < n {
        return is_count(g, b+1, n);
    }

    false
}

fn go_next(a: i32, b: i32) -> i32 {
    a + b + 1
}
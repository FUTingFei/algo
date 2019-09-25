use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    println!("{}", f(8, &mut hm));
}

fn f(n: i32, hm: &mut HashMap<i32, i32>) -> i32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    if hm.contains_key(&n) {
        return *hm.get(&n).unwrap();
    }
    let ret = f(n - 1, hm) + f(n - 2, hm);
    hm.insert(n, ret);
    ret
}
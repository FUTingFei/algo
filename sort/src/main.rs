use sort;

fn main() {
    let mut arr = vec![2,1,3,5,8,23,9];
    sort::quick_sort(&mut arr);
    println!("{:?}", arr);
}
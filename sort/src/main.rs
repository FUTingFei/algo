use sort;

fn main() {
    let mut arr = vec![3,45,11,6,14,1,8];
    sort::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
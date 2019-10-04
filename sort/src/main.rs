use sort;

fn main() {
    let mut arr = vec![2,1];
    sort::insert_sort(&mut arr);
    println!("{:?}", arr);
}
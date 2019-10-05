use search;

fn main() {
    let arr = vec![1,2,3,4,5,6,7,8,9,10];
    let res = search::binary_search(&arr, 5);
    println!("{}", res);
}

// use algo::quicksort::quicksort_lomuto;
// use algo::array;
use algo::my_link_list::List;

fn main() {
    let mut my_list = List::<u8>::new();
    my_list.push(0);
    my_list.push(1);
    my_list.push(2);
    println!("{:?}", my_list.len());
    println!("{:?}", my_list.get_last_value());
    my_list.pop();
    println!("{:?}", my_list);
}
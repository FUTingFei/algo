// use algo::quicksort::quicksort_lomuto;
use algo::array;

fn main() {
    let mut nums = vec![0,1,0,3,12,0];
    array::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
mod my_binary_tree;

fn main() {
   let mut nums1 = vec![5,0];
   let m = 1;
   let mut nums2 = vec![1,];
   let n = 1;
   
   algo::merge(&mut nums1, m, &mut nums2, n);
   println!("{:?}", nums1);
}
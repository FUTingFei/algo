use algo::stack_algo;

fn main() {
    let mut s = stack_algo::ArrayQueue::new(4);
    s.enqueue("just test".to_string());
    s.enqueue("just test 2".to_string());
    println!("{:?}", s);
    s.dequeue();
    println!("{:?}", s);
}
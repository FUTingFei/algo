use algo::string_algo;

fn main() {
    let s = "0P".to_string();
    let r = string_algo::is_palindrome(s);
    println!("{}", r);

}
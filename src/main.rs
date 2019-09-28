use algo::string_algo;

fn main() {
    let s = "aaa".to_string();
    let t = "aaaa".to_string();
    let r = string_algo::str_str(s, t);
    println!("{}", r);
}
use algo::string_algo;

fn main() {
    let s = "cc".to_string();
    let r = string_algo::first_uniq_char(s);
    println!("{}", r);
}
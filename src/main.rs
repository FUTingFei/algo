use algo::string_algo;

fn main() {
    let s = "  -4193 with words".to_string();
    let r = string_algo::my_atoi(s);
    println!("{}", r);

}
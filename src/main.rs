mod string_algo;

fn main() {
    let s: Vec<String> = vec!["lower".to_string(),"flow".to_string(),"flight".to_string()];
    let r = string_algo::string_algo::longest_common_prefix(s);
    println!("{}", r);
}
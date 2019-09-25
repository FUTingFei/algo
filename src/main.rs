use algo::string_algo;

fn main() {
    let s = "anagram".to_string();
    let t = "nagaeam".to_string();

    let r = string_algo::is_anagram(s, t);
    println!("{}", r);

}
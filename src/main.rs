mod string_algo;

fn main() {
    let s = 5;
    let r = string_algo::string_algo::count_and_say(s);
    println!("{}", r);
}
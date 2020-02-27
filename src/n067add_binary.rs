pub fn add_binary(a: String, b: String) -> String {
    let a = i128::from_str_radix(a.as_str(), 2).unwrap();
    let b = i128::from_str_radix(b.as_str(), 2).unwrap();
    format!("{:b}", a+b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = "11".to_string();
        let b = "1".to_string();
        let res = "100".to_string();
        assert_eq!(res, add_binary(a, b));
    }
}
pub fn climb_stairs(n: i32) -> i32 {
    let mut arr:Vec<i32> = Vec::with_capacity(n as usize + 1);
    arr.push(1);
    arr.push(1);

    for i in 2..=n {
        let temp = arr[i as usize - 1] + arr[i as usize - 2];
        arr.push(temp);
    }

    arr[n as usize] as i32
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_climb_stairs() {
        let res = climb_stairs(3);
        assert_eq!(res, 3);

        let res = climb_stairs(4);
        assert_eq!(res, 5);
    }
}
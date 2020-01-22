pub struct Solution {}

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut bits = bits;
        bits.reverse();

        while bits.len() > 1 {
            let n = bits.len() - 1;
            if bits[n] == 1 {
                bits.pop();
                bits.pop();
            } else {
                bits.pop();
            }
        }

        if bits.len() == 0 {
            false
        } else {
            true
        }
    }
}
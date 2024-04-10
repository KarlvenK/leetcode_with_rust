struct Solution {}

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut binary: Vec<char> = binary.chars().collect();
        let mut zero_cnt = 0;
        let mut first_pos: Option<usize> = None;
        for i in 0..binary.len() {
            if binary[i] == '0' {
                zero_cnt += 1;
                match first_pos {
                    None => first_pos = Some(i),
                    _ => (),
                }
                binary[i] = '1';
            }
        }
        if zero_cnt != 0 {
            binary[first_pos.unwrap() + zero_cnt - 1] = '0';
        }
        binary.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1702() {
        assert_eq!(
            "111011",
            Solution::maximum_binary_string("000110".to_string())
        );
        assert_eq!("01", Solution::maximum_binary_string("01".to_string()));
    }
}

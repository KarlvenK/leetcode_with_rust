pub struct Solution {}

impl Solution {
    pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
        let (mut ans, mut max) = (0, 0);
        for (i, v) in flips.iter().enumerate() {
            if *v > max {
                max = *v;
            }
            if i as i32 == max - 1 {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1375() {
        assert_eq!(Solution::num_times_all_blue(vec![3, 2, 4, 1, 5]), 2)
    }
}

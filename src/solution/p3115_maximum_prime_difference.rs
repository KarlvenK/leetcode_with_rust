struct Solution {}
impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut is_prime = vec![true; max_num as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..=max_num {
            if is_prime[i as usize] {
                if i * i > max_num {
                    continue;
                }
                for j in (i * i..=max_num).step_by(i as usize) {
                    is_prime[j as usize] = false;
                }
            }
        }
        let (mut first, mut last): (Option<usize>, Option<usize>) = (None, None);
        for (i, &v) in nums.iter().enumerate() {
            match is_prime[v as usize] {
                false => (),
                true => match first {
                    None => first = Some(i),
                    Some(_) => last = Some(i),
                },
            }
        }
        match (first, last) {
            (None, None) => panic!(),
            (Some(_), None) => 0,
            (Some(x), Some(y)) => (y - x) as i32,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3115() {
        assert_eq!(Solution::maximum_prime_difference(vec![4, 2, 9, 5, 3]), 3);
        assert_eq!(Solution::maximum_prime_difference(vec![4, 8, 2, 8]), 0);
    }
}

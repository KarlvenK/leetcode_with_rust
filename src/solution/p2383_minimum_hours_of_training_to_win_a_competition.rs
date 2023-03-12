use std::cmp::max;

pub struct Solution {}
impl Solution {
    pub fn min_number_of_hours(
        initial_energy: i32,
        mut initial_experience: i32,
        energy: Vec<i32>,
        experience: Vec<i32>,
    ) -> i32 {
        let mut ans = max(0, energy.iter().sum::<i32>() + 1 - initial_energy);

        for e in experience {
            if e >= initial_experience {
                ans += e - initial_experience + 1;
                initial_experience = e * 2 + 1;
            } else {
                initial_experience += e;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2383() {
        assert_eq!(
            Solution::min_number_of_hours(5, 3, vec![1, 4, 3, 2], vec![2, 6, 3, 1]),
            8
        )
    }
}

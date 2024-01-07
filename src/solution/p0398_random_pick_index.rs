use rand;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

struct Solution {
    nums: Vec<i32>,
    map: HashMap<i32, Vec<usize>>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let rng = rand::thread_rng();
        let mut map = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            let eny = map.entry(v).or_insert(vec![]);
            eny.push(i);
        }
        Solution { nums, map, rng }
    }

    fn pick(&mut self, target: i32) -> i32 {
        let eny = self.map.get(&target).unwrap();
        let n = eny.len();
        let i = self.rng.gen_range(0..n);
        *(eny.get(i).unwrap()) as i32
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_398() {
        let nums = vec![1, 2, 3, 3, 3];
        let mut obj = Solution::new(nums);
        let ret_1 = obj.pick(1);
        assert_eq!(0, ret_1);
    }
}

use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    nums: Vec<i32>,
    map: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            nums: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let mut ret = true;

        match self.map.get(&val) {
            None => {
                self.nums.push(val);
                self.map.insert(val, self.nums.len() - 1);
            }
            _ => {
                ret = false;
            }
        }
        ret
    }

    fn remove(&mut self, val: i32) -> bool {
        let mut ret = true;

        match self.map.get(&val) {
            None => {
                ret = false;
            }
            Some(pos) => {
                let tail = self.nums[self.nums.len() - 1];
                self.nums[*pos] = tail;
                self.nums.pop();
                self.map.insert(tail, *pos);
                self.map.remove(&val);
            }
        }
        ret
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let pos: usize = rng.gen_range(0..self.nums.len());
        // pos = pos % self.nums.len();
        self.nums[pos]
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_380() {
        let mut obj = RandomizedSet::new();
        let ret_1 = obj.insert(1);
        assert_eq!(ret_1, true);
        let ret_2 = obj.remove(1);
        assert_eq!(ret_2, true);
        let _ret_3 = obj.insert(100000);
        let ret_4 = obj.get_random();
        assert_eq!(ret_4, 100000);
    }
}

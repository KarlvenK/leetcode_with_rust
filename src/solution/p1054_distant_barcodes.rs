pub struct Solution {}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Copy, Clone)]
struct Comb {
    val: i32,
    cnt: i32,
}

impl PartialOrd<Self> for Comb {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cnt.cmp(&other.cnt))
    }
}

impl Ord for Comb {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cnt.cmp(&other.cnt)
    }
}

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        let mut map = HashMap::new();
        for b in &barcodes {
            let entry = map.entry(b).or_insert(0);
            *entry += 1;
        }

        for en in map {
            heap.push(Comb {
                val: *en.0,
                cnt: en.1,
            })
        }

        let mut ans = vec![];

        for i in 0..barcodes.len() {
            let mut x = heap.pop().unwrap();
            if !ans.is_empty() && x.val == ans[ans.len() - 1] {
                let temp = x;
                x = heap.pop().unwrap();
                heap.push(temp);
            }
            ans.push(x.val);
            if x.cnt == 1 {
                continue;
            }
            x.cnt -= 1;
            heap.push(x);
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1054() {
        let res = Solution::rearrange_barcodes(vec![1, 1, 1, 1, 2, 2, 3, 3]);
        for i in 1..res.len() {
            if res[i] == res[i - 1] {
                panic!("shit")
            }
        }
    }
}

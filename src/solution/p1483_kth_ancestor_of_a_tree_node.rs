use std::vec;

struct TreeAncestor {
    size: i32,
    ancestor: Vec<Vec<i32>>,
    parent: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let mut ancestor = vec![vec![-1; 20]; n as usize];
        for i in 0..n {
            ancestor[i as usize][0] = parent[i as usize];
        }
        for i in 1..20 {
            let mut changed = false;
            for j in 0..n {
                let pre = ancestor[j as usize][i - 1];
                if pre == -1 {
                    continue;
                }
                changed = true;
                ancestor[j as usize][i] = ancestor[pre as usize][i - 1];
            }
            if !changed {
                break;
            }
        }
        Self {
            size: n,
            ancestor: ancestor,
            parent: parent,
        }
    }

    fn get_kth_ancestor(&self, node: i32, mut k: i32) -> i32 {
        let mut step = 0;
        let mut anc = node;
        while k != 0 {
            if k & 1 == 1 {
                anc = self.ancestor[anc as usize][step];
            }
            if anc == -1 {
                break;
            }
            k = k >> 1;
            step += 1;
        }
        anc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1483() {
        let ancestor = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
        assert_eq!(1, ancestor.get_kth_ancestor(3, 1));
        assert_eq!(0, ancestor.get_kth_ancestor(5, 2));
        assert_eq!(-1, ancestor.get_kth_ancestor(6, 3));
    }
}

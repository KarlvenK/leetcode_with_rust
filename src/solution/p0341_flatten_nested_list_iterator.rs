use std::collections::vec_deque::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    vals: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut vals = VecDeque::new();
        fn dfs(list: Vec<NestedInteger>, vals: &mut VecDeque<i32>) {
            for item in list.into_iter() {
                match item {
                    NestedInteger::Int(x) => {
                        vals.push_back(x);
                    }
                    NestedInteger::List(x) => {
                        dfs(x, vals);
                    }
                }
            }
        }
        dfs(nested_list, &mut vals);
        NestedIterator { vals }
    }

    fn next(&mut self) -> i32 {
        let ret = self.vals.pop_front().unwrap();
        ret
    }

    fn has_next(&self) -> bool {
        !self.vals.is_empty()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {

    #[test]
    fn test_341() {}
}

pub struct Solution {}

use crate::utils::linked_list::{to_list, ListNode};

use std::cmp::{Ord, Ordering, PartialEq};
use std::collections::BinaryHeap;

impl PartialOrd<Self> for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut pre = &mut dummy_head;
        let mut heap = BinaryHeap::new();

        for node in lists {
            if let Some(x) = node {
                heap.push(x);
            }
        }

        while let Some(mut node) = heap.pop() {
            if let Some(nex) = node.next.take() {
                heap.push(nex);
            }
            pre.next = Some(node);
            pre = pre.next.as_mut().unwrap();
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_23() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}

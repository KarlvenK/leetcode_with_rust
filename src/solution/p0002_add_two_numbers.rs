pub struct Solution {}
use crate::utils::linked_list::{to_list, ListNode};

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;

        let (mut mark1, mut mark2) = (false, false);
        let mut carry = 0;
        loop {
            let lhs = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => {
                    mark1 = true;
                    0
                }
            };

            let rhs = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => {
                    mark2 = true;
                    0
                }
            };

            if mark1 && mark2 && carry == 0 {
                break;
            }

            let mut sum = lhs + rhs + carry;
            carry = 0;
            if sum > 9 {
                carry = 1;
                sum = sum - 10;
            }
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4])),
            to_list(vec![7, 0, 8])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![9, 9, 9, 9]), to_list(vec![9, 9, 9, 9, 9, 9])),
            to_list(vec![8, 9, 9, 9, 0, 0, 1])
        );

        assert_eq!(
            Solution::add_two_numbers(to_list(vec![0]), to_list(vec![0])),
            to_list(vec![0])
        )
    }
}

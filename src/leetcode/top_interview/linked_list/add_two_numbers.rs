// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub trait ListMaker {
    fn link(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }
}

impl ListMaker for Option<Box<ListNode>> {}

// 2. Add Two Numbers, Medium
// https://leetcode.com/problems/add-two-numbers/
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum: Option<Box<ListNode>> = None;

        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut p3 = &mut sum;
        let mut carry = 0;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;

            if let Some(n1) = p1.as_ref() {
                val += n1.val;
                p1 = &n1.next;
            }

            if let Some(n2) = p2.as_ref() {
                val += n2.val;
                p2 = &n2.next;
            }

            carry = val / 10;
            *p3 = Option::<Box<ListNode>>::link(val % 10, None);
            p3 = &mut p3.as_mut().unwrap().next;
        }
        sum
    }
}

struct Solution {}

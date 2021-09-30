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

// 725. Split Linked List in Parts, Medium
// https://leetcode.com/problems/split-linked-list-in-parts/
impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut len = 0;
        let mut node = &head;
        
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        
        let mut head = head;
        let mut answer = Vec::with_capacity(k as usize);

        for i in 0..k as usize {
            answer.push(head);
            let mut node = &mut answer[i];
            for _ in 0..len / k + if i < (len % k) as usize { 1 } else { 0 } {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            head = node.take();
        }
        answer
    }
}

struct Solution {}

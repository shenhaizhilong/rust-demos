pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            val: val,
            next: next,
        }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut pre = &mut dummy;
        while let Some(mut p) = pre.next.take() {
            if let Some(mut q) = p.next.take() {
                let t = q.next.take();
                p.next = t;
                q.next = Some(p);
                pre.next = Some(q);
                pre = pre.next.as_mut().unwrap().next.as_mut().unwrap();
            } else {
                pre.next = Some(p);
                pre = pre.next.as_mut().unwrap();
            }
        }
        return dummy.next;
    }

    pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut h1) = head {
            // Takes the value out of the option, leaving a None in its place.
            match h1.next.take() {
                Some(mut h2) => {
                    let tail = Self::swap_pairs2(h2.next.take());
                    h2.next.insert(h1).next = tail;
                    return Some(h2);
                }
                _ => { return Some(h1); }
            }
        }
        return head;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_swap_pairs() {}
}
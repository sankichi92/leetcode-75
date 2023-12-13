pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;

    let mut stack = Vec::new();

    while let Some(mut node) = head {
        head = node.next.take();
        stack.push(node);
    }

    let mut head = stack.pop();
    let mut current = head.as_mut().unwrap();

    while !stack.is_empty() {
        current.next = stack.pop();
        current = current.next.as_mut().unwrap();
    }

    head
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5)))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(1)))
                        }))
                    }))
                }))
            }))
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1)))
            }))
        )
    }
}

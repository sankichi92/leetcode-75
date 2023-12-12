// Definition for singly-linked list.
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

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut size = 0;
    let mut current = head.as_ref();

    while let Some(boxed_node) = current {
        size += 1;
        current = boxed_node.next.as_ref();
    }

    let middle = size / 2;
    let mut current = head.as_mut().unwrap();

    for _ in 0..middle - 1 {
        current = current.next.as_mut().unwrap();
    }
    current.next = current.next.as_mut().unwrap().next.take();

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            delete_middle(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 2,
                                    next: Some(Box::new(ListNode::new(6)))
                                }))
                            }))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode::new(6)))
                            }))
                        }))
                    }))
                }))
            }))
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            delete_middle(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4)))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(4)))
                }))
            }))
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            delete_middle(Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1)))
            }))),
            Some(Box::new(ListNode::new(2)))
        )
    }
}

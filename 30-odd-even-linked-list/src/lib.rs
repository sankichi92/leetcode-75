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

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd = head.as_mut()?;

    let mut even_head;
    if let Some(node) = odd.next.take() {
        even_head = node;
    } else {
        return head;
    }
    let mut even = &mut even_head;

    let mut current = even.next.take();
    let mut is_odd = true;

    while current.is_some() {
        if is_odd {
            odd.next = current;
            odd = odd.next.as_mut().unwrap();
            current = odd.next.take();
            is_odd = false;
        } else {
            even.next = current;
            even = even.next.as_mut().unwrap();
            current = even.next.take();
            is_odd = true;
        }
    }

    odd.next = Some(even_head);

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            odd_even_list(Some(Box::new(ListNode {
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
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(4)))
                        }))
                    }))
                }))
            }))
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            odd_even_list(Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode {
                                val: 6,
                                next: Some(Box::new(ListNode {
                                    val: 4,
                                    next: Some(Box::new(ListNode::new(7)))
                                }))
                            }))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode {
                            val: 7,
                            next: Some(Box::new(ListNode {
                                val: 1,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode::new(4)))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        )
    }
}

pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    let mut vec = Vec::new();

    while let Some(mut node) = head {
        head = node.next.take();
        vec.push(node.val);
    }

    let n = vec.len();
    let mut result = 0;

    for i in 0..n / 2 {
        result = result.max(vec[i] + vec[n - 1 - i])
    }

    result
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
            pair_sum(Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(1)))
                    }))
                }))
            }))),
            6
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            pair_sum(Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                }))
            }))),
            7
        )
    }

    #[test]
    fn case3() {
        assert_eq!(
            pair_sum(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(100000)))
            }))),
            100001
        )
    }
}

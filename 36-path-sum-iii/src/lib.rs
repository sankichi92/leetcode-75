use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    inner_path_sum(&root, VecDeque::new(), target_sum)
}

fn inner_path_sum(
    node: &Option<Rc<RefCell<TreeNode>>>,
    mut parents: VecDeque<i32>,
    target_sum: i32,
) -> i32 {
    if let Some(node) = node {
        let node = node.borrow();
        let mut sum: i64 = node.val as i64;
        let mut results = if sum == target_sum as i64 { 1 } else { 0 };

        for parent in parents.iter() {
            sum += *parent as i64;
            if sum == target_sum as i64 {
                results += 1
            }
        }

        parents.push_front(node.val);

        results
            + inner_path_sum(&node.left, parents.clone(), target_sum)
            + inner_path_sum(&node.right, parents.clone(), target_sum)
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 3,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(-2)))),
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(11))))
                    }))),
                }))),
                8
            ),
            3
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 11,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                        })))
                    }))),
                }))),
                22
            ),
            3
        )
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1000000000,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1000000000,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 294967296,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1000000000,
                                left: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 1000000000,
                                    left: Some(Rc::new(RefCell::new(TreeNode::new(1000000000)))),
                                    right: None,
                                }))),
                                right: None,
                            }))),
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: None,
                }))),
                0
            ),
            0
        )
    }
}

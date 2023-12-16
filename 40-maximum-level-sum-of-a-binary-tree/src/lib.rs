use std::cell::RefCell;
use std::rc::Rc;

pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn calc_level_sum(node: Rc<RefCell<TreeNode>>, level: usize, level_sums: &mut Vec<i32>) {
        let node = node.borrow();
        if let Some(sum) = level_sums.get_mut(level) {
            *sum += node.val
        } else {
            level_sums.insert(level, node.val)
        }

        if let Some(left) = &node.left {
            calc_level_sum(Rc::clone(left), level + 1, level_sums)
        }
        if let Some(right) = &node.right {
            calc_level_sum(Rc::clone(right), level + 1, level_sums)
        }
    }

    let mut level_sums = Vec::new();
    calc_level_sum(root.unwrap(), 0, &mut level_sums);

    level_sums
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_, sum1), (_, sum2)| sum1.cmp(sum2))
        .unwrap()
        .0 as i32
        + 1
}

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
            max_level_sum(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            })))),
            2
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            max_level_sum(Some(Rc::new(RefCell::new(TreeNode {
                val: 989,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 10250,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(98693)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -89388,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(-32127)))),
                    }))),
                }))),
            })))),
            2
        )
    }

    #[test]
    fn failed_case1() {
        assert_eq!(
            max_level_sum(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(-8)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                }))),
            })))),
            1
        )
    }
}

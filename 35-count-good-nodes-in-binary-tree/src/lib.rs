use std::cell::RefCell;
use std::rc::Rc;

pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root = root.unwrap();
    let val = root.borrow().val;
    inner_good_nodes(&root, val)
}

fn inner_good_nodes(node: &Rc<RefCell<TreeNode>>, max: i32) -> i32 {
    let node = node.borrow();
    let max = max.max(node.val);
    let good = if node.val >= max { 1 } else { 0 };

    match (&node.left, &node.right) {
        (None, None) => good,
        (Some(left), None) => good + inner_good_nodes(left, max),
        (None, Some(right)) => good + inner_good_nodes(right, max),
        (Some(left), Some(right)) => {
            good + inner_good_nodes(left, max) + inner_good_nodes(right, max)
        }
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
            good_nodes(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                }))),
            })))),
            4
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            good_nodes(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            })))),
            3
        )
    }

    #[test]
    fn case3() {
        assert_eq!(good_nodes(Some(Rc::new(RefCell::new(TreeNode::new(1))))), 1)
    }
}

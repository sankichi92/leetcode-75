use std::cell::RefCell;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root) = root {
        get_depth(&root, 1)
    } else {
        0
    }
}

fn get_depth(node: &Rc<RefCell<TreeNode>>, depth: i32) -> i32 {
    let node = node.borrow();
    match (&node.left, &node.right) {
        (Some(left), Some(right)) => get_depth(left, depth + 1).max(get_depth(right, depth + 1)),
        (Some(left), None) => get_depth(left, depth + 1),
        (None, Some(right)) => get_depth(right, depth + 1),
        (None, None) => depth,
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
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                }))),
            })))),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            max_depth(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            })))),
            2
        );
    }
}

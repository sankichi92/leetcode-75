use std::cell::RefCell;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut results = Vec::new();
    let mut stack = if let Some(root) = root {
        vec![(Rc::clone(&root), 0)]
    } else {
        return results;
    };

    while let Some((node, depth)) = stack.pop() {
        let node = node.borrow();

        if depth >= results.len() {
            results.push(node.val)
        }

        if let Some(left) = &node.left {
            stack.push((Rc::clone(left), depth + 1))
        }
        if let Some(right) = &node.right {
            stack.push((Rc::clone(right), depth + 1))
        }
    }

    results
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
            right_side_view(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
            })))),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            right_side_view(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            vec![1, 3]
        );
    }

    #[test]
    fn case3() {
        assert_eq!(right_side_view(None), vec![]);
    }
}

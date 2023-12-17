use std::cell::RefCell;
use std::rc::Rc;

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = vec![root.unwrap()];

    while let Some(node) = stack.pop() {
        if node.borrow().val == val {
            return Some(node);
        }

        let node = node.borrow();
        if let Some(left) = &node.left {
            stack.push(Rc::clone(left))
        }
        if let Some(right) = &node.right {
            stack.push(Rc::clone(right))
        }
    }

    None
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
        let expected = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }));

        assert_eq!(
            search_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::clone(&expected)),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
                2
            ),
            Some(expected)
        )
    }

    #[test]
    fn case2() {
        assert_eq!(
            search_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
                5
            ),
            None
        )
    }
}

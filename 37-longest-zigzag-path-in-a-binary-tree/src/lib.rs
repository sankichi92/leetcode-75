use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = Vec::new();
    stack_tree(&root, &mut stack);

    let mut longest = 0;
    while let Some(node) = stack.pop() {
        let node = node.borrow();
        longest = cmp::max(
            longest,
            cmp::max(zig_zag(&node.left, true, 0), zig_zag(&node.right, false, 0)),
        )
    }

    longest
}

fn stack_tree(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = node {
        stack.push(Rc::clone(node));
        let node = node.borrow();
        stack_tree(&node.left, stack);
        stack_tree(&node.right, stack);
    }
}

fn zig_zag(node: &Option<Rc<RefCell<TreeNode>>>, parent_is_left: bool, length: i32) -> i32 {
    match (node, parent_is_left) {
        (Some(node), true) => zig_zag(&node.borrow().right, false, length + 1),
        (Some(node), false) => zig_zag(&node.borrow().left, true, length + 1),
        (None, _) => length,
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
            longest_zig_zag(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 1,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                            })))
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                    })))
                }))),
            })))),
            3
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            longest_zig_zag(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            })))),
            4
        );
    }
}

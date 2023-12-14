use std::cell::RefCell;
use std::rc::Rc;

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    get_leaves(&root1.unwrap()) == get_leaves(&root2.unwrap())
}

fn get_leaves(node: &Rc<RefCell<TreeNode>>) -> Vec<i32> {
    let mut leaves = Vec::new();

    let node = node.borrow();
    match (&node.left, &node.right) {
        (None, None) => leaves.push(node.val),
        (Some(left), None) => leaves.extend(get_leaves(left)),
        (None, Some(right)) => leaves.extend(get_leaves(right)),
        (Some(left), Some(right)) => {
            leaves.extend(get_leaves(left));
            leaves.extend(get_leaves(right));
        }
    }

    leaves
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
        assert!(leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(8))))
                }))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8))))
                    })))
                }))),
            })))
        ))
    }

    #[test]
    fn case2() {
        assert!(!leaf_similar(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            })))
        ))
    }
}

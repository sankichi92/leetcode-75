use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    fn inner_delete_node(node: Rc<RefCell<TreeNode>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node_ref = node.borrow_mut();
        match node_ref.val.cmp(&key) {
            Ordering::Greater => {
                if let Some(left) = &node_ref.left {
                    let left = inner_delete_node(Rc::clone(left), key);
                    node_ref.left = left;
                }
            }
            Ordering::Less => {
                if let Some(right) = &node_ref.right {
                    let right = inner_delete_node(Rc::clone(right), key);
                    node_ref.right = right;
                }
            }
            Ordering::Equal => match (&node_ref.left, &node_ref.right) {
                (None, None) => return None,
                (Some(left), None) => return Some(Rc::clone(left)),
                (None, Some(right)) => return Some(Rc::clone(right)),
                (Some(_), Some(right)) => {
                    let next_val = find_next_val(Rc::clone(right));
                    let right = inner_delete_node(Rc::clone(right), next_val);
                    node_ref.right = right;
                    node_ref.val = next_val;
                }
            },
        }
        drop(node_ref);
        Some(node)
    }

    fn find_next_val(node: Rc<RefCell<TreeNode>>) -> i32 {
        if let Some(left) = &node.borrow().left {
            find_next_val(Rc::clone(left))
        } else {
            node.borrow().val
        }
    }

    inner_delete_node(root?, key)
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
            delete_node(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                }))),
                3
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))
        )
    }

    #[test]
    fn case2() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(
            delete_node(Some(Rc::clone(root.as_ref().unwrap())), 0),
            root
        )
    }

    #[test]
    fn case3() {
        assert_eq!(delete_node(None, 0), None)
    }
}

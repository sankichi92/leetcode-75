use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p = p.unwrap();
    let q = q.unwrap();

    let mut p_ancestors = Vec::new();
    let mut q_ancestors = Vec::new();

    let mut stack = vec![(root.unwrap(), Vec::new())];

    while let Some((node, mut ancestors)) = stack.pop() {
        if !p_ancestors.is_empty() && !q_ancestors.is_empty() {
            break;
        }

        ancestors.push(Rc::clone(&node));

        if p_ancestors.is_empty() && node == p {
            p_ancestors.extend(ancestors.clone())
        } else if q_ancestors.is_empty() && node == q {
            q_ancestors.extend(ancestors.clone())
        }

        let node = node.borrow();
        if let Some(left) = &node.left {
            stack.push((Rc::clone(left), ancestors.clone()));
        }
        if let Some(right) = &node.right {
            stack.push((Rc::clone(right), ancestors.clone()));
        }
    }

    for p_ancestor in p_ancestors.iter().rev() {
        for q_ancestor in q_ancestors.iter().rev() {
            if p_ancestor == q_ancestor {
                return Some(Rc::clone(p_ancestor));
            }
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
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::clone(p.as_ref().unwrap())),
            right: Some(Rc::clone(q.as_ref().unwrap())),
        })));

        assert_eq!(
            lowest_common_ancestor(Some(Rc::clone(root.as_ref().unwrap())), p, q),
            root
        )
    }

    #[test]
    fn case2() {
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));

        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::clone(q.as_ref().unwrap())),
            }))),
        })));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::clone(p.as_ref().unwrap())),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
        })));

        assert_eq!(
            lowest_common_ancestor(root, Some(Rc::clone(p.as_ref().unwrap())), q),
            p
        )
    }

    #[test]
    fn case3() {
        let q = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::clone(q.as_ref().unwrap())),
            right: None,
        })));
        assert_eq!(
            lowest_common_ancestor(
                Some(Rc::clone(root.as_ref().unwrap())),
                Some(Rc::clone(root.as_ref().unwrap())),
                q
            ),
            root
        )
    }
}

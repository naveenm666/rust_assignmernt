use std::cmp;
use std::collections::VecDeque;

// Definition for a binary tree node.
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            cmp::max(left_depth, right_depth) + 1
        }
        None => 0,
    }
}

fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    let mut right_node = root.right.clone().unwrap();
    right_node.left = Some(Box::new(TreeNode::new(15)));
    right_node.right = Some(Box::new(TreeNode::new(7)));

    let depth = max_depth(&Some(Box::new(root)));
    println!("Maximum depth of the binary tree: {}", depth);
}

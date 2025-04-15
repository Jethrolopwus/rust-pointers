#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// === Implementation with some useful methods ===//
impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    //== Add a left child ==//
    fn add_left(&mut self, value: T) {
        self.left = Some(Box::new(TreeNode::new(value)));
    }

    // == Add a right child ===//
    fn add_right(&mut self, value: T) {
        self.right = Some(Box::new(TreeNode::new(value)));
    }
}

fn main() {
    // Create a tree:
    //      10
    //     /  \
    //    5    15
    //   / \
    //  3   7
    let mut root = TreeNode::new(10);

    root.add_left(5);
    root.add_right(15);

    //== Access and modify a child node (need to unwrap the Option) ===//
    if let Some(ref mut left_child) = root.left {
        left_child.add_left(3);
        left_child.add_right(7);
    }
    println!("Root value: {:?}", root.value);
    println!("Tree created successfully!");
}

// This works because Box allows us create the recursive structure, without Box, Rust could not determine the size of the
// the TreeNode since it would contain itself
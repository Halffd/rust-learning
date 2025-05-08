
use std::fmt::Display;

// Node structure for our binary tree
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Binary Tree implementation
struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    // Create a new node with a given value
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Display> BinaryTree<T> {
    // Create a new empty binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // Insert a value into the tree (assuming T is Ord, for simplicity)
    fn insert(&mut self, value: T) 
    where T: Ord {
        fn insert_rec<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                None => *node = Some(Box::new(Node::new(value))),
                Some(n) => {
                    if value < n.value {
                        insert_rec(&mut n.left, value);
                    } else {
                        insert_rec(&mut n.right, value);
                    }
                }
            }
        }
        
        insert_rec(&mut self.root, value);
    }

    // Pre-order traversal (Root, Left, Right)
    fn pre_order(&self) {
        println!("Pre-order traversal:");
        
        fn pre_order_rec<T: Display>(node: &Option<Box<Node<T>>>, prefix: &str) {
            if let Some(n) = node {
                // Visit the node itself
                println!("{}{}", prefix, n.value);
                
                // Visit left subtree
                pre_order_rec(&n.left, prefix);
                
                // Visit right subtree
                pre_order_rec(&n.right, prefix);
            }
        }
        
        pre_order_rec(&self.root, "");
        println!();
    }

    // In-order traversal (Left, Root, Right)
    fn in_order(&self) {
        println!("In-order traversal:");
        
        fn in_order_rec<T: Display>(node: &Option<Box<Node<T>>>, prefix: &str) {
            if let Some(n) = node {
                // Visit left subtree
                in_order_rec(&n.left, prefix);
                
                // Visit the node itself
                println!("{}{}", prefix, n.value);
                
                // Visit right subtree
                in_order_rec(&n.right, prefix);
            }
        }
        
        in_order_rec(&self.root, "");
        println!();
    }

    // Post-order traversal (Left, Right, Root)
    fn post_order(&self) {
        println!("Post-order traversal:");
        
        fn post_order_rec<T: Display>(node: &Option<Box<Node<T>>>, prefix: &str) {
            if let Some(n) = node {
                // Visit left subtree
                post_order_rec(&n.left, prefix);
                
                // Visit right subtree
                post_order_rec(&n.right, prefix);
                
                // Visit the node itself
                println!("{}{}", prefix, n.value);
            }
        }
        
        post_order_rec(&self.root, "");
        println!();
    }

    // Bonus: Print the tree structure (for visualization)
    fn print_tree(&self) {
        println!("Tree structure:");
        fn print_tree_rec<T: Display>(node: &Option<Box<Node<T>>>, prefix: &str, is_left: bool) {
            if let Some(n) = node {
                println!("{}{}{}",
                         prefix,
                         if is_left { "├── " } else { "└── " },
                         n.value);
                
                // Prepare prefix for children
                let new_prefix = format!("{}{}",
                                       prefix,
                                       if is_left { "│   " } else { "    " });
                
                // Print left child
                print_tree_rec(&n.left, &new_prefix, true);
                
                // Print right child
                print_tree_rec(&n.right, &new_prefix, false);
            }
        }
        
        if let Some(root) = &self.root {
            println!("{}", root.value);
            print_tree_rec(&root.left, "", true);
            print_tree_rec(&root.right, "", false);
        } else {
            println!("<empty tree>");
        }
        println!();
    }
}

fn main() {
    // Create a binary search tree and insert some values
    let mut tree = BinaryTree::new();
    
    // Let's build a simple tree:
    //       50
    //      /  \
    //     30   70
    //    / \   / \
    //   20 40 60 80

    tree.insert(50);
    tree.insert(30);
    tree.insert(70);
    tree.insert(20);
    tree.insert(40);
    tree.insert(60);
    tree.insert(80);
    
    // Visualize the tree structure
    tree.print_tree();
    
    // Perform all three traversals
    tree.pre_order();    // Expected: 50, 30, 20, 40, 70, 60, 80
    tree.in_order();     // Expected: 20, 30, 40, 50, 60, 70, 80 (sorted for BST!)
    tree.post_order();   // Expected: 20, 40, 30, 60, 80, 70, 50
}
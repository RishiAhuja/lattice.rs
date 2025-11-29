use std::cmp;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub val: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy + Debug> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Copy + Debug> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    pub fn insert(&mut self, val: T) {
        Self::insert_rec(&mut self.root, val);
    }

    fn insert_rec(node: &mut Option<Box<Node<T>>>, val: T) {
        match node {
            None => {
                *node = Some(Box::new(Node::new(val)));
            }
            Some(n) => {
                if val < n.val {
                    Self::insert_rec(&mut n.left, val);
                } else {
                    Self::insert_rec(&mut n.right, val);
                }
            }
        }
    }

    pub fn search(&self, val: T) -> bool {
        let mut current = &self.root;
        while let Some(n) = current {
            if val == n.val {
                return true;
            } else if val < n.val {
                current = &n.left;
            } else {
                current = &n.right;
            }
        }

        false
    }

    pub fn height(&self) -> i32 {
        Self::height_rec(&self.root)
    }

    fn height_rec(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            None => -1,
            Some(n) => {
                let left_h = Self::height_rec(&n.left);
                let right_h = Self::height_rec(&n.right);
                cmp::max(left_h, right_h) + 1
            }
        }
    }

    pub fn count_nodes(&self) -> i32 {
        Self::count_nodes_rec(&self.root)
    }

    fn count_nodes_rec(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => 1 + Self::count_nodes_rec(&n.left) + Self::count_nodes_rec(&n.right),
        }
    }

    pub fn count_leaves(&self) -> i32 {
        Self::count_leaves_rec(&self.root)
    }

    fn count_leaves_rec(node: &Option<Box<Node<T>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => {
                if n.left.is_none() && n.right.is_none() {
                    1
                } else {
                    Self::count_leaves_rec(&n.left) + Self::count_leaves_rec(&n.right)
                }
            }
        }
    }

    pub fn print_inorder(&self) {
        print!("Inorder: ");
        Self::inorder_rec(&self.root);
        println!();
    }

    fn inorder_rec(node: &Option<Box<Node<T>>>) {
        if let Some(n) = node {
            Self::inorder_rec(&n.left);
            print!("{:?} ", n.val);
            Self::inorder_rec(&n.right);
        }
    }

    pub fn print_preorder(&self) {
        print!("Preorder: ");
        Self::preorder_rec(&self.root);
        println!();
    }

    fn preorder_rec(node: &Option<Box<Node<T>>>) {
        if let Some(n) = node {
            print!("{:?} ", n.val);
            Self::preorder_rec(&n.left);
            Self::preorder_rec(&n.right);
        }
    }

    pub fn print_postorder(&self) {
        print!("Inorder: ");
        Self::postorder_rec(&self.root);
        println!();
    }

    fn postorder_rec(node: &Option<Box<Node<T>>>) {
        if let Some(n) = node {
            Self::postorder_rec(&n.left);
            Self::postorder_rec(&n.right);
            print!("{:?} ", n.val);
        }
    }

    pub fn delete(&mut self, val: T) {
        self.root = Self::delete_rec(self.root.take(), val);
    }

    fn delete_rec(node: Option<Box<Node<T>>>, val: T) -> Option<Box<Node<T>>> {
        match node {
            None => None,
            Some(mut n) => {
                if val < n.val {
                    n.left = Self::delete_rec(n.left, val);
                    Some(n)
                } else if val > n.val {
                    n.right = Self::delete_rec(n.right, val);
                    Some(n)
                } else {
                    if n.left.is_none() && n.right.is_none() {
                        return None;
                    }
                    if n.left.is_none() {
                        return n.right;
                    }
                    if n.right.is_none() {
                        return n.left;
                    }

                    let min_val = Self::find_min(&n.right);
                    n.val = min_val;
                    n.right = Self::delete_rec(n.right, min_val);
                    Some(n)
                }
            }
        }
    }

    fn find_min(node: &Option<Box<Node<T>>>) -> T {
        let mut current = node;
        while let Some(n) = current {
            if n.left.is_none() {
                return n.val;
            }
            current = &n.left;
        }
        panic!("Tree is empty");
    }
}

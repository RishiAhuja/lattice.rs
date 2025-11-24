use std::cmp;
use std::fmt::Debug;

type NodeLink<T> = Option<Box<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub val: T,
    pub height: i32,
    pub left: NodeLink<T>,
    pub right: NodeLink<T>
}

impl<T> Node<T> {
    fn new(val: i32) -> Self {
        Node {
            val,
            height: 1,
            left: None, 
            right: None
        }
    }
}

#[derive(Debug)]
pub struct AVLTree<T> {
    pub root: NodeLink<T>,
}

impl<T: Ord + Copy + Debug> AVLTree<T> {
    pub fn new () -> Self {
        AVLTree {root: None}
    }

    pub fn insert (&mut self, val: T) {
        // assign potentially new root back
        self.root = Self::insert_rec(self.root.take(), val);
    }

    fn insert_rec(node: &NodeLink<T>, val: i32) -> NodeLink<T> {
        let mut n = match node {
            None => return Some(Box::new(Node::new(val))),
            Some(n) => n
        }

        if val < n.val {
            n.left = Self::insert_rec(n.left, val);
        } else if val > n.val {
            n.right = Self::insert_rec(n.right, val);
        } else {
            return Some(n);
        }

        Self::update_height(&mut n);
        Self::rebalance(n);
    }

    fn rebalance(&Box<Node<T>) -> NodeLink<T> {
        let balance = Self::get_balance(&node);

        if balance > 1 {
            if Self::get_balance(node.left.as_ref().unwrap()) < 0 {
                node.left = Self::rotate_left(node.left.unwrap());
            }
            return Self::rotate_right(node);
        }

        if balance < -1 {
            if Self::get_balance(node.right.as_ref().unwrap() > 0) {
                node.right = Self::rotate_right(node.right.unwrap());
            }
            return Self::rotate_left(node);
        }

        Some(node)
    }

    //       y                     x 
    //      / \                   / \
    //     x   T3                T1  y
    //    / \                       / \
    //   T1  T2                    T2  T3

    fn rotate_right(mut y: Box<Node<T>>) -> NodeLink<T> {
        let mut x = y.left.take();
        let mut t2 = x.right.take();
        y.left = t2;

        Self::update_height(&mut y);

        x.right = Some(y);

        Self::update_height(&mut x);

        Some(x)
    }

    //       x                     y 
    //      / \                   / \
    //     T1  y                 x   T3
    //        / \               / \
    //       T2  T3            T1  T2

    fn rotate_left(mut x: Box<Node<T>>) -> NodeLink<T> {
        let mut y = x.right.take();
        let mut t2 = y.left.take();
        x.right = t2;

        Self::update_height(&mut x);

        y.left = Some(x);

        Self::update_height(&mut y);

        Some(x)
    }

    pub fn height(&self) -> i32 {
        Self::get_height(&self.root)
    }

    fn update_height(node: &mut Box<Node<T>>) {
        node.height = 1 + cmp::max(
            Self::get_height(&node.left), 
            Self::get_height(&node.right)
        );
    }

    fn get_height(node: &NodeLink<T>) -> i32 {
        match node {
            None => 0,
            Some(n) => n.height,
        }
    }

    fn get_balance(node: &Box<Node<T>>) -> i32 {
        Self::get_height(&node.left) - Self::get_height(&node.right)
    }

    pub fn print_inorder(&self) {
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

}
use std::cmp
use std::fmt::Debug

#[derive(Clone, Debug)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    is_leaf: bool,
}

impl<T> Node<T> {
    fn new(is_leaf: false) -> Self {
        Node {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf
        }
    }
}

#[derive(Debug)]
pub struct BTree<T> {
    root: Node<T>,
    order: usize,
}

impl<T: Ord + Copy + Debug> BTree<T> {
    pub fn new(order: usize) -> Self {
        assert!(order >= 3, "Order must be at least 3");
        BTree {
            root: Node::new(true),
            order,
        }
    }

    pub fn search(&self, val: T) -> bool {
        Self::search_node(&self.root, val)
    }

    fn search_node(node: &Node<T>, val: T) -> bool {
        let mut i = 0;
        while i < node.keys.len() && val > node.keys[i] {
            i += 1;
        }

        if i < node.keys.len() && val == node.keys[i] {
            return true;
        }

        if node.is_leaf {
            return false;
        }

        Self::search_node(&node.children[i], val)
    }

    pub fn insert(&mut self, val: T) {
        let split_result = Self::insert_rec(&mut self.root, val, self.order);

        if let Some((median, right_child)) = split_result {
            let new_root = Node {
                keys: vec![median],
                children: vec![mem::replace(&mut self.root, Node::new(false)), right_child],
                is_leaf: false,
            };
            self.root = new_root;
        }
    }

    fn insert_rec(node: &mut Node<T>, val: T, order: usize) -> Option<(T, Node<T>)> {
        let mut i = 0;
        while i < node.keys.len() && val > node.keys[i] {
            i += 1;
        }

        if i < node.keys.len() && val == node.keys[i] {
            return None;
        }

        if node.is_leaf {
            node.keys.insert(i, val);
        } else {
            let child_split = Self::insert_rec(&mut node.children[i], val, order);

            if let Some((median, right_node)) = child_split {
                node.keys.insert(i, median);
                node.children.insert(i + 1, right_node);
            }
        }

        if node.keys.len() == order {
            return Some(Self::split(node));
        }

        None
    }

    fn split(node: &mut Node<T>) -> (T, Node<T>) {
        let mid_index = node.keys.len() / 2;
        let median = node.keys.remove(mid_index);

        let right_keys = node.keys.split_off(mid_index);

        let right_children = if node.is_leaf {
            Vec::new()
        } else {
            node.children.split_off(mid_index + 1)
        };

        let right_node = Node {
            keys: right_keys,
            children: right_children,
            is_leaf: node.is_leaf,
        };

        (median, right_node)
    }
}
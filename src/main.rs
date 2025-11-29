use lattice::trees::binary_tree::BinaryTree;

fn main() {
    println!("=== Rust Data Structures: Binary Search Tree ===");

    let mut bst = BinaryTree::new();
    let data = vec![14, 10, 17, 12, 11, 20, 18, 25, 8, 22, 23, 7, 13];

    println!("Inserting: {:?}", data);
    for &val in &data {
        bst.insert(val);
    }

    println!("\n--- Metrics ---");
    println!("Height: {}", bst.height());
    println!("Total Nodes: {}", bst.count_nodes());

    println!("\n--- Traversals ---");
    bst.print_inorder();
    bst.print_preorder();
}

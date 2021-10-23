use lab4::Tree;

fn main() {
    let mut bst = Tree::new();

    bst.insert(15).expect("Failed to insert");
    bst.insert(10).expect("Failed to insert");
    bst.insert(20).expect("Failed to insert");
    bst.insert(20).expect_err("Value was already in the tree");
    bst.insert(8).expect("Failed to insert");
    bst.insert(12).expect("Failed to insert");
    bst.insert(18).expect("Failed to insert");
    bst.insert(30).expect("Failed to insert");
    bst.insert(16).expect("Failed to insert");
    bst.insert(19).expect("Failed to insert");
    println!("{:#?}", bst);

    bst.delete(&20).expect("Failed to delete");
    bst.delete(&20).expect_err("Value is not in the tree");
    println!("{:#?}", bst);
}

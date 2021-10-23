use std::env;
use std::fs;
use std::fmt::Debug;

type Edge<T> = Option<Box<BinarySearchTree<T>>>;

#[derive(Debug)]
struct BinarySearchTree<T> {
    elem: Option<T>,
    left: Edge<T>,
    right: Edge<T>,
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Debug
{
    pub fn new() -> Self {
        BinarySearchTree {
            elem: None,
            left: None,
            right: None,
        }
    }

    pub fn leaf(elem: T) -> Self {
        BinarySearchTree {
            elem: Some(elem),
            left: None,
            right: None,
        }
    }

    pub fn push(&mut self, new_elem: T) {
        match &self.elem {
            Some(elem) => {
                let next = if new_elem < *elem {
                    &mut self.left
                } else {
                    &mut self.right
                };

                match next {
                    Some(ref mut n) => n.push(new_elem),
                    None => {
                        let mut new_node = BinarySearchTree::leaf(new_elem);
                        *next = Some(Box::new(new_node));
                    }
                }
            }
            None => self.elem = Some(new_elem)
        }
    }

    pub fn contains(&self, target_elem: T) -> bool {
        match &self.elem {
            Some(elem) => {
                if target_elem == *elem {
                    true
                } else {
                    let next = if target_elem < *elem {
                        &self.left
                    } else {
                        &self.right
                    };

                    match next {
                        Some(ref node) => node.contains(target_elem),
                        None => false
                    }
                }
            }
            None => false
        }
    }

    pub fn print(&self, spacing: u32) {
        if let Some(right) = &self.right {
            right.print(spacing + 1);
        }

        for _ in 0..spacing {
            print!("    ");
        }
        println!("{:?}", &self.elem);

        if let Some(left) = &self.left {
            left.print(spacing + 1);
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("usage: {} file.txt", args[0]);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename)
        .expect("cannot read file");

    let mut btree = BinarySearchTree::new();

    for i in contents.split_whitespace() {
        let value: u32 = i.parse().expect("value is not a number");

        btree.push(value);
    }

    btree.print(0);

    println!("42 is in the BST: {}", btree.contains(42));
    println!("19 is in the BST: {}", btree.contains(19));
}

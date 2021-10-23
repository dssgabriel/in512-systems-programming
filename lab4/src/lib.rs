use std::cmp::Ordering;
use std::fmt::Debug;

/// Simple binary search tree.
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
#[derive(Debug)]
pub struct Tree<T>(Option<Box<Node<T>>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Tree<T>,
    right: Tree<T>,
}

/// Errors thrown by operations on a Tree.
#[derive(Debug)]
pub enum TreeOpError {
    NoValue,
    ValueAlreadyExists,
    NoneTree,
}

impl<T> Tree<T>
where
    T: Ord + Debug,
{
    /// Returns an empty tree.
    pub fn new() -> Self {
        Tree(None)
    }

    /// Returns a tree containing a single value.
    fn leaf(value: T) -> Self {
        Tree(Some(Box::new(Node {
            value,
            left: Tree(None),
            right: Tree(None),
        })))
    }

    /// Inserts `value` into the tree.
    /// Returns `TreeOpError::ValueAlreadyExists` iff the `value` was already
    /// contained in the tree.
    pub fn insert(&mut self, value: T) -> Result<(), TreeOpError> {
        match self.0 {
            Some(ref mut n) => match value.cmp(&n.value) {
                Ordering::Equal => Err(TreeOpError::ValueAlreadyExists),
                Ordering::Less => n.left.insert(value),
                Ordering::Greater => n.right.insert(value),
            },
            None => {
                *self = Tree::leaf(value);
                Ok(())
            }
        }
    }

    /// Returns true iff `value` belongs to the tree.
    pub fn contains(&self, target: T) -> bool {
        match self.0 {
            Some(ref n) => match target.cmp(&n.value) {
                Ordering::Equal => true,
                Ordering::Less => n.left.contains(target),
                Ordering::Greater => n.right.contains(target),
            },
            None => false,
        }
    }

    /// Inserts `tree` at the corresponding place in `self`.
    /// Returns a `TreeOpError` Result if the function failed to insert `tree`.
    pub fn add_to_end(&mut self, tree: Tree<T>) -> Result<(), TreeOpError> {
        // Get the value at the root of `tree`
        let target = if tree.0.is_some() {
            &tree.0.as_ref().unwrap().value
        } else {
            return Err(TreeOpError::NoneTree);
        };

        match self.0 {
            Some(ref mut n) => match target.cmp(&n.value) {
                Ordering::Equal => Err(TreeOpError::ValueAlreadyExists),
                Ordering::Less => n.left.add_to_end(tree),
                Ordering::Greater => n.right.add_to_end(tree),
            },
            None => {
                *self = tree;
                Ok(())
            }
        }
    }

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `TreeOpError::NoValue` is returned.
    pub fn delete(&mut self, target: &T) -> Result<(), TreeOpError> {
        // Destructure `n` thanks to default binding modes and get mutable references
        // on each field of `n`
        //
        // # Example:
        // ```
        // Some(n) = n,          // `n` is of type `&mut Box<Node<T>>`
        // Some(n) = n.as_mut(), // `n` is of type `&mut Node<T>`
        // ```
        // We can then destructure `n` into its fiels: value, left and right
        let Node { value, left, right } = match &mut self.0 {
            Some(n) => n.as_mut(),
            None => return Err(TreeOpError::NoValue),
        };

        match target.cmp(value) {
            Ordering::Equal => {
                match left.inorder_predecessor() {
                    Some(predecessor) => *value = predecessor,
                    None => self.0 = right.0.take(),
                }
                Ok(())
            }
            Ordering::Less => left.delete(target),
            Ordering::Greater => right.delete(target),
        }
    }

    pub fn inorder_predecessor(&mut self) -> Option<T> {
        match &mut self.0 {
            Some(n) => match n.right.inorder_predecessor() {
                Some(child) => Some(child),
                None => {
                    let predecessor = self.0.take().unwrap();
                    self.0 = predecessor.left.0;
                    Some(predecessor.value)
                }
            },
            None => None,
        }
    }
}

impl<T> Default for Tree<T>
where
    T: Ord + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PartialEq for Tree<T>
where
    T: PartialEq,
{
    /// Required to be able to compare `Tree`s together.
    fn eq(&self, other: &Self) -> bool {
        match (self.0.as_ref(), other.0.as_ref()) {
            (Some(a), Some(b)) => a.value == b.value,
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let bst: Tree<i32> = Tree::new();
        assert_eq!(bst, Tree::<i32>::new());
    }

    #[test]
    fn inserts() {
        let mut bst = Tree::new();
        assert!(bst.insert(2).is_ok());
    }

    #[test]
    fn leaf() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        assert_eq!(bst, Tree::leaf(2));
    }

    #[test]
    fn contain() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        bst.insert(23).expect("Failed to insert");
        bst.insert(20).expect("Failed to insert");
        assert_eq!(bst.contains(23), true);
        assert_eq!(bst.contains(29), false);
    }

    #[test]
    fn remove() {
        let mut bst = Tree::new();
        bst.insert(2).expect("Failed to insert");
        bst.insert(23).expect("Failed to insert");
        bst.insert(20).expect("Failed to insert");
        assert!(bst.delete(&2).is_ok());
        assert!(bst.delete(&23).is_ok());
        assert!(bst.delete(&2).is_err());
    }
}

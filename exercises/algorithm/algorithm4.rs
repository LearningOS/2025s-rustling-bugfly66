/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {

        let mut p = &mut self.root;
        if p.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }
        // if self.root.se
        while p.is_some() {
            if p.as_ref().unwrap().value < value {
                if p.as_ref().unwrap().right.is_some(){
                    p = &mut p.as_mut().unwrap().right;
                }else {
                    break
                }
            }else if p.as_ref().unwrap().value == value {
               // break;
                return;
            }else {
                if p.as_ref().unwrap().left.is_some() {
                    p = &mut p.as_mut().unwrap().left;
                }else {
                    break
                }
            }
        }
        if p.as_ref().unwrap().value <value{
            p.as_mut().unwrap().right = Some(Box::new(TreeNode::new(value)));
        }else {
            p.as_mut().unwrap().left = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let mut p = &self.root;
        while p.is_some() {
            if p.as_ref().unwrap().value < value {
                p = &p.as_ref().unwrap().right;
            }else if p.as_ref().unwrap().value == value {
                return true;
            }else {
                p = &p.as_ref().unwrap().left;
            }
        }
        return false;
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        let  node = Box::new(TreeNode::new(value));
       if self.value < node.value {
           self.right = Some(node);
       } else {
           self.left = Some(node);
       }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}

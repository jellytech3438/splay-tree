pub mod node;
pub mod tree;

pub use node::Node;
pub use node::SplayNode;
pub use tree::SplayTree;
pub use tree::Splayable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_example() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(1));
        splay_tree.insert(&mut Node::new(2));
        splay_tree.insert(&mut Node::new(3));
        splay_tree.insert(&mut Node::new(4));
        assert_eq!(splay_tree.root.as_ref().unwrap().borrow().key, 4);
    }

    #[test]
    fn delete_example() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(10));
        splay_tree.insert(&mut Node::new(15));
        splay_tree.insert(&mut Node::new(0));
        splay_tree.delete(15);
        assert_ne!(splay_tree.root.as_ref().unwrap().borrow().key, 15);
    }

    #[test]
    fn left_most() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(1));
        splay_tree.insert(&mut Node::new(2));
        splay_tree.insert(&mut Node::new(3));
        splay_tree.insert(&mut Node::new(4));
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(6));
        assert_eq!(splay_tree.pop_left_most().as_ref().unwrap().borrow().key, 1);
        assert_eq!(splay_tree.pop_left_most().as_ref().unwrap().borrow().key, 2);
        assert_eq!(splay_tree.pop_left_most().as_ref().unwrap().borrow().key, 3);
        assert_eq!(splay_tree.pop_left_most().as_ref().unwrap().borrow().key, 4);
    }

    #[test]
    fn same_key() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(10));
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(3));
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(5));
        assert_eq!(splay_tree.pop_left_most().as_ref().unwrap().borrow().key, 3);
    }

    #[test]
    fn splay_nearest() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(5));
        splay_tree.insert(&mut Node::new(2));
        splay_tree.insert(&mut Node::new(10));
        splay_tree.insert(&mut Node::new(7));
        splay_tree.insert(&mut Node::new(9));
        splay_tree.insert(&mut Node::new(1));
        splay_tree.insert(&mut Node::new(3));
        splay_tree.splay(8);
        assert_eq!(splay_tree.root.as_ref().unwrap().borrow().key, 9);
    }

    #[test]
    fn into_iter() {
        let mut splay_tree = SplayTree::new();
        splay_tree.insert(&mut Node::new(4));
        splay_tree.insert(&mut Node::new(2));
        splay_tree.insert(&mut Node::new(8));
        splay_tree.insert(&mut Node::new(10));
        splay_tree.insert(&mut Node::new(9));
        splay_tree.insert(&mut Node::new(7));
        splay_tree.insert(&mut Node::new(6));
        let ans = [2, 4, 6, 7, 8, 9, 10];
        for (i, v) in splay_tree.into_iter().enumerate() {
            assert_eq!(v.as_ref().borrow().key, ans[i]);
        }
    }
}

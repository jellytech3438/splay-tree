use crate::Node;
use crate::SplayNode;

use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

pub trait Splayable<K> {
    fn splay(&mut self, key: K);
}

#[derive(Clone, Debug)]
pub struct SplayTree<K> {
    pub root: SplayNode<K>,
}

#[derive(Clone, Copy, Debug)]
enum SplayCase {
    Merge,
    LeftRotate,
    RightRotate,
    ZigZigLeft,
    ZigZigRight,
    ZigZagLeft,
    ZigZagRight,
    CaseError,
}

fn splay_case<K: Ord>(nodeptr: &SplayNode<K>, key: K) -> SplayCase {
    if nodeptr.as_ref().unwrap().as_ref().borrow().key == key {
        return SplayCase::Merge;
    } else if nodeptr.as_ref().unwrap().borrow().key < key {
        // current node is smaller than target, so we head for right tree
        if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .is_some_and(|x| x.borrow().key == key)
        {
            return SplayCase::RightRotate;
        } else if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .is_some_and(|x| x.borrow().key > key)
        {
            return SplayCase::ZigZagRight;
        } else if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .is_some_and(|x| x.borrow().key < key)
        {
            return SplayCase::ZigZigRight;
        } else {
            return SplayCase::Merge;
        }
    } else if nodeptr.as_ref().unwrap().borrow().key > key {
        // current node is larger than target, so we head for left tree
        if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .is_some_and(|x| x.borrow().key == key)
        {
            return SplayCase::LeftRotate;
        } else if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .is_some_and(|x| x.borrow().key > key)
        {
            return SplayCase::ZigZigLeft;
        } else if nodeptr
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .is_some_and(|x| x.borrow().key < key)
        {
            return SplayCase::ZigZagLeft;
        } else {
            return SplayCase::Merge;
        }
    }
    SplayCase::CaseError
}

impl<K: Ord + Clone + Debug> SplayTree<K> {
    pub fn new() -> Self {
        SplayTree { root: None }
    }

    pub fn insert(&mut self, inserted: &mut Rc<RefCell<Node<K>>>) {
        if self.root.is_none() {
            self.root = Some(inserted.to_owned());
            return;
        }

        self.root.as_ref().unwrap().borrow_mut().bstinsert(inserted);

        // BUG: self.splay(inserted.borrow().key.clone())
        let key = inserted.borrow().key.clone();
        self.splay(key);
    }

    pub fn delete(&mut self, key: K) {
        if self.root.is_none() {
            return;
        }

        self.splay(key.clone());
        if self.root.as_ref().unwrap().borrow().key != key {
            return;
        }

        let delete_node = mem::take(&mut self.root);
        let left_tree = mem::take(&mut delete_node.as_ref().unwrap().borrow_mut().left);
        let right_tree = mem::take(&mut delete_node.as_ref().unwrap().borrow_mut().right);

        let left_right_most = left_tree.as_ref().unwrap().borrow().right_most_key();
        self.root = left_tree;
        self.splay(left_right_most);
        self.root.as_ref().unwrap().borrow_mut().right = right_tree;
    }

    pub fn pop_left_most(&mut self) -> SplayNode<K> {
        if self.root.is_none() {
            return None;
        }

        // Rc clone
        let mut current = &mut self.root.clone();
        let mut parent: &mut SplayNode<K> = &mut None;
        loop {
            if current.as_ref().unwrap().borrow().left.is_none() && parent.is_none() {
                let left_most = mem::take(&mut self.root);
                let right_tree = mem::take(&mut left_most.as_ref().unwrap().borrow_mut().right);
                self.root = right_tree;
                return left_most;
            } else if current.as_ref().unwrap().borrow().left.is_none() && parent.is_some() {
                let left_most = mem::take(current);
                let right_tree = mem::take(&mut left_most.as_ref().unwrap().borrow_mut().right);
                parent.as_ref().unwrap().borrow_mut().left = right_tree;
                return left_most;
            } else if current.as_ref().unwrap().borrow().left.is_some() {
                let left_tree = mem::take(&mut current.as_ref().unwrap().borrow_mut().left);
                let mut old_current = mem::replace(current, left_tree);
                if parent.is_some() {
                    mem::swap(parent, &mut old_current);
                    old_current.as_mut().unwrap().borrow_mut().left = parent.clone();
                    *parent = old_current.as_ref().unwrap().borrow_mut().left.clone();
                } else {
                    *parent = old_current;
                }
            }
        }
    }
}

impl<K: Ord + Clone + Debug> IntoIterator for SplayTree<K> {
    type Item = Rc<RefCell<Node<K>>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        let mut iter: Vec<Self::Item> = Vec::new();

        while let Some(node) = SplayTree::pop_left_most(&mut self) {
            iter.push(node);
        }

        iter.into_iter()
    }
}

// implementation of top-down splay algorithm based on:
//      http://ccf.ee.ntu.edu.tw/~yen/courses/ds17/chapter-4c.pdf
impl<K: Ord + Clone + Debug> Splayable<K> for SplayTree<K> {
    fn splay(&mut self, key: K) {
        if self.root.is_none() {
            return;
        }

        let mut new_left_tree = None;
        let mut new_right_tree = None;
        let mut nodeptr = mem::take(&mut self.root);

        loop {
            match splay_case(&nodeptr, key.clone()) {
                SplayCase::Merge => {
                    // NOTE: merge new_left_tree, new_right_tree to nodeptr X
                    //
                    // L     X    R                X
                    //  \   /\   /   =>          /  \
                    //   a b  c d              a     d
                    //                          \   /
                    //                           b c
                    let mut right_tree =
                        mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().right);
                    let mut left_tree = mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().left);

                    if new_right_tree.is_none() {
                        new_right_tree = right_tree;
                    } else {
                        new_right_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(right_tree);
                    }

                    if new_left_tree.is_none() {
                        new_left_tree = left_tree;
                    } else {
                        new_left_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_right_most(left_tree);
                    }

                    nodeptr.as_ref().unwrap().borrow_mut().right = new_right_tree;
                    nodeptr.as_ref().unwrap().borrow_mut().left = new_left_tree;

                    self.root = nodeptr;

                    break;
                }
                SplayCase::LeftRotate => {
                    // NOTE:find key X, left rotate X to root
                    //
                    // L     Y    R          L     X       R
                    //      /\      =>            / \     /
                    //     X  c                  a   b   Y
                    //    /\                              \
                    //   a  b                              c
                    let mut left_tree = mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().left);

                    if new_right_tree.is_none() {
                        new_right_tree = nodeptr;
                    } else {
                        new_right_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(nodeptr);
                    }

                    nodeptr = left_tree;
                }
                SplayCase::RightRotate => {
                    // NOTE:find key X, right rotate X to root
                    //
                    // L     Y    R         L       X       R
                    //      /\      =>       \     / \
                    //     c  X               Y   a   b
                    //       /\                \
                    //      a  b                c
                    let mut right_tree =
                        mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().right);

                    if new_left_tree.is_none() {
                        new_left_tree = nodeptr;
                    } else {
                        new_left_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_right_most(nodeptr);
                    }

                    nodeptr = right_tree;
                }
                SplayCase::ZigZigLeft => {
                    // NOTE:find key X, zig zig left X to root
                    //
                    // L     Z    R          L     X       R
                    //      /\      =>            / \     /
                    //     Y  d                  a   b   Y
                    //    /\                              \
                    //   X  c                              Z
                    //                                    / \
                    //                                   c  d
                    let mut left_tree = mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().left);
                    let mut left_left_tree =
                        mem::take(&mut left_tree.as_ref().unwrap().borrow_mut().left);
                    let mut left_right_tree =
                        mem::take(&mut left_tree.as_ref().unwrap().borrow_mut().right);

                    nodeptr.as_ref().unwrap().borrow_mut().left = left_right_tree;
                    left_tree.as_ref().unwrap().borrow_mut().right = nodeptr;

                    if new_right_tree.is_none() {
                        new_right_tree = left_tree;
                    } else {
                        new_right_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(left_tree);
                    }

                    nodeptr = left_left_tree;
                }
                SplayCase::ZigZigRight => {
                    // NOTE: find key X, zig zig right X to root
                    //
                    // L     Z    R          L      X       R
                    //      /\         =>    \     / \
                    //     d  Y               Y   a   b
                    //       /\              /
                    //      c  X            Z
                    //                     / \
                    //                    d   c
                    let mut right_tree =
                        mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().right);
                    let mut right_right_tree =
                        mem::take(&mut right_tree.as_ref().unwrap().borrow_mut().right);
                    let mut right_left_tree =
                        mem::take(&mut right_tree.as_ref().unwrap().borrow_mut().left);

                    nodeptr.as_ref().unwrap().borrow_mut().right = right_left_tree;
                    right_tree.as_ref().unwrap().borrow_mut().left = nodeptr;

                    if new_left_tree.is_none() {
                        new_left_tree = right_tree;
                    } else {
                        new_left_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_right_most(right_tree);
                    }

                    nodeptr = right_right_tree;
                }
                SplayCase::ZigZagLeft => {
                    // NOTE:find key X, zig zag left X to root
                    //
                    // L     Z    R          L       X       R
                    //      /\         =>    \      / \     /
                    //     Y  d               Y    a   b   Z
                    //    /\                  /            \
                    //   c  X                c              d
                    let mut left_tree = mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().left);
                    let mut left_right_tree =
                        mem::take(&mut left_tree.as_ref().unwrap().borrow_mut().right);

                    if new_left_tree.is_none() {
                        new_left_tree = left_tree;
                    } else {
                        new_left_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(left_tree);
                    }
                    if new_right_tree.is_none() {
                        new_right_tree = nodeptr;
                    } else {
                        new_right_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(nodeptr);
                    }

                    nodeptr = left_right_tree;
                }
                SplayCase::ZigZagRight => {
                    // NOTE: find key X, zig zag right X to root
                    //
                    // L     Z    R          L       X       R
                    //      /\         =>    \      / \     /
                    //     c  Y               Z    a   b   Y
                    //       /\               /            \
                    //      X  d             c              d
                    let mut right_tree =
                        mem::take(&mut nodeptr.as_ref().unwrap().borrow_mut().right);
                    let mut right_left_tree =
                        mem::take(&mut right_tree.as_ref().unwrap().borrow_mut().left);

                    if new_right_tree.is_none() {
                        new_right_tree = right_tree;
                    } else {
                        new_right_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_left_most(right_tree);
                    }
                    if new_left_tree.is_none() {
                        new_left_tree = nodeptr;
                    } else {
                        new_left_tree
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .insert_right_most(nodeptr);
                    }

                    nodeptr = right_left_tree;
                }
                _ => {
                    println!("Case Error");
                    break;
                }
            }
        }
    }
}

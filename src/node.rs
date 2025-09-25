use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Node<K> {
    pub left: SplayNode<K>,
    pub right: SplayNode<K>,
    pub key: K,
}

pub type SplayNode<K> = Option<Rc<RefCell<Node<K>>>>;

impl<K: Ord> PartialEq for Node<K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.left == other.left && self.right == other.right
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<K: Ord + Clone + Debug> Node<K> {
    pub fn new(k: K) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            left: None,
            right: None,
            key: k,
        }))
    }

    pub fn insert_left_most(&mut self, inserted: SplayNode<K>) {
        if inserted.is_none() {
            return;
        }
        let mut temp = self;

        if let Some(ref mut current) = temp.left {
            if current.borrow().left.is_none() {
                current.borrow_mut().left = inserted;
            } else {
                current.borrow_mut().insert_left_most(inserted);
            }
        } else {
            temp.left = inserted;
        }
    }

    pub fn insert_right_most(&mut self, inserted: SplayNode<K>) {
        if inserted.is_none() {
            return;
        }
        let mut temp = self;

        if let Some(ref mut current) = temp.right {
            if current.borrow().right.is_none() {
                current.borrow_mut().right = inserted;
            } else {
                current.borrow_mut().insert_right_most(inserted);
            }
        } else {
            temp.right = inserted;
        }
    }

    pub fn left_most_key(&self) -> K {
        if let Some(ref left) = self.left {
            return left.borrow().left_most_key();
        } else {
            return self.key.clone();
        }
    }

    pub fn right_most_key(&self) -> K {
        if let Some(ref right) = self.right {
            return right.borrow().right_most_key();
        } else {
            return self.key.clone();
        }
    }

    pub fn bstinsert(&mut self, inserted: &mut Rc<RefCell<Node<K>>>) {
        let key = inserted.borrow().key.clone();
        if self.key == key {
            let mut temp = mem::take(&mut self.left);

            // new node inserted to self's left
            self.left = Some(inserted.clone());

            // update nodes
            self.left.as_mut().unwrap().borrow_mut().left = temp;
        } else if self.key > key {
            if self.left.is_some() {
                self.left.as_mut().unwrap().borrow_mut().bstinsert(inserted);
            } else {
                // insert node as left child
                self.left = Some(inserted.clone());
            }
        } else if self.key < key {
            if self.right.is_some() {
                self.right
                    .as_mut()
                    .unwrap()
                    .borrow_mut()
                    .bstinsert(inserted);
            } else {
                // insert node as right child
                self.right = Some(inserted.clone());
            }
        }
    }
}

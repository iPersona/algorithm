use std::cmp::max;
use std::fmt::{Debug, Display};
use std::mem::swap;

pub enum BST<T: PartialOrd + Clone + Debug + Display> {
    Node(T, Box<BST<T>>, Box<BST<T>>),
    Empty,
}

// enum MatchedChild {
//     Left,
//     Right,
//     Non,
// }

impl<T: PartialOrd + Clone + Debug + Display> BST<T> {
    pub fn new() -> BST<T> {
        BST::Empty
    }

    pub fn depth(&self) -> usize {
        match *self {
            BST::Empty => 0,
            BST::Node(_, box ref l, box ref r) => 1 + max(l.depth(), r.depth()),
        }
    }

    pub fn insert(&mut self, value: T) -> &mut Self {
        match *self {
            BST::Empty => *self = BST::Node(value, Box::new(BST::Empty), Box::new(BST::Empty)),
            BST::Node(ref old_value, box ref mut l, box ref mut r) => {
                if value < *old_value {
                    l.insert(value);
                } else {
                    r.insert(value);
                }
            }
        }
        self
    }

    // fn left(&self) -> Option<&Self> {
    //     match self {
    //         &BST::Empty => None,
    //         &BST::Node(_, box ref l, _) => match l {
    //             // &AVLTree::Empty => None,
    //             _ => Some(l),
    //         },
    //     }
    // }

    // fn left_mut(&mut self) -> Option<&mut Self> {
    //     match self {
    //         &mut BST::Empty => None,
    //         &mut BST::Node(_, box ref mut l, _) => match l {
    //             // &AVLTree::Empty => None,
    //             _ => Some(l),
    //         },
    //     }
    // }

    // fn right_mut(&mut self) -> Option<&mut Self> {
    //     match self {
    //         &mut BST::Empty => None,
    //         &mut BST::Node(_, _, box ref mut r) => match r {
    //             // &AVLTree::Empty => None,
    //             _ => Some(r),
    //         },
    //     }
    // }

    pub fn remove(&mut self, value: T) -> &mut Self {
        let child_num = self.get_child_num();
        let mut node = BST::Empty;
        let mut is_swaped = false;
        let is_leaf = self.is_leaf();
        match *self {
            BST::Empty => {}
            BST::Node(ref mut v, box ref mut l, box ref mut r) => {
                // println!("childs of <{:?}>={:?}", (*v).clone(), child_num);
                if value < *v {
                    l.remove(value);
                } else if value > *v {
                    r.remove(value);
                } else {
                    if is_leaf && *v == value {
                        // a leaf node
                        is_swaped = true;
                    } else if child_num == 2 {
                        // 2 childs, put min of right branch as the current value,
                        // then remove it from right branch
                        *v = r.find_min().unwrap();
                        r.remove(v.clone());
                    } else {
                        if !l.is_empty() {
                            is_swaped = true;
                            swap(&mut node, l);
                        } else if !r.is_empty() {
                            is_swaped = true;
                            swap(&mut node, r);
                        } else {
                            // will not be reached.
                        }
                    }
                }
            }
        }
        if is_swaped {
            *self = node;
        }

        self
    }

    fn left(&self) -> Option<&Self> {
        match self {
            &BST::Empty => None,
            &BST::Node(_, box ref l, _) => match l {
                _ => Some(l),
            },
        }
    }

    fn right(&self) -> Option<&Self> {
        match self {
            &BST::Empty => None,
            &BST::Node(_, _, box ref r) => match r {
                _ => Some(r),
            },
        }
    }

    pub fn get_value(&self) -> Option<T> {
        match self {
            &BST::Empty => None,
            &BST::Node(ref v, _, _) => Some(v.clone()),
        }
    }

    fn value(&self) -> Option<&T> {
        match self {
            &BST::Empty => None,
            &BST::Node(ref v, _, _) => Some(v),
        }
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            BST::Empty => true,
            _ => false,
        }
    }

    pub fn get_child_num(&self) -> usize {
        let mut num = 0;
        match self {
            &BST::Empty => num = 0,
            &BST::Node(_, box ref l, box ref r) => {
                if !l.is_empty() {
                    num += 1;
                }
                if !r.is_empty() {
                    num += 1;
                }
            }
        }
        num
    }

    pub fn is_left_empty(&self) -> bool {
        match self {
            &BST::Empty => true,
            &BST::Node(_, box ref l, _) => l.is_empty(),
        }
    }

    pub fn is_right_empty(&self) -> bool {
        match self {
            &BST::Empty => true,
            &BST::Node(_, _, box ref r) => r.is_empty(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            &BST::Empty => false,
            _ => self.get_child_num() == 0,
        }
    }

    pub fn find_min(&self) -> Option<T> {
        match self {
            &BST::Empty => None,
            &BST::Node(ref v, box ref l, _) => {
                if l.is_empty() {
                    Some(v.clone())
                } else {
                    l.find_min()
                }
            }
        }
    }

    pub fn find_max(&self) -> Option<T> {
        match self {
            &BST::Empty => None,
            &BST::Node(ref v, _, box ref r) => {
                if r.is_empty() {
                    Some(v.clone())
                } else {
                    r.find_max()
                }
            }
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self {
            &BST::Empty => false,
            &BST::Node(ref v, box ref l, box ref r) => {
                if value == *v {
                    true
                } else if value > *v {
                    r.contains(value)
                } else {
                    l.contains(value)
                }
            }
        }
    }

    pub fn pre_order(&self) -> Option<Vec<&T>> {
        if self.is_empty() {
            return None;
        }

        let mut tmp = Vec::new();
        tmp.push(self);
        let mut nodes = Vec::new();
        loop {
            if tmp.is_empty() {
                break;
            }

            // stack first in last out
            let n = tmp.pop().unwrap();
            nodes.push(n.value().unwrap());

            // right poped out first
            if !n.is_right_empty() {
                tmp.push(&n.right().unwrap())
            }

            if !n.is_left_empty() {
                tmp.push(&n.left().unwrap())
            }
        }

        Some(nodes)
    }

    pub fn in_order(&self) -> Option<Vec<&T>> {
        if self.is_empty() {
            return None;
        }

        let mut tmp = Vec::new();
        let mut nodes = Vec::new();

        let mut p = self;
        loop {
            if p.is_empty() && tmp.is_empty() {
                break;
            }

            // push left branch
            loop {
                if p.is_empty() {
                    break;
                }

                tmp.push(p);
                p = p.left().unwrap();
            }

            p = tmp.pop().unwrap();
            nodes.push(p.value().unwrap()); // left, current, right
            p = p.right().unwrap();
        }

        Some(nodes)
    }

    pub fn post_order(&self) -> Option<Vec<&T>> {
        if self.is_empty() {
            return None;
        }

        let mut tmp = Vec::new();
        let mut nodes = Vec::new();

        tmp.push(self);
        loop {
            // 2. current > right > left
            let p = tmp.pop();
            if p.is_none() {
                break;
            }

            // 1. current > left > right
            let p = p.unwrap();
            nodes.push(p.value().unwrap());

            if !p.is_left_empty() {
                tmp.push(p.left().unwrap());
            }

            if !p.is_right_empty() {
                tmp.push(p.right().unwrap());
            }
        }

        // 3. left > right > current
        Some(nodes.iter().rev().map(|&x| x).collect::<Vec<&T>>())
    }
}

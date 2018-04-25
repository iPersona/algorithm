use std::cmp::max;
use std::fmt::{Debug, Display};
use std::mem::swap;

const ALLOWED_IMBALANCE: isize = 1;

enum RotateCase {
    RotateLeftChild,  // LL
    RotateRightChild, // RR
    DoubleLeftChild,  // LR
    DoubleRightChild, // RL
    Unknown,
}

#[derive(Clone)]
pub enum AVLTree<T: PartialOrd + Clone + Debug + Display> {
    // value, height, left, right
    Node(T, isize, Box<AVLTree<T>>, Box<AVLTree<T>>),
    Empty,
}

impl<T: PartialOrd + Clone + Debug + Display> AVLTree<T> {
    pub fn new() -> AVLTree<T> {
        AVLTree::Empty
    }

    pub fn depth(&self) -> usize {
        match *self {
            AVLTree::Empty => 0,
            AVLTree::Node(_, _, box ref l, box ref r) => 1 + max(l.depth(), r.depth()),
        }
    }

    pub fn height(&self) -> isize {
        match *self {
            AVLTree::Empty => 0,
            AVLTree::Node(_, ref h, _, _) => *h,
        }
    }

    fn left(&self) -> Option<&Self> {
        match self {
            &AVLTree::Empty => None,
            &AVLTree::Node(_, _, box ref l, _) => match l {
                // &AVLTree::Empty => None,
                _ => Some(l),
            },
        }
    }

    fn right(&self) -> Option<&Self> {
        match self {
            &AVLTree::Empty => None,
            &AVLTree::Node(_, _, _, box ref r) => match r {
                // &AVLTree::Empty => None,
                _ => Some(r),
            },
        }
    }

    fn left_mut(&mut self) -> Option<&mut Self> {
        match self {
            &mut AVLTree::Empty => None,
            &mut AVLTree::Node(_, _, box ref mut l, _) => match l {
                // &mut AVLTree::Empty => None,
                _ => Some(l),
            },
        }
    }

    fn right_mut(&mut self) -> Option<&mut Self> {
        match self {
            &mut AVLTree::Empty => None,
            &mut AVLTree::Node(_, _, _, box ref mut r) => match r {
                // &mut AVLTree::Empty => None,
                _ => Some(r),
            },
        }
    }

    fn balance(&mut self) {
        let mut rotate_case = RotateCase::Unknown;
        match self {
            &mut AVLTree::Empty => return,
            &mut AVLTree::Node(_, _, box ref l, box ref r) => {
                if (l.height() - r.height()) > ALLOWED_IMBALANCE {
                    let ll = l.left().unwrap();
                    let lr = l.right().unwrap();
                    if ll.height() >= lr.height() {
                        rotate_case = RotateCase::RotateLeftChild;
                    } else {
                        rotate_case = RotateCase::DoubleLeftChild;
                    }
                } else if r.height() - l.height() > ALLOWED_IMBALANCE {
                    let rr = r.right().unwrap();
                    let rl = r.left().unwrap();
                    if rr.height() > rl.height() {
                        rotate_case = RotateCase::RotateRightChild;
                    } else {
                        rotate_case = RotateCase::DoubleRightChild;
                    }
                } else {
                    // the tree is already balanced
                }
            }
        }

        match rotate_case {
            RotateCase::RotateLeftChild => self.rotate_left_child(),
            RotateCase::DoubleLeftChild => self.double_left_child(),
            RotateCase::RotateRightChild => self.rotate_right_child(),
            RotateCase::DoubleRightChild => self.double_right_child(),
            _ => {}
        }
        self.update_height();
    }

    fn rotate_left_child(&mut self) {
        let mut x: AVLTree<T> = AVLTree::Empty;
        let mut y: AVLTree<T> = AVLTree::Empty;
        let mut z: AVLTree<T> = AVLTree::Empty;
        let mut k1: AVLTree<T>;
        let mut k2: AVLTree<T>;
        let k1_v: T;
        let k2_v: T;

        match self {
            &mut AVLTree::Empty => return,
            &mut AVLTree::Node(ref mut v, _, box ref mut l, box ref mut r) => {
                k1_v = l.get_value().unwrap();
                k2_v = v.clone();
                swap(l.left_mut().unwrap(), &mut x);
                swap(l.right_mut().unwrap(), &mut y);
                swap(r, &mut z);
            }
        }

        // generate lower k2 node
        k2 = AVLTree::Node(k2_v, 0, Box::new(y), Box::new(z));
        k2.update_height();

        // generate k1 node
        k1 = AVLTree::Node(k1_v, 0, Box::new(x), Box::new(k2));
        k1.update_height();

        *self = k1;
    }

    /// rotate right child (RR)
    /// Rust 由于声明周期的关系，不适合做类似指针赋值交换这样测操作
    /// 这里采用swap保存子树，然后对局部根节点做重组的方式进行
    fn rotate_right_child(&mut self) {
        let mut x: AVLTree<T> = AVLTree::Empty;
        let mut y: AVLTree<T> = AVLTree::Empty;
        let mut z: AVLTree<T> = AVLTree::Empty;
        let mut k1: AVLTree<T>;
        let mut k2: AVLTree<T>;
        let k1_v: T;
        let k2_v: T;

        match self {
            &mut AVLTree::Empty => return,
            &mut AVLTree::Node(ref mut v, _, box ref mut l, box ref mut r) => {
                k2_v = r.get_value().unwrap();
                k1_v = v.clone();
                swap(r.left_mut().unwrap(), &mut y);
                swap(r.right_mut().unwrap(), &mut z);
                swap(l, &mut x);
            }
        }

        // generate lower k2 node
        k1 = AVLTree::Node(k1_v, 0, Box::new(x), Box::new(y));
        k1.update_height();

        // generate lower k1 node
        k2 = AVLTree::Node(k2_v, 0, Box::new(k1), Box::new(z));
        k2.update_height();

        *self = k2;
    }

    fn double_left_child(&mut self) {
        match self {
            &mut AVLTree::Empty => return,
            &mut AVLTree::Node(_, _, box ref mut l, _) => {
                l.rotate_right_child();
            }
        }
        self.rotate_left_child();
    }

    fn double_right_child(&mut self) {
        match self {
            &mut AVLTree::Empty => return,
            &mut AVLTree::Node(_, _, _, box ref mut r) => {
                r.rotate_left_child();
            }
        }
        self.rotate_right_child();
    }

    pub fn insert(&mut self, value: T) -> &mut Self {
        match *self {
            AVLTree::Empty => {
                *self = AVLTree::Node(value, 1, Box::new(AVLTree::Empty), Box::new(AVLTree::Empty))
            }
            AVLTree::Node(ref old_value, _, box ref mut l, box ref mut r) => {
                if value < *old_value {
                    l.insert(value);
                } else {
                    r.insert(value);
                }
            }
        }
        self.balance(); // blaance tree

        self
    }

    pub fn remove(&mut self, value: T) -> &mut Self {
        let child_num = self.get_child_num();
        let mut node = AVLTree::Empty;
        let mut is_swaped = false;
        let is_leaf = self.is_leaf();
        match *self {
            AVLTree::Empty => {}
            AVLTree::Node(ref mut v, _, box ref mut l, box ref mut r) => {
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
        self.balance();

        self
    }

    pub fn get_value(&self) -> Option<T> {
        match self {
            &AVLTree::Empty => None,
            &AVLTree::Node(ref v, _, _, _) => Some(v.clone()),
        }
    }

    fn value(&self) -> Option<&T> {
        match self {
            &AVLTree::Empty => None,
            &AVLTree::Node(ref v, _, _, _) => Some(v),
        }
    }

    // fn set_left(&mut self, node: AVLTree<T>) -> &mut Self {
    //     match self {
    //         &mut AVLTree::Empty => {},
    //         &mut AVLTree::Node {value: ref mut _v, height: ref mut _h, left: box ref mut l, right: box ref mut _r} => {
    //             *l = node;
    //         }
    //     }
    //     self
    // }

    // fn set_right(&mut self, node: AVLTree<T>) -> &mut Self {
    //     match self {
    //         &mut AVLTree::Empty => {},
    //         &mut AVLTree::Node {value: ref mut _v, height: ref mut _h, left: box ref mut _l, right: box ref mut r} => {
    //             *r = node;
    //         }
    //     }
    //     self
    // }

    fn update_height(&mut self) -> &mut Self {
        match self {
            &mut AVLTree::Empty => {}
            &mut AVLTree::Node(_, ref mut h, box ref l, box ref r) => {
                *h = max(l.height(), r.height()) + 1;
            }
        }
        self
    }

    pub fn is_empty(&self) -> bool {
        match *self {
            AVLTree::Empty => true,
            _ => false,
        }
    }

    pub fn get_child_num(&self) -> usize {
        let mut num = 0;
        match self {
            &AVLTree::Empty => num = 0,
            &AVLTree::Node(_, _, box ref l, box ref r) => {
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
            &AVLTree::Empty => true,
            &AVLTree::Node(_, _, box ref l, _) => l.is_empty(),
        }
    }

    pub fn is_right_empty(&self) -> bool {
        match self {
            &AVLTree::Empty => true,
            &AVLTree::Node(_, _, _, box ref r) => r.is_empty(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            &AVLTree::Empty => false,
            _ => self.get_child_num() == 0,
        }
    }

    pub fn find_min(&self) -> Option<T> {
        match self {
            &AVLTree::Empty => None,
            &AVLTree::Node(ref v, _, box ref l, _) => {
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
            &AVLTree::Empty => None,
            &AVLTree::Node(ref v, _, _, box ref r) => {
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
            &AVLTree::Empty => false,
            &AVLTree::Node(ref v, _, box ref l, box ref r) => {
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

    // pub fn to_vec(&self) -> Option<Vec<T>> {
    //     match self {
    //         &AVLTree::Empty => None,
    //         &AVLTree::Node(ref v, _, box ref l, box ref r) => {
    //             let mut vs = Vec::new();
    //             vs.push(v.clone());
    //             if !l.is_empty() {
    //                 let lvs = l.to_vec();
    //                 match lvs {
    //                     Some(v) => vs.extend(v),
    //                     _ => {}
    //                 }
    //             }
    //             if !r.is_empty() {
    //                 let rvs = r.to_vec();
    //                 match rvs {
    //                     Some(v) => vs.extend(v),
    //                     _ => {}
    //                 }
    //             }
    //             Some(vs)
    //         }
    //     }
    // }

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

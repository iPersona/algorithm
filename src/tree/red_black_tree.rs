use std::cmp::{max, PartialEq};
use std::fmt::{Debug, Display, Formatter, Result};
use std::mem::swap;
// use utils::rb_tree_helper::{Branch, RBTreeHelper};

enum BalanceAction {
    RotateLeft,
    RotateRight,
    FlipColors,
    Unknown,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Black,
}

#[derive(Clone)]
pub enum RBNode<T: PartialOrd + Clone + Debug + Display> {
    /// Color means the color of incoming link of the node,
    /// NOT the color of node itself.
    ///
    /// //      |   <----- color means the color of this incoming link
    /// //      a
    /// //     / \
    /// //    b   c
    ///
    // value, color, left, right
    Node(T, Color, Box<RBNode<T>>, Box<RBNode<T>>),
    Empty,
}

impl<T: PartialOrd + Clone + Debug + Display> RBNode<T> {
    pub fn new() -> RBNode<T> {
        RBNode::Empty
    }

    pub fn depth(&self) -> usize {
        match *self {
            RBNode::Empty => 0,
            RBNode::Node(_, _, box ref l, box ref r) => 1 + max(l.depth(), r.depth()),
        }
    }

    fn left(&self) -> Option<&Self> {
        match self {
            &RBNode::Empty => None,
            &RBNode::Node(_, _, box ref l, _) => match l {
                _ => Some(l),
            },
        }
    }

    fn right(&self) -> Option<&Self> {
        match self {
            &RBNode::Empty => None,
            &RBNode::Node(_, _, _, box ref r) => match r {
                _ => Some(r),
            },
        }
    }

    fn left_mut(&mut self) -> Option<&mut Self> {
        match self {
            &mut RBNode::Empty => None,
            &mut RBNode::Node(_, _, box ref mut l, _) => match l {
                _ => Some(l),
            },
        }
    }

    fn right_mut(&mut self) -> Option<&mut Self> {
        match self {
            &mut RBNode::Empty => None,
            &mut RBNode::Node(_, _, _, box ref mut r) => match r {
                // &mut RBNode::Empty => None,
                _ => Some(r),
            },
        }
    }

    fn is_red(&self) -> bool {
        match self {
            &RBNode::Empty => false,
            &RBNode::Node(_, ref c, _, _) => *c == Color::Red,
        }
    }

    fn is_left_red(&self) -> bool {
        match self {
            &RBNode::Empty => false,
            &RBNode::Node(_, _, box ref l, _) => l.color() == Color::Red,
        }
    }

    fn is_right_red(&self) -> bool {
        match self {
            &RBNode::Empty => false,
            &RBNode::Node(_, _, _, box ref r) => r.color() == Color::Red,
        }
    }

    fn color(&self) -> Color {
        match self {
            &RBNode::Empty => Color::Black,
            &RBNode::Node(_, ref c, _, _) => c.clone(),
        }
    }

    /// Set color of node (expecially means the incoming link of the node)
    ///
    /// //      |   <----- color means the color of this incoming link
    /// //      a
    /// //     / \
    /// //    b   c
    fn set_color(&mut self, c: Color) {
        match self {
            &mut RBNode::Node(_, ref mut color, _, _) => {
                *color = c;
            }
            _ => {}
        }
    }

    fn flip_color(&mut self) {
        if self.is_empty() {
            return;
        }

        let c = self.color();
        self.set_color(if c == Color::Red {
            Color::Black
        } else {
            Color::Red
        });
    }

    fn flip_colors(&mut self) {
        // flip both outcoming link color and the incoming link color
        // flip root
        self.flip_color();
        // flip left
        if self.left_mut().is_some() {
            self.left_mut().unwrap().flip_color();
        }
        // flip right
        if self.right_mut().is_some() {
            self.right_mut().unwrap().flip_color();
        }
    }

    fn insert(&mut self, value: T) -> &mut Self {
        // 新插入节点只有两种情况
        // 1. 在 2-节点 插入
        // 2. 在 3-节点 插入
        match *self {
            RBNode::Empty => {
                *self = RBNode::Node(
                    value,
                    Color::Red,
                    Box::new(RBNode::Empty),
                    Box::new(RBNode::Empty),
                )
            }
            RBNode::Node(ref old_value, _, box ref mut l, box ref mut r) => {
                if value < *old_value {
                    l.insert(value);
                } else {
                    r.insert(value);
                }
            }
        }

        self.balance();

        self
    }

    fn balance(&mut self) {
        let mut ba = BalanceAction::Unknown;

        match self {
            &mut RBNode::Empty => return,
            &mut RBNode::Node(_, _, box ref l, box ref r) => {
                // 【左黑 & 右红】（依据“只能左红——红黑树”定义，需要将右红翻转到左边）
                // 新（插入）节点在【3-节点】右边
                //
                //               a
                //   RED --->   / \   <-- BLACK
                //             b
                //   RED --->   \
                //               c
                //
                if r.is_red() && !l.is_red() {
                    ba = BalanceAction::RotateLeft;
                    // println!("<balance-{}>: rotateLeft", *v);
                }

                // 【左红 & 左左红】（需要将左红翻转到右边，形成左右均红的下面那种情况，进行处理）
                // 新（插入）节点在【3-节点】左边
                //
                //             a
                // RED --->   / \  <--- BLACK
                //           b
                // RED ---> /
                //         c
                //
                if l.is_red() && l.is_left_red() {
                    ba = BalanceAction::RotateRight;
                    // println!("<balance>: rotateRight");
                }

                // 【左右均红】（直接一个翻转分解4-节点即可）
                // 新插入节点在【2-节点】右边
                //
                //              a
                //   RED --->  / \  <--- RED
                //            b   c
                //
                if l.is_red() && r.is_red() {
                    ba = BalanceAction::FlipColors;
                    // println!("<balance>: flipColors");
                }
            }
        }

        match ba {
            BalanceAction::RotateLeft => self.rotate_left(),
            BalanceAction::RotateRight => self.rotate_right(),
            BalanceAction::FlipColors => self.flip_colors(),
            _ => {}
        }
    }

    // pub fn remove(&mut self, value: T) -> &mut Self {

    //     self
    // }

    fn remove_min(&mut self) -> &mut Self {
        // 删除模型中，均以根节点为红色作为假设前提！

        // 如果当前节点为空（空树：根节点即为空），什么都不要做
        if self.is_empty() {
            return self;
        }

        // 无论是【2-节点】还是【3-节点】只要他们的左节点为空，则将当前节点置空即可
        // （因为自己肯定是红色的，删除模型红色假定）
        if self.is_left_empty() {
            // 事实上的叶子节点（本身存储数据，并且有两片Empty叶子）
            *self = RBNode::Empty;
            return self;
        }

        // 【向下】进行节点的摊平操作，使其满足删除条件
        // left.left 都是黑色的目的是为了保证左节点是2-节点的情况，
        // 如果 left.left 是红色的话，左节点就是一个3-节点，满足删除条件不用做转换
        // Empty 叶子节点的颜色也是黑色的，满足下列判断条件，走转换流程
        if !self.left().unwrap().is_red() && !self.left().unwrap().left().unwrap().is_red() {
            self.move_red_left();
        }

        // 【向左】继续执行
        self.left_mut().unwrap().remove_min();

        // 【向上】回溯将4-节点进行拆解恢复
        self.balance();

        self
    }

    fn remove_max(&mut self) -> &mut Self {
        // 与 'remove_min' 类似，只是方向向右
        if self.is_empty() {
            // println!("removing empty");
            return self;
        }

        // let v = self.get_value().unwrap();
        // println!("cur-node({}):<{:?}>", v, self.color());

        if self.left().unwrap().is_red() {
            self.rotate_right();
            // println!(
            //     "after rotate_right: {:?}",
            //     self.pre_order_with_color().unwrap()
            // );
        }

        if self.is_right_empty() {
            *self = RBNode::Empty;
            return self;
        }

        // 保证右节点为 2-节点
        if !self.right().unwrap().is_red() && !self.right().unwrap().left().unwrap().is_red() {
            self.move_red_right();
            // println!(
            //     "after move_red_right: {:?}",
            //     self.pre_order_with_color().unwrap()
            // );
        }

        self.right_mut().unwrap().remove_max();

        // println!(
        //     "balancing node({}) => {:?}",
        //     self.get_value().unwrap(),
        //     self.pre_order_with_color().unwrap()
        // );
        self.balance();
        // println!(
        //     "balanced node({}) => {:?}",
        //     self.get_value().unwrap(),
        //     self.pre_order_with_color().unwrap()
        // );

        self
    }

    fn remove(&mut self, value: &T) -> &mut Self {
        // println!("[[[delete]]]:{:?}", self.to_vec_with_color().unwrap());

        if self.is_empty() {
            return self;
        }

        if *value < self.get_value().unwrap() {
            // 左边
            if !self.left().unwrap().is_red()
                && !(self.left().unwrap().left().is_some()
                    && self.left().unwrap().left().unwrap().is_red())
            {
                // RBTreeHelper::hit_branch(Branch::One);
                self.move_red_left();
            }
            self.left_mut().unwrap().remove(value);
        } else {
            // 右边
            if self.left().unwrap().is_red() {
                // RBTreeHelper::hit_branch(Branch::Two);
                self.rotate_right();
            }
            // 翻转后，在判断右边是否已经找到尽头
            //          |   RED
            //          a
            if *value == self.get_value().unwrap() && self.is_right_empty() {
                // RBTreeHelper::hit_branch(Branch::Three);
                *self = RBNode::Empty;
                return self;
            }

            // 右边还有东西的情况下，查看右边是否为3节点不是的话 move_red_right
            if !self.right().unwrap().is_red()
                && !(self.right().unwrap().left().is_some()
                    && self.right().unwrap().left().unwrap().is_red())
            {
                // RBTreeHelper::hit_branch(Branch::Three);
                self.move_red_right();
            }

            // 不是叶子节点的情况下
            if *value == self.get_value().unwrap() {
                // RBTreeHelper::hit_branch(Branch::Four);
                // 使用右节点最小值更新当前节点
                {
                    let min_v: T;
                    {
                        let min = self.right().unwrap().min().unwrap();
                        min_v = min.get_value().unwrap();
                    }
                    self.set_value(min_v);
                }

                // 删除右节点最小值
                self.right_mut().unwrap().remove_min();
            } else {
                // RBTreeHelper::hit_branch(Branch::Five);
                // 继续向右查找
                self.right_mut().unwrap().remove(value);
            }
        }

        // println!(">>>balance:{:?}", self.to_vec_with_color().unwrap());
        self.balance();
        // println!("<<<balance:{:?}", self.to_vec_with_color().unwrap());

        self
    }

    // ---------------------------------------------------------------------------------
    //                              helper methods
    // ---------------------------------------------------------------------------------
    /// Right rotate tree (view from new root, rotate happened on the right hand)
    /// //      k2                                    k1
    /// //     /  \          left rotate             /  \
    /// //    k1   z        ------------->          x    k2
    /// //   /  \                                       /  \
    /// //  x    y                                     y    z
    ///
    fn rotate_right(&mut self) {
        let mut x: RBNode<T> = RBNode::Empty;
        let mut y: RBNode<T> = RBNode::Empty;
        let mut z: RBNode<T> = RBNode::Empty;
        let k1_v: T;
        let k2_v: T;
        let k1_c: Color;
        let k2_c: Color;

        match self {
            &mut RBNode::Empty => return,
            &mut RBNode::Node(ref mut v, ref c, box ref mut l, box ref mut r) => {
                k1_v = l.get_value().unwrap();
                k1_c = l.color();

                k2_v = v.clone();
                k2_c = c.clone();

                swap(l.left_mut().unwrap(), &mut x);
                swap(l.right_mut().unwrap(), &mut y);
                swap(r, &mut z);
            }
        }

        // generate lower k2 node
        let k2 = RBNode::Node(k2_v, k1_c, Box::new(y), Box::new(z));
        // k2.update_height();

        // generate k1 node
        let k1 = RBNode::Node(k1_v, k2_c, Box::new(x), Box::new(k2));
        // k1.update_height();

        *self = k1;
    }

    /// rotate left tree (RR)
    /// // Rust 由于声明周期的关系，不适合做类似指针赋值交换这样测操作
    /// // 这里采用swap保存子树，然后对局部根节点做重组的方式进行
    /// //    k1                                      k2
    /// //   /  \          right rotate              /  \
    /// //  x   k2        ------------->            k1   z
    /// //      / \                                /  \
    /// //     y   z                              x    y
    fn rotate_left(&mut self) {
        let mut x: RBNode<T> = RBNode::Empty;
        let mut y: RBNode<T> = RBNode::Empty;
        let mut z: RBNode<T> = RBNode::Empty;
        let k1_v: T;
        let k2_v: T;
        let k1_c: Color;
        let k2_c: Color;

        match self {
            &mut RBNode::Empty => return,
            &mut RBNode::Node(ref mut v, ref c, box ref mut l, box ref mut r) => {
                k2_v = r.get_value().unwrap();
                k2_c = r.color();

                k1_v = v.clone();
                k1_c = c.clone();

                swap(r.left_mut().unwrap(), &mut y);
                swap(r.right_mut().unwrap(), &mut z);
                swap(l, &mut x);
            }
        }

        // generate lower k2 node
        let k1 = RBNode::Node(k1_v, k2_c, Box::new(x), Box::new(y));
        // k1.update_height();

        // generate lower k1 node
        let k2 = RBNode::Node(k2_v, k1_c, Box::new(k1), Box::new(z));
        // k2.update_height();

        *self = k2;
    }

    fn move_red_left(&mut self) {
        // 这里的所有模型都是以根节点为【红色】节点为假设，原因是因为删除操作开始时，
        // 将根节点颜色修改为红色，而每次 'move_red_left' 的变换总能将左子节点变为红色，
        // 从而保证了红色根节点子树能模型能够向下传递

        if self.is_empty() || self.is_left_empty() || self.is_right_empty() {
            return;
        }
        // 这段由【插入】流程保证
        // if !self.is_red() || self.left().unwrap().is_red() || self.right().unwrap().is_red() {
        //     return;
        // }

        // 这是在操作删除最小值的情况下，查找方向是向左下方进行，目的是在路径上的途径节点都进行摊平
        // 有两种情况：
        // 1）右节点为2-节点：直接通过【反色】操作就可以变成4-节点
        //
        //                  |   <-- RED
        //                  a
        //     BLACK -->   / \  <-- BLACK
        //                b   c
        //
        // 2）右节点是3-节点（右孩子的左孩子肯定是红色，不然不会形成3-节点）：
        //    a. 先【反色】
        //    b. 右旋右儿子（将 【right.left】 变成 right）（目的是为了让【3-节点的红边】向上移动一层，为后续的继续移动这条【3-节点红边】做铺垫
        //    c. 左旋根节点，让原先3-节点的红边继续往左移动，变成左节点，这样根节点两边同时出现红边，为【反色】恢复原状做铺垫
        //    d. 【反色】新的根节点的，这时候虽然和原来相比牺牲了一条红边（但是由于步骤a中的【反色】实际上多出了一条红边，
        //       抵消下结果就变成了：对比转换前后的两幅图，看到的现象类似将红边从右边移动到了左边），类似左节点从兄弟节点借了一个孩子变成了3-节点
        //
        //                      |   <-- RED
        //                      a
        //        BLACK -->    / \  <-- BLACK（红黑树形式：不能有连续的红色连接）
        //                    b   c
        //                       /  <-- RED（根据红黑树性质：红色节点必在左边）
        //                      d
        self.flip_colors();
        if self.right().unwrap().left().unwrap().is_red() {
            self.right_mut().unwrap().rotate_right();
            self.rotate_left();
            self.flip_colors();
        }
    }

    fn move_red_right(&mut self) {
        if self.is_empty() || self.is_left_empty() || self.is_right_empty() {
            return;
        }
        // 有红色节点移到右边，没有的话上下做个颜色翻转（变成4-节点满足删除条件）

        // 这个步骤处理两种情况：
        // 1) 两边都是2-节点（只需一次翻转就行）
        // 2) 左边3-节点，右边2-节点（做完翻转还得进入下一个if流程，做右转和再次翻转）
        // 具体参考 move_red_left 的注释说明
        self.flip_colors();

        // 左节点是3-节点的话需要右转，让右节点变成3-节点（才能删除）
        // 否则的话通过上一步的翻转就可以分解当前的4-节点）
        //      | BLACK
        //      a
        // RED / \ RED
        //    b   c
        if self.left().unwrap().left().unwrap().is_red() {
            self.rotate_right();
            self.flip_colors();
        }
    }

    fn get_value(&self) -> Option<T> {
        match self {
            &RBNode::Empty => None,
            &RBNode::Node(ref v, _, _, _) => Some(v.clone()),
        }
    }

    fn value(&self) -> Option<&T> {
        match self {
            &RBNode::Empty => None,
            &RBNode::Node(ref v, _, _, _) => Some(v),
        }
    }

    fn set_value(&mut self, value: T) {
        match self {
            &mut RBNode::Empty => {}
            &mut RBNode::Node(ref mut v, _, _, _) => {
                *v = value;
            }
        }
    }

    fn height(&self) -> usize {
        match self {
            &RBNode::Empty => 0,
            &RBNode::Node(_, _, box ref l, box ref r) => max(l.height(), r.height()) + 1,
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            RBNode::Empty => true,
            _ => false,
        }
    }

    fn is_left_empty(&self) -> bool {
        match self {
            &RBNode::Empty => true,
            &RBNode::Node(_, _, box ref l, _) => l.is_empty(),
        }
    }

    fn is_right_empty(&self) -> bool {
        match self {
            &RBNode::Empty => true,
            &RBNode::Node(_, _, _, box ref r) => r.is_empty(),
        }
    }

    fn min(&self) -> Option<&Self> {
        match self {
            &RBNode::Empty => return None,
            &RBNode::Node(_, _, box ref l, _) => {
                if !l.is_empty() {
                    return l.min();
                }
            }
        }
        Some(self)
    }

    fn max(&self) -> Option<&Self> {
        match self {
            &RBNode::Empty => return None,
            &RBNode::Node(_, _, _, box ref r) => {
                if !r.is_empty() {
                    return r.max();
                }
            }
        }
        Some(self)
    }

    fn contains(&self, value: &T) -> bool {
        match self {
            &RBNode::Empty => false,
            &RBNode::Node(ref v, _, box ref l, box ref r) => {
                if *value == *v {
                    true
                } else if *value > *v {
                    r.contains(&value)
                } else {
                    l.contains(&value)
                }
            }
        }
    }

    fn pre_order(&self) -> Option<Vec<&T>> {
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

    fn in_order(&self) -> Option<Vec<&T>> {
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

    fn post_order(&self) -> Option<Vec<&T>> {
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

    fn pre_order_with_color(&self) -> Option<Vec<RBNodeInfo<T>>> {
        // match self {
        //     &RBNode::Empty => None,
        //     &RBNode::Node(ref v, ref c, box ref l, box ref r) => {
        //         let mut vs = Vec::new();
        //         vs.push(RBNodeInfo {
        //             value: v.clone(),
        //             color: c.clone(),
        //         });
        //         if !l.is_empty() {
        //             let lvs = l.to_vec_with_color();
        //             match lvs {
        //                 Some(v) => vs.extend(v),
        //                 _ => {}
        //             }
        //         }
        //         if !r.is_empty() {
        //             let rvs = r.to_vec_with_color();
        //             match rvs {
        //                 Some(v) => vs.extend(v),
        //                 _ => {}
        //             }
        //         }
        //         Some(vs)
        //     }
        // }

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
            nodes.push(RBNodeInfo {
                value: n.value().unwrap().clone(),
                color: n.color(),
            });

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
}

// ---------------------------------------------------------------------------------
//                                  Node Info
// ---------------------------------------------------------------------------------
// pub struct RBNodeInfo<T: Clone + Debug + Display> {
pub struct RBNodeInfo<T: Debug> {
    pub value: T,
    pub color: Color,
}

impl<T: Debug> Debug for RBNodeInfo<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({:?} <{:?}>)", self.value, self.color)
    }
}

impl<T: PartialEq + Debug> PartialEq for RBNodeInfo<T> {
    fn eq(&self, other: &RBNodeInfo<T>) -> bool {
        self.value == other.value && self.color == other.color
    }
}

pub struct RBTree<T: PartialOrd + Clone + Debug + Display> {
    root: RBNode<T>,
}

impl<T: PartialOrd + Clone + Debug + Display> RBTree<T> {
    pub fn new() -> Self {
        RBTree {
            root: RBNode::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> &mut Self {
        if self.root.contains(&value) {
            return self;
        }

        self.root.insert(value);
        self.root.set_color(Color::Black);
        self
    }

    pub fn remove_min(&mut self) -> &mut Self {
        if self.root.is_empty() {
            return self;
        }
        // 删除模型都以根节点为红色做前提假设，如果两边都为黑色，
        // 则因为首次执行时不满足子树以红色节点作为根节点的假设，
        // 需要暂时将根节点置红来满足假设模型，后面再进行恢复
        //
        // 至于两边均为黑色的判断是因为只要两个儿子节点有一边为红色，
        // 就可以通过旋转操作将这条红遍转向左边或者右边从而达到红边向下传递的作用
        if !self.root.is_left_red() && !self.root.is_right_red() {
            self.root.set_color(Color::Red);
        }

        self.root.remove_min();
        // 恢复根节点颜色
        self.root.set_color(Color::Black);
        self
    }

    pub fn remove_max(&mut self) -> &mut Self {
        if !self.root.is_left_red() && !self.root.is_right_red() {
            self.root.set_color(Color::Red);
        }

        self.root.remove_max();
        self.root.set_color(Color::Black);

        self
    }

    pub fn remove(&mut self, value: &T) -> &mut Self {
        if !self.root.contains(value) {
            return self;
        }

        if !self.root.is_left_red() && !self.root.is_right_red() {
            self.root.set_color(Color::Red);
        }

        self.root.remove(value);
        self.root.set_color(Color::Black);

        self
    }

    pub fn pre_order_with_color(&self) -> Option<Vec<RBNodeInfo<T>>> {
        self.root.pre_order_with_color()
    }

    pub fn height(&self) -> usize {
        self.root.height()
    }

    pub fn max(&self) -> Option<T> {
        if let Some(max) = self.root.max() {
            max.get_value()
        } else {
            None
        }
    }

    pub fn min(&self) -> Option<T> {
        if let Some(min) = self.root.min() {
            min.get_value()
        } else {
            None
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root.contains(value)
    }

    pub fn pre_order(&self) -> Option<Vec<&T>> {
        self.root.pre_order()
    }

    pub fn in_order(&self) -> Option<Vec<&T>> {
        self.root.in_order()
    }

    pub fn post_order(&self) -> Option<Vec<&T>> {
        self.root.post_order()
    }
}

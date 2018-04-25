extern crate algorithm;
use algorithm::tree::red_black_tree::{RBTree, RBNodeInfo, Color};

#[test]
fn test_insert() {
    let mut tree = RBTree::new();
    tree.insert('S')
        .insert('E')
        .insert('A')    // <== rotate right
        .insert('R')
        .insert('C')    // <== rotate left
        .insert('H')    // <== flip colors
        .insert('X')
        .insert('M')
        .insert('P')
        .insert('L');
    let act = tree.pre_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['M', 'E', 'C', 'A', 'L', 'H', 'R', 'P', 'X', 'S'];
    assert_eq!(act, exp);
}

#[test]
fn test_remove_min() {
    // covers 'move_red_left'
    let mut tree = RBTree::new();
    tree.insert('S')
        .insert('E')
        .insert('R')
        .insert('C')
        .insert('H')
        .insert('X')
        .insert('M')
        .insert('P')
        .insert('L');
    tree.remove_min();
    let act = tree.pre_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['M', 'H', 'E', 'L', 'R', 'P', 'X', 'S'];
    assert_eq!(act, exp);
}

#[test]
pub fn test_remove_max() {
    // covers 'move_red_left'
    let mut tree = RBTree::new();
    tree.insert('S')
        .insert('I')
        .insert('H')
        .insert('J')
        .insert('Q')
        .insert('M');
    println!("rbtree: {:?}", tree.pre_order_with_color().unwrap());

    tree.remove_max();
    let act = tree.pre_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['M', 'I', 'H', 'J', 'Q'];
    assert_eq!(act, exp);
}

#[test]
pub fn test_remove() {
    // full coverage
    let mut tree = RBTree::new();
    tree.insert('L')
        .insert('A')
        .insert('Q')
        .insert('N')
        .insert('P')
        .insert('T')
        .insert('U')
        .insert('J')
        .insert('C')
        .insert('R');
    // println!("rbtree: {:?}", tree.to_vec_with_color().unwrap());

    tree.remove(&'L');
    let act = tree.pre_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['P', 'C', 'A', 'N', 'J', 'T', 'R', 'Q', 'U'];
    assert_eq!(act, exp);
}

#[test]
pub fn test_height() {
    let mut tree = RBTree::new();

    let mut height = tree.height();
    assert_eq!(height, 0);

    tree.insert('L')
        .insert('A')
        .insert('Q')
        .insert('N')
        .insert('P')
        .insert('T')
        .insert('U')
        .insert('J')
        .insert('C')
        .insert('R');
    height = tree.height();
    assert_eq!(height, 4);
}

#[test]
fn test_min() {
    let mut tree = RBTree::new();
    let max = tree.max();
    assert_eq!(max.is_some(), false);

    tree.insert('Q')
        .insert('X')
        .insert('A')
        .insert('B')
        .insert('C')
        .insert('L')
        .insert('H')
        .insert('J')
        .insert('N')
        .insert('R');
    let max = tree.max().unwrap();
    assert_eq!(max, 'X');
}

#[test]
fn test_max() {
    let mut tree = RBTree::new();
    let max = tree.min();
    assert_eq!(max.is_some(), false);

    tree.insert('Q')
        .insert('X')
        .insert('A')
        .insert('B')
        .insert('C')
        .insert('L')
        .insert('H')
        .insert('J')
        .insert('N')
        .insert('R');
    let max = tree.min().unwrap();
    assert_eq!(max, 'A');
}

#[test]
fn test_contains() {
    let mut tree = RBTree::new();
    tree.insert('Q')
        .insert('X')
        .insert('A')
        .insert('B')
        .insert('C')
        .insert('L')
        .insert('H')
        .insert('J')
        .insert('N')
        .insert('R');
    assert_eq!(tree.contains(&'A'), true);
    assert_eq!(tree.contains(&'Z'), false);
}

#[test]
fn test_pre_order() {
    let mut tree = RBTree::new();
    tree.insert('C')
        .insert('B')
        .insert('A')
        .insert('D')
        .insert('E');
    let act = tree.pre_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['D', 'B', 'A', 'C', 'E'];
    assert_eq!(act, exp);
}

#[test]
fn test_in_order() {
    let mut tree = RBTree::new();
    tree.insert('C')
        .insert('B')
        .insert('A')
        .insert('D')
        .insert('E');
    let act = tree.in_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['A', 'B', 'C', 'D', 'E'];
    assert_eq!(act, exp);
}

#[test]
fn test_post_order() {
    let mut tree = RBTree::new();
    tree.insert('C')
        .insert('B')
        .insert('A')
        .insert('D')
        .insert('E');
    let act = tree.post_order().unwrap();
    let act: Vec<char> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec!['A', 'C', 'B', 'E', 'D'];
    assert_eq!(act, exp);
}

#[test]
fn test_pre_order_with_color() {
    let mut tree = RBTree::new();
    tree.insert('C')
        .insert('B')
        .insert('A')
        .insert('D')
        .insert('E');
    let act = tree.pre_order_with_color().unwrap();
    println!("act:{:?}", act);
    let exp = vec![
        RBNodeInfo {
            value: 'D',
            color: Color::Black,
        },
        RBNodeInfo {
            value: 'B',
            color: Color::Red,
        },
        RBNodeInfo {
            value: 'A',
            color: Color::Black,
        },
        RBNodeInfo {
            value: 'C',
            color: Color::Black,
        },
        RBNodeInfo {
            value: 'E',
            color: Color::Black,
        },
    ];
    assert_eq!(act, exp);
}

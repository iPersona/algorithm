extern crate algorithm;
use algorithm::tree::avl_tree::AVLTree;

#[test]
fn test_find_min() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let min = tree.find_min().unwrap();
    assert_eq!(min, 1);
}

#[test]
fn test_find_max() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let min = tree.find_max().unwrap();
    assert_eq!(min, 16);
}

// #[test]
pub fn test_depth() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let depth = tree.depth();
    // println!("depth: {:?}", depth);
    assert_eq!(depth, 5);
}

#[test]
fn test_contains() {
    let mut tree = AVLTree::Empty;
    tree.insert(5).insert(4).insert(3).insert(6).insert(2);
    let mut is_found = tree.contains(6);
    assert_eq!(is_found, true);
    is_found = tree.contains(1);
    assert_eq!(is_found, false);
    is_found = tree.contains(7);
    assert_eq!(is_found, false);
}

#[test]
fn test_remove_no_unbalanced() {
    // remove node that does not cause unbalanced
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    tree.remove(8);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 4, 2, 1, 3, 6, 5, 13, 11, 9, 10, 12, 15, 14, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_remove_ll() {
    // remove node that cause LL rotation
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    tree.remove(10).remove(12);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 4, 2, 1, 3, 6, 5, 13, 9, 8, 11, 15, 14, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_remove_rr() {
    // remove node that cause RR rotation
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9)
        .insert(17);
    tree.remove(14);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 4, 2, 1, 3, 6, 5, 13, 11, 9, 8, 10, 12, 16, 15, 17];
    assert_eq!(act, exp);
}

#[test]
fn test_remove_lr() {
    // remove node that cause LR rotation
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    tree.remove(8).remove(12);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 4, 2, 1, 3, 6, 5, 13, 10, 9, 11, 15, 14, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_remove_rl() {
    // remove node that cause RL rotation
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(9);
    tree.remove(1).remove(3).remove(2);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 5, 4, 6, 13, 11, 9, 12, 15, 14, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_to_pre_order() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![7, 4, 2, 1, 3, 6, 5, 13, 11, 9, 8, 10, 12, 15, 14, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_to_in_order() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let act = tree.in_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert_eq!(act, exp);
}

#[test]
fn test_to_post_order() {
    let mut tree = AVLTree::Empty;
    tree.insert(3)
        .insert(2)
        .insert(1)
        .insert(4)
        .insert(5)
        .insert(6)
        .insert(7)
        .insert(16)
        .insert(15)
        .insert(14)
        .insert(13)
        .insert(12)
        .insert(11)
        .insert(10)
        .insert(8)
        .insert(9);
    let act = tree.post_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 3, 2, 5, 6, 4, 8, 10, 9, 12, 11, 14, 16, 15, 13, 7];
    assert_eq!(act, exp);
}

#[test]
fn test_is_empty() {
    let mut tree = AVLTree::Empty;
    let empty = tree.is_empty();
    assert_eq!(empty, true);

    tree.insert(1);
    let empty = tree.is_empty();
    assert_eq!(empty, false);
}

#[test]
fn test_get_value() {
    let mut tree = AVLTree::Empty;
    let value = tree.get_value();
    assert_eq!(value, None);

    tree.insert(1);
    let value = tree.get_value().unwrap();
    assert_eq!(value, 1);
}

#[test]
fn test_get_child_num() {
    let mut tree = AVLTree::Empty;
    let num = tree.get_child_num();
    assert_eq!(num, 0);

    tree.insert(2);
    let num = tree.get_child_num();
    assert_eq!(num, 0);

    tree.insert(4);
    let num = tree.get_child_num();
    assert_eq!(num, 1);

    tree.insert(1);
    let num = tree.get_child_num();
    assert_eq!(num, 2);
}

#[test]
fn test_is_left_empty() {
    let mut tree = AVLTree::Empty;
    let empty = tree.is_left_empty();
    assert_eq!(empty, true);

    tree.insert(2);
    let empty = tree.is_left_empty();
    assert_eq!(empty, true);

    tree.insert(1);
    let empty = tree.is_left_empty();
    assert_eq!(empty, false);
}

#[test]
fn test_is_right_empty() {
    let mut tree = AVLTree::Empty;
    let empty = tree.is_right_empty();
    assert_eq!(empty, true);

    tree.insert(2);
    let empty = tree.is_right_empty();
    assert_eq!(empty, true);

    tree.insert(4);
    let empty = tree.is_right_empty();
    assert_eq!(empty, false);
}

extern crate algorithm;
use algorithm::tree::binary_search_tree::BST;

#[test]
fn test_find_min() {
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    let min = tree.find_min().unwrap();
    assert_eq!(min, 1);
}

#[test]
fn test_find_max() {
    let mut tree = BST::Empty;
    tree.insert(5).insert(4).insert(3).insert(6).insert(2);
    let min = tree.find_max().unwrap();
    assert_eq!(min, 6);
}

#[test]
fn test_depth() {
    let mut tree = BST::Empty;
    tree.insert(5).insert(4).insert(3).insert(6).insert(2);
    let depth = tree.depth();
    assert_eq!(depth, 4);
}

#[test]
fn test_contains() {
    let mut tree = BST::Empty;
    tree.insert(5).insert(4).insert(3).insert(6).insert(2);
    let mut is_found = tree.contains(6);
    assert_eq!(is_found, true);
    is_found = tree.contains(1);
    assert_eq!(is_found, false);
    is_found = tree.contains(7);
    assert_eq!(is_found, false);
}

#[test]
fn test_remove() {
    // remove left leaf
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    tree.remove(2);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 4, 3, 6, 5, 10];
    assert_eq!(act, exp);

    // remove right leaf
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    tree.remove(10);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 4, 3, 2, 6, 5];
    assert_eq!(act, exp);

    // remove branch node with single left child
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    tree.remove(3);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    // println!("act-left: {:?}", act);
    let exp = vec![1, 4, 2, 6, 5, 10];
    assert_eq!(act, exp);

    // remove branch node with single right child
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(11);
    tree.remove(6);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    // println!("act-right: {:?}", act);
    let exp = vec![1, 4, 3, 2, 10, 11];
    assert_eq!(act, exp);

    // remove branch node both left and right childs
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    tree.remove(6);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 4, 3, 2, 10, 5];
    assert_eq!(act, exp);

    // remove root
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    tree.remove(1);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![4, 3, 2, 6, 5, 10];
    assert_eq!(act, exp);
}

#[test]
fn test_pre_order() {
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    let act = tree.pre_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 4, 3, 2, 6, 5, 10];
    assert_eq!(act, exp);
}

#[test]
fn test_in_order() {
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    let act = tree.in_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![1, 2, 3, 4, 5, 6, 10];
    assert_eq!(act, exp);
}

#[test]
fn test_post_order() {
    let mut tree = BST::Empty;
    tree.insert(1)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2)
        .insert(10)
        .insert(5);
    let act = tree.post_order().unwrap();
    let act: Vec<u32> = act.into_iter().map(|v| v.clone()).collect();
    let exp = vec![2, 3, 5, 10, 6, 4, 1];
    assert_eq!(act, exp);
}

#[test]
fn test_is_empty() {
    let mut tree = BST::Empty;
    let empty = tree.is_empty();
    assert_eq!(empty, true);

    tree.insert(1);
    let empty = tree.is_empty();
    assert_eq!(empty, false);
}

#[test]
fn test_get_value() {
    let mut tree = BST::Empty;
    let value = tree.get_value();
    assert_eq!(value, None);

    tree.insert(1);
    let value = tree.get_value().unwrap();
    assert_eq!(value, 1);
}

#[test]
fn test_get_child_num() {
    let mut tree = BST::Empty;
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
    let mut tree = BST::Empty;
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
    let mut tree = BST::Empty;
    let empty = tree.is_right_empty();
    assert_eq!(empty, true);

    tree.insert(2);
    let empty = tree.is_right_empty();
    assert_eq!(empty, true);

    tree.insert(4);
    let empty = tree.is_right_empty();
    assert_eq!(empty, false);
}

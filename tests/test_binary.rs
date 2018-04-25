extern crate algorithm;
use algorithm::search::binary::binary_search;

#[test]
fn test_array_found() {
    let source = vec![1, 2, 3, 4, 5];
    let target = 4;
    let is_found = binary_search(&source, &target);
    assert_eq!(is_found, true);
}

#[test]
fn test_array_not_found() {
    let source = vec![1, 2, 3, 4, 5];
    let target = 6;
    let is_found = binary_search(&source, &target);
    assert_eq!(is_found, false);
}


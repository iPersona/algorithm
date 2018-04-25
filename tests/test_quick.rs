extern crate algorithm;
use algorithm::sort::quick::quick_sort;

#[test]
fn test_quick_sort_array() {
    let mut source = vec![3, 2, 5, 1, 4];
    quick_sort(&mut source);
    assert_eq!(source, vec![1, 2, 3, 4, 5]);
}
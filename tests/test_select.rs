extern crate algorithm;
use algorithm::sort::select::selection_sort;

#[test]
fn test_sort_array() {
    let mut source = vec![1, 4, 5, 2, 3];
    selection_sort(&mut source);
    assert_eq!(source, vec![1, 2, 3, 4, 5]);
}

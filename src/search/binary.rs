use std::cmp::{ PartialOrd, PartialEq };

pub fn binary_search<T: PartialOrd + PartialEq>(source: &Vec<T>, target: &T) -> bool {
    let mut lo:usize = 0;
    let mut hi:usize = (*source).len() - 1;

    while lo <= hi {
        println!("[{}, {}]", lo, hi);
        let mid = lo + (hi - lo) / 2;
        if *target < (*source)[mid] {
            hi = mid - 1;
        } else if *target > (*source)[mid] {
            lo = mid + 1;
        } else {
            return true
        }
    }

    false
}

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




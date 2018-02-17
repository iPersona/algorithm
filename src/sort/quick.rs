use std::cmp::{PartialEq, PartialOrd};
// use std::clone::Clone;

pub fn quick_sort<T: PartialEq + PartialOrd>(source: &mut Vec<T>) {
    if (*source).len() <= 1 {
        return
    }

    let base = (*source).pop().unwrap();

    let mut less = Vec::new();
    let mut greater = Vec::new();
    loop {
        if (*source).len() <= 0 {
            break;
        }

        let v = (*source).pop().unwrap();
        if v < base {
            less.push(v);
        } else {
            greater.push(v);
        }
    }
    
    let mut sort_array = Vec::new();
    quick_sort(&mut less);
    quick_sort(&mut greater);
    sort_array.append(&mut less);
    sort_array.push(base);
    sort_array.append(&mut greater);

    (*source).clear();
    source.append(&mut sort_array);
}

#[test]
fn test_quick_sort_array() {
    let mut source = vec![3, 2, 5, 1, 4];
    quick_sort(&mut source);
    assert_eq!(source, vec![1, 2, 3, 4, 5]);
}
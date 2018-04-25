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

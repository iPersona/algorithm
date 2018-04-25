use std::cmp::{PartialEq, PartialOrd};

pub fn selection_sort<T: PartialEq + PartialOrd>(source: &mut Vec<T>) {
    let mut sorted_array = Vec::new();
    loop {
        if (*source).len() <= 0 {
            break;
        }

        let smallest = find_smallest(source);
        println!("smallest:{}", smallest);
        sorted_array.push((*source).remove(smallest));
    }
    (*source).append(&mut sorted_array);
}

type Index = usize;

fn find_smallest<T: PartialEq + PartialOrd>(source: &Vec<T>) -> Index {
    let mut smallest = &(*source)[0];
    let mut index:Index = 0;

    for (i, v) in (*source).iter().enumerate() {
        if *v < *smallest {
            smallest = v;
            index = i as Index;
        }
    }

    index
}

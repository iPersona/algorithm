use ndarray::Array2;
use quickersort;

/// Indicate a item.
pub struct Item {
    name: String,
    size: u32,
    value: u32,
}

impl Item {
    pub fn new(name: &str, size: u32, value: u32) -> Item {
        Item {
            name: String::from(name),
            size: size,
            value: value,
        }
    }
}

/// Indicate a unit of bag table.
#[derive(Clone, Default)]
pub struct Bag {
    value: u32,
    items: Vec<String>,
}

impl Bag {
    fn new(value: u32, items: Vec<String>) -> Bag {
        Bag {
            value: value,
            items: items,
        }
    }
}

pub fn bag_problem(items: &Vec<Item>, bag_size: u32) -> Vec<String> {
    let (mut bag_table, bag_size_list, _) = calc_bag_problem_table(items);

    let mut best_value_items = Vec::new();
    let mut best_value = 0_u32;
    let mut item_row = 0_usize;
    let item_col = bag_size_list
        .iter()
        .enumerate()
        .filter(|&(_, c)| *c == bag_size)
        .nth(0)
        .map(|(i, _)| i)
        .unwrap();

    for i in 0..bag_table.rows() {
        if bag_table[[i, item_col]].value > best_value {
            best_value = bag_table[[i, item_col]].value;
            item_row = i;
        }
    }

    best_value_items.append(&mut bag_table[[item_row, item_col]].items);
    best_value_items
}

/// Generate bag problem table, returning a truple of bag table, size list and items name list.
///
/// # Example
/// ```
/// use algorithm::dynamic::bag::*;
///
/// let mut items: Vec<Item> = Vec::new();
/// items.push(Item::new("Guitar", 1, 1500));
/// items.push(Item::new("Audio Electronics", 4, 3000));
/// items.push(Item::new("Notebook", 3, 2000));
///
/// let (_, _, _) = calc_bag_problem_table(&items);
/// ```
pub fn calc_bag_problem_table<'a>(items: &'a Vec<Item>) -> (Array2<Bag>, Vec<u32>, Vec<&'a str>) {
    // let (min, max) = find_size_range(&items);
    // let bag_size_list = gen_bag_size_list(min, max, size_step);
    let bag_size_list = gen_bag_size_list(&items);
    let mut bag_table = Array2::<Bag>::default((items.len(), bag_size_list.len()));
    for i in 0..bag_table.rows() {
        for j in 0..bag_table.cols() {
            update_largest_value(&mut bag_table, &items, &bag_size_list, i, j);
        }
    }

    let mut items_name_list = Vec::new();
    for item in items {
        items_name_list.extend_from_slice(&vec![item.name.as_str()]);
    }
    (bag_table, bag_size_list, items_name_list)
}

// fn test_bag() {
//     let mut items: Vec<Item> = Vec::new();
//     items.push(Item::new("Guitar", 1, 1500));
//     items.push(Item::new("Audio Electronics", 4, 3000));
//     items.push(Item::new("Notebook", 3, 2000));
//     items.push(Item::new("iPhone", 1, 2000));

//     let (bag_table, bag_size_list, items_name_list) = calc_bag_problem_table(&items);
//     print_bag_table(&bag_table, &bag_size_list, &items_name_list);
// }

// fn print_bag_table(
//     bag_table: &Array2<Bag>,
//     bag_size_list: &Vec<u32>,
//     items_name_list: &Vec<&str>,
// ) {
//     let col_width = 30;

//     let mut title = format!("{: <1$}|\t", "", col_width);
//     for size in bag_size_list {
//         let info = format!("{: <1$}|\t", size, col_width);
//         title.push_str(&info);
//     }
//     println!("{}", title);
//     for i in 0..bag_table.rows() {
//         let mut row = "".to_string();
//         for j in 0..bag_table.cols() {
//             let info = format!(
//                 "{}$, {:?}",
//                 bag_table[[i, j]].value,
//                 bag_table[[i, j]].items
//             );
//             row.push_str(&format!("{: <1$}|\t", info, col_width));
//         }
//         println!("{: <2$}|\t{}", items_name_list[i], row, col_width);
//     }
// }

fn update_largest_value(
    table: &mut Array2<Bag>,
    items: &Vec<Item>,
    bag_size_list: &Vec<u32>,
    row: usize,
    col: usize,
) {
    let item = &items[row];
    let cur_size = bag_size_list[col];
    if row == 0 {
        if item.size <= cur_size {
            table[[row, col]] = Bag::new(item.value, vec![item.name.clone()]);
        }
        return;
    }

    let pre_unit_value = table[[row - 1, col]].value;
    let cur_item_value = item.value;
    if cur_size < item.size {
        let mut items = Vec::new();
        items.extend_from_slice(&table[[row - 1, col]].items[..]);
        table[[row, col]] = Bag::new(table[[row - 1, col]].value, items);
        return;
    }

    let left_size = cur_size - item.size;
    if left_size == 0 {
        if cur_item_value > pre_unit_value {
            table[[row, col]] = Bag::new(cur_item_value, vec![item.name.clone()]);
        } else {
            let mut items = Vec::new();
            items.extend_from_slice(&table[[row - 1, col]].items[..]);
            table[[row, col]] = Bag::new(pre_unit_value, items);
        }
        return;
    }

    let left_size_col = get_bag_size_index(&bag_size_list, &left_size).unwrap();
    let left_size_value = table[[row - 1, left_size_col]].value;
    table[[row, col]] = if pre_unit_value > (cur_item_value + left_size_value) {
        let mut items = Vec::new();
        items.extend_from_slice(&table[[row - 1, col]].items[..]);
        Bag::new(pre_unit_value, items)
    } else {
        let mut items = Vec::new();
        items.extend_from_slice(&table[[row - 1, left_size_col]].items[..]);
        items.push(item.name.clone());
        Bag::new(cur_item_value + left_size_value, items)
    };
}

fn get_bag_size_index(bag_size_list: &Vec<u32>, value: &u32) -> Option<usize> {
    for (i, v) in bag_size_list.iter().enumerate() {
        if *v == *value {
            return Some(i);
        }
    }
    None
}

/// Return all available combination of bag sizes.
///
/// Here we use the Full Combination Algorithm to generate the bag size list.
/// 
pub fn gen_bag_size_list(items: &Vec<Item>) -> Vec<u32> {
    let len = items.len();
    let total: usize = 1 << len;
    let mut all_size_list = Vec::new();

    // get max size.
    let mut max = 0_u32;
    for item in items {
        if item.size > max {
            max = item.size;
        }
    }

    for i in 1..total {
        let mut size_list = Vec::new();
        for j in 0..len {
            if i & (1 << j) != 0 {
                size_list.push(items[j].size.clone());
            }
        }
        let mut total = size_list.into_iter().sum();
        if !all_size_list.contains(&total) && total <= max {
            all_size_list.push(total);
        }
    }

    quickersort::sort(&mut all_size_list);
    all_size_list
}

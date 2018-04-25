extern crate algorithm;
use algorithm::dynamic::bag::*;

#[test]
fn test_gen_bag_size_list_no_repeat() {
    let mut items: Vec<Item> = Vec::new();
    items.push(Item::new("Guitar", 1, 1500));
    items.push(Item::new("Audio Electronics", 4, 3000));
    items.push(Item::new("Notebook", 3, 2000));

    let size_list = gen_bag_size_list(&items);
    assert_eq!(size_list, vec![1, 3, 4]);
}

#[test]
fn test_gen_bag_size_list_partly_repart() {
    let mut items: Vec<Item> = Vec::new();
    items.push(Item::new("Guitar", 1, 1500));
    items.push(Item::new("Audio Electronics", 1, 3000));
    items.push(Item::new("Notebook", 3, 2000));

    let size_list = gen_bag_size_list(&items);
    // println!("size_list: {:?}", size_list);
    assert_eq!(size_list, vec![1, 2, 3]);
}

#[test]
fn test_bag_problem() {
    let mut items: Vec<Item> = Vec::new();
    items.push(Item::new("Guitar", 1, 1500));
    items.push(Item::new("Audio Electronics", 4, 3000));
    items.push(Item::new("Notebook", 3, 2000));

    let bag_size = 4;
    let best_value_items = bag_problem(&items, bag_size);
    assert_eq!(
        best_value_items,
        vec!["Guitar".to_owned(), "Notebook".to_owned()]
    );
}

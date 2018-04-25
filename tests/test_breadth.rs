extern crate algorithm;

use std::collections::HashMap;

use algorithm::search::breadth::*;

#[test]
fn breadth_search_generic_array_found() {
    let mut graph = HashMap::new();
    graph.insert("you", vec!["alice", "bob", "claire"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("anuj", vec![]);
    graph.insert("peggy", vec!["thom"]);
    graph.insert("thom", vec![]);
    graph.insert("jony", vec![]);

    let starter = "you";

    let found = breadth_search_generic(&mut graph, &starter, |name| {
        let chars: Vec<char> = name.chars().collect();
        *chars.last().unwrap() == 'm'
    });
    assert_eq!(found, true);
}

#[test]
fn test_breadth_search_new_array_not_found() {
    let mut graph = HashMap::new();
    graph.insert("you", vec!["alice", "bob", "claire"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("anuj", vec![]);
    graph.insert("peggy", vec![]);
    graph.insert("thom", vec![]);
    graph.insert("jony", vec![]);

    let starter = "you";

    let found = breadth_search(&mut graph, starter, |name| {
        let chars: Vec<char> = name.chars().collect();
        *chars.last().unwrap() == 'm'
    });
    assert_eq!(found, false);
}

#[test]
fn test_breadth_search_array_not_found() {
    let mut graph = HashMap::new();
    graph.insert("you", vec!["alice", "bob", "claire"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("anuj", vec![]);
    graph.insert("peggy", vec![]);
    graph.insert("thom", vec![]);
    graph.insert("jony", vec![]);

    let starter = "you";

    let found = breadth_search(&mut graph, starter, |name| {
        let chars: Vec<char> = name.chars().collect();
        *chars.last().unwrap() == 'm'
    });
    assert_eq!(found, false);
}

#[test]
fn test_breadth_search_array_found() {
    let mut graph = HashMap::new();
    graph.insert("you", vec!["alice", "bob", "claire"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("anuj", vec![]);
    graph.insert("peggy", vec!["thom"]);
    graph.insert("thom", vec![]);
    graph.insert("jony", vec![]);

    let starter = "you";

    let found = breadth_search(&mut graph, starter, |name| {
        let chars: Vec<char> = name.chars().collect();
        *chars.last().unwrap() == 'm'
    });
    assert_eq!(found, true);
}

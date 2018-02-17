use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::clone::Clone;
use std::cmp::Eq;
use std::hash::Hash;

pub fn breadth_search_generic<T: Display + Debug + Clone + Eq + Hash, F>(
    source: &HashMap<T, Vec<T>>,
    starter: &T,
    condition: F,
) -> bool
where
    F: Fn(&T) -> bool,
{
    let mut search_queue = Vec::new();
    search_queue.extend_from_slice(source.get(starter).unwrap());
    let mut searched: Vec<T> = Vec::new();

    loop {
        if search_queue.len() <= 0 as usize {
            return false;
        }

        let source_str = format!("{:?}", search_queue);
        let person = search_queue.pop().unwrap();
        println!("[{:?}] from {:?}", person, source_str);

        if searched.contains(&person) {
            continue;
        }

        if condition(&person) {
            println!("{} is a mongo seller!", person);
            return true;
        }
        if let Some(friends) = source.get(&person) {
            search_queue.extend_from_slice(friends);
        }
        searched.push(person);
    }
}

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



pub fn breadth_search<F>(source: &HashMap<&str, Vec<&str>>, starter: &str, condition: F) -> bool
where
    F: Fn(&str) -> bool,
{
    let mut search_queue = Vec::new();
    search_queue.extend_from_slice(&source[starter]);
    let mut searched = Vec::new();

    loop {
        if search_queue.len() <= 0 as usize {
            return false;
        }

        let source_str = format!("{:?}", search_queue);
        let person = search_queue.pop().unwrap();
        println!("[{}] from {}", person, source_str);

        if searched.contains(&person) {
            continue;
        }

        if condition(person) {
            println!("{} is a mongo seller!", person);
            return true;
        }
        if let Some(friends) = source.get(person) {
            search_queue.extend_from_slice(friends);
        }
        searched.push(person);
    }
}

pub fn breadth_search_new<F>(
    source: &HashMap<String, Vec<String>>,
    starter: &str,
    condition: F,
) -> bool
where
    F: Fn(&str) -> bool,
{
    let mut search_queue = Vec::new();
    search_queue.extend_from_slice(&source[starter]);
    let mut searched: Vec<String> = Vec::new();

    loop {
        if search_queue.len() <= 0 as usize {
            return false;
        }

        let source_str = format!("{:?}", search_queue);
        let person = search_queue.pop().unwrap();
        println!("[{}] from {}", person, source_str);

        if searched.contains(&person) {
            continue;
        }

        // person: String
        // *person: str (via Deref<Target=str>, *deref => *&str => str)
        // &*person: &str
        if condition(&*person) {
            println!("{} is a mongo seller!", person);
            return true;
        }
        if let Some(friends) = source.get(&*person) {
            search_queue.extend_from_slice(friends);
        }
        searched.push(person);
    }
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

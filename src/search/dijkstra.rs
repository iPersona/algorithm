// TODO: 
// * Write a method to generate graph, parents, and costs from file.

use std::collections::HashMap;
use std::f32;
use std::cmp::{Eq, PartialEq};
use std::hash::Hash;
use std::clone::Clone;
use std::fmt::{Debug, Display};

pub fn search<K: PartialEq + Eq + Hash + Clone + Debug + Display>(
    graph: &HashMap<K, HashMap<K, f32>>,
    start: &K,
    end: &K,
    costs: &mut HashMap<K, f32>,
    parents: &mut HashMap<K, Option<K>>,
) -> Vec<K> {
    let mut processed = Vec::new();

    let mut node = find_lowest_cost_node(costs, &processed);
    loop {
        if node == None {
            break;
        }

        let node_key = node.unwrap();
        println!("lowest_cost_node:{:?}", node_key);
        let cost = *costs.get(&node_key).unwrap();
        let neighbors = graph.get(&node_key).unwrap();
        for n in neighbors.keys() {
            let new_cost = cost + neighbors[n];
            if costs[n] > new_cost {
                costs.insert((*n).clone(), new_cost);
                parents.insert((*n).clone(), Some(node_key.clone()));
            }
        }
        processed.extend_from_slice(&vec![node_key.clone()]);
        node = find_lowest_cost_node(costs, &processed);
    }

    // generate the path
    get_path(parents, start, end)
}

fn find_lowest_cost_node<T: Eq + Hash + Clone>(costs: &mut HashMap<T, f32>, processed: &Vec<T>) -> Option<T> {
    let mut lowest_cost = f32::INFINITY;
    let mut lowest_cost_node = None;

    for node in costs.keys() {
        let cost = *costs.get(node).unwrap();
        if cost < lowest_cost && !processed.contains(node) {
            lowest_cost = cost;
            lowest_cost_node = Some((*node).clone());
        }
    }

    lowest_cost_node
}

fn get_path<K: PartialEq + Eq + Hash + Clone>(
    parents: &HashMap<K, Option<K>>,
    start: &K,
    end: &K,
) -> Vec<K> {
    let mut path = Vec::new();
    // let x = parents.get(end).unwrap();
    // let y = (*x).clone().unwrap();
    path.extend_from_slice(&vec![(*end).clone()]);
    path.extend_from_slice(&vec![(*parents.get(end).unwrap()).clone().unwrap()]);
    loop {
        let parent = path.last().unwrap().clone();
        if parent == *start {
            break;
        }

        path.extend_from_slice(&vec![(*parents.get(&parent).unwrap()).clone().unwrap()]);
    }

    path.reverse();
    path
}

pub fn search_array_found() {
    // graph
    // ---------------------------------------
    //              <a>
    //  <start>             <end>
    //              <b>
    // ---------------------------------------

    let mut graph = HashMap::new();

    // node: start
    let mut start_vertices = HashMap::new();
    start_vertices.insert("a", 6_f32);
    start_vertices.insert("b", 2_f32);
    graph.insert("start", start_vertices);

    // node: a
    let mut a_vertices = HashMap::new();
    a_vertices.insert("fin", 1_f32);
    graph.insert("a", a_vertices);

    // node: b
    let mut b_vertices = HashMap::new();
    b_vertices.insert("a", 3_f32);
    b_vertices.insert("fin", 5_f32);
    graph.insert("b", b_vertices);

    // node: fin
    graph.insert("fin", HashMap::new());

    // cost
    let mut costs = HashMap::new();
    costs.insert("a", 6_f32);
    costs.insert("b", 2_f32);
    costs.insert("fin", f32::INFINITY);

    // parents
    let mut parents = HashMap::new();
    parents.insert("a", Some("start"));
    parents.insert("b", Some("start"));
    parents.insert("fin", None);

    let key_start = "start";
    let key_end = "fin";

    let path = search(&graph, &key_start, &key_end, &mut costs, &mut parents);
    assert_eq!(path, vec!["start", "b", "a", "fin"]);
}

#[test]
fn search_array_found_a() {
    // graph
    // ---------------------------------------
    //              <a>     <c>
    //  <start>                     <end>
    //              <b>     <d>
    // ---------------------------------------
    //              <a>     <c>
    let mut graph = HashMap::new();

    // node: start
    let mut start_vertices = HashMap::new();
    start_vertices.insert("a", 5_f32);
    start_vertices.insert("b", 2_f32);
    graph.insert("start", start_vertices);

    // node: a
    let mut a_vertices = HashMap::new();
    a_vertices.insert("c", 4_f32);
    a_vertices.insert("d", 2_f32);
    graph.insert("a", a_vertices);

    // node: b
    let mut b_vertices = HashMap::new();
    b_vertices.insert("a", 8_f32);
    b_vertices.insert("d", 7_f32);
    graph.insert("b", b_vertices);

    // node: c
    let mut c_vertices = HashMap::new();
    c_vertices.insert("d", 6_f32);
    c_vertices.insert("fin", 3_f32);
    graph.insert("c", c_vertices);
    
    // node: d
    let mut d_vertices = HashMap::new();
    d_vertices.insert("fin", 1_f32);
    graph.insert("d", d_vertices);

    // node: fin
    graph.insert("fin", HashMap::new());

    // cost
    let mut costs = HashMap::new();
    costs.insert("a", 5_f32);
    costs.insert("b", 2_f32);
    costs.insert("c", f32::INFINITY);
    costs.insert("d", f32::INFINITY);
    costs.insert("fin", f32::INFINITY);

    // parents
    let mut parents = HashMap::new();
    parents.insert("a", Some("start"));
    parents.insert("b", Some("start"));
    parents.insert("c", Some("a"));
    parents.insert("d", None);
    parents.insert("fin", None);

    let key_start = "start";
    let key_end = "fin";

    let path = search(&graph, &key_start, &key_end, &mut costs, &mut parents);
    println!("{:?}", path);
    assert_eq!(path, vec!["start", "a", "d", "fin"]);
}

#[test]
fn search_array_found_b() {
    // graph
    // ---------------------------------------
    //              <a>     <c>
    //  <start>                     <end>
    //                  <b>
    // ---------------------------------------
    //              <a>     <c>
    let mut graph = HashMap::new();

    // node: start
    let mut start_vertices = HashMap::new();
    start_vertices.insert("a", 10_f32);
    graph.insert("start", start_vertices);

    // node: a
    let mut a_vertices = HashMap::new();
    a_vertices.insert("c", 20_f32);
    graph.insert("a", a_vertices);

    // node: b
    let mut b_vertices = HashMap::new();
    b_vertices.insert("a", 1_f32);
    graph.insert("b", b_vertices);

    // node: c
    let mut c_vertices = HashMap::new();
    c_vertices.insert("b", 1_f32);
    c_vertices.insert("fin", 30_f32);
    graph.insert("c", c_vertices);
    
    // node: fin
    graph.insert("fin", HashMap::new());

    // cost
    let mut costs = HashMap::new();
    costs.insert("a", 10_f32);
    costs.insert("b", f32::INFINITY);
    costs.insert("c", f32::INFINITY);
    costs.insert("fin", f32::INFINITY);

    // parents
    let mut parents = HashMap::new();
    parents.insert("a", Some("start"));
    parents.insert("b", Some("c"));
    parents.insert("c", Some("a"));
    parents.insert("fin", Some("c"));

    let key_start = "start";
    let key_end = "fin";

    let path = search(&graph, &key_start, &key_end, &mut costs, &mut parents);
    println!("{:?}", path);
    assert_eq!(path, vec!["start", "a", "c", "fin"]);
}

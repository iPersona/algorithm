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

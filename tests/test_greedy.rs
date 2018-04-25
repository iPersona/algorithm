extern crate algorithm;

use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

use algorithm::search::greedy::*;

#[test]
fn test_array_success() {
    // stations
    let mut stations = HashMap::new();

    let kone: HashSet<&str> = vec!["id", "nv", "ut"].into_iter().collect();
    stations.insert("kone", kone);

    let ktwo: HashSet<&str> = vec!["wa", "id", "mt"].into_iter().collect();
    stations.insert("ktwo", ktwo);

    let kthree: HashSet<&str> = vec!["or", "nv", "ca"].into_iter().collect();
    stations.insert("kthree", kthree);

    let kfour: HashSet<&str> = vec!["nv", "ut"].into_iter().collect();
    stations.insert("kfour", kfour);

    let kfive: HashSet<&str> = vec!["ca", "az"].into_iter().collect();
    stations.insert("kfive", kfive);

    // states needed
    let states_needed: HashSet<&str> = vec!["mt", "wa", "or", "id", "nv", "ut", "ca", "az"]
        .into_iter()
        .collect();

    let covered = find_best_covered_state(&stations, &states_needed);
    // let expect: HashSet<&str> = vec!["ktwo", "kthree", "kone", "kfive"]
    //     .into_iter()
    //     .collect();
    let stations_of_covered = get_covered_states(&stations, &covered);
    println!("stations_of_covered: {:?}", stations_of_covered);
    assert_eq!(stations_of_covered, states_needed);
}

fn get_covered_states<T: Eq + Hash + Copy + Display + Debug>(
    stations: &HashMap<T, HashSet<T>>,
    target_stations: &HashSet<T>,
) -> HashSet<T> {
    let mut states = HashSet::new();

    for station in target_stations.iter() {
        let states_of_station = stations.get(&station).unwrap();
        states = states
            .union(states_of_station)
            .map(|item| item.clone())
            .collect();
    }

    states
}

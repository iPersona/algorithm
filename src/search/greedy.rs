use std::collections::{HashMap, HashSet};
use std::cmp::Eq;
use std::hash::Hash;
use std::clone::Clone;
use std::fmt::{Debug, Display};

pub fn find_best_covered_state<T: Eq + Hash + Copy + Display + Debug>(
    stations: &HashMap<T, HashSet<T>>,
    states_needed: &HashSet<T>,
) -> HashSet<T> {
    let mut final_stations = HashSet::new();
    let mut best_station: Option<T>;
    let mut states_covered = HashSet::new();
    let mut states_needed_tmp = states_needed.clone();

    println!("states_needed: {:?}", states_needed_tmp);
    while !states_needed_tmp.is_empty() {
        best_station = None;
        states_covered.clear();

        println!("--------------------------------------------------");
        for (station, states_for_station) in stations.iter() {
            let covered: HashSet<_> = states_needed_tmp.intersection(&states_for_station).map(|item| {
                item.clone()
            }).collect();
            println!("station: {:10}\tcovered: {:?}", station, covered);
            if states_covered.is_empty() || covered.len() > states_covered.len() {
                best_station = Some(*station);
                states_covered = covered.clone();
            }
        }
        println!("best_station: {:?}", best_station.unwrap());
        println!("states_covered: {:?}", states_covered);
        states_needed_tmp = states_needed_tmp.difference(&states_covered).map(|item| {
            item.clone()
        }).collect();
        println!("states_needed_tmp:{:?}", states_needed_tmp);
        println!("is_empty: {}", states_needed_tmp.is_empty());
        println!("len: {}", states_needed_tmp.len());
        if best_station != None {
            final_stations.insert(best_station.unwrap());
        }
    }

    final_stations
}

fn get_covered_states<T: Eq + Hash + Copy + Display + Debug>(
    stations: &HashMap<T, HashSet<T>>,
    target_stations: &HashSet<T>) -> HashSet<T> {
    let mut states = HashSet::new();
    
    for station in target_stations.iter() {
        let states_of_station = stations.get(&station).unwrap();
        states = states.union(states_of_station).map(|item| item.clone()).collect();
    }

    states
}

pub fn test_array_success() {
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


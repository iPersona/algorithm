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

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn advent() {
    let orbit_map = read_data();
    println!("Checksum: {}", orbit_checksum(&orbit_map));
    println!("Min Transfers: {}", orbital_transfers(&orbit_map, "YOU", "SAN"));
}

fn read_data() -> HashMap<String, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\w+)\)(\w+)$").unwrap();
    }

    let lines = BufReader::new(File::open("data/day6.txt").expect("Cannot open")).lines();
    let mut orbit_map = HashMap::new();
    for orbit in lines {
        let orbit = orbit.unwrap();
        let caps = regex_captures!(RE, &orbit).expect("Invalid input");
        let parent = capture_group!(caps, 1);
        let satellite = capture_group!(caps, 2);
        assert!(!orbit_map.contains_key(satellite));
        orbit_map.insert(satellite.into(), parent.into());
    }
    orbit_map
}

fn orbit_checksum(orbit_map: &HashMap<String, String>) -> u32 {
    let mut depth_cache = HashMap::new();
    let ret = orbit_map.keys().map(|body| orbit_depth(orbit_map, body, &mut depth_cache)).sum();
    println!("Cache size: {}", depth_cache.len());
    ret
}

fn orbit_depth(orbit_map: &HashMap<String, String>, body: &str, depth_cache: &mut HashMap<String, u32>) -> u32 {
    let mut depth = 0;
    let mut b = body.to_string();
    while &b != "COM" {
        if let Some(&d) = depth_cache.get(&b) {
            depth_cache.insert(body.into(), depth + d);
            return depth + d;
        }
        b = orbit_map.get(&b).expect("Invalid orbit map").to_string();
        depth += 1;
    }
    depth_cache.insert(body.into(), depth);
    depth
}

fn orbit_parents(orbit_map: &HashMap<String, String>, body: &str) -> Vec<String> {
    let mut parents = Vec::new();
    let mut b = body;
    while let Some(parent) = orbit_map.get(b) {
        parents.push(parent.to_string());
        b = parent;
    }
    parents
}

fn orbital_transfers(orbit_map: &HashMap<String, String>, source: &str, dest: &str) -> u32 {
    let source_parents = orbit_parents(orbit_map, source);
    let dest_parents = orbit_parents(orbit_map, dest);
    let (mut s, mut d) = (source_parents.len(), dest_parents.len());
    // count the number of non-common path components
    while s > 0 && d > 0 {
        if source_parents[s-1] != dest_parents[d-1] {
             return (s + d) as u32;
        }
        s -= 1;
        d -= 1;
    }

    panic!("No path!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum() {
        let orbit_map: HashMap<String, String> = [
            ("B", "COM"),
            ("C", "B"),
            ("D", "C"),
            ("E", "D"),
            ("F", "E"),
            ("G", "B"),
            ("H", "G"),
            ("I", "D"),
            ("J", "E"),
            ("K", "J"),
            ("L", "K"),
        ].iter().cloned().map(|(k, v)| (k.into(), v.into())).collect();
        assert_eq!(orbit_checksum(&orbit_map), 42);
    }

    #[test]
    fn transfers() {
        let orbit_map: HashMap<String, String> = [
            ("B", "COM"),
            ("C", "B"),
            ("D", "C"),
            ("E", "D"),
            ("F", "E"),
            ("G", "B"),
            ("H", "G"),
            ("I", "D"),
            ("J", "E"),
            ("K", "J"),
            ("L", "K"),
            ("YOU", "K"),
            ("SAN", "I"),
        ].iter().cloned().map(|(k, v)| (k.into(), v.into())).collect();
        assert_eq!(orbital_transfers(&orbit_map, "YOU", "SAN"), 4);
    }

    #[test]
    fn read_file() {
        assert!(read_data().len() > 0)
    }
}
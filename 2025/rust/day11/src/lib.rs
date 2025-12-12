use std::collections::{HashMap, HashSet};

pub mod parser;

#[derive(Debug, Clone)]
pub struct DeviceNetwork {
    pub device_map: HashMap<String, Vec<String>>,
}

impl DeviceNetwork {
    pub fn new() -> Self {
        Self {
            device_map: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, device: String, outputs: Vec<String>) {
        self.device_map.insert(device, outputs);
    }

    pub fn get_device(&self, device: &str) -> Option<&Vec<String>> {
        self.device_map.get(device)
    }

    pub fn count_paths(&self, start: &str, end: &str, required: Option<&HashSet<String>>) -> usize {
        if required.map_or(true, HashSet::is_empty) {
            let mut memory: HashMap<String, usize> = HashMap::new();
            return self.dfs_without_required(start, end, &mut memory);
        }

        let required_devices = required.unwrap();

        let required_list: Vec<String> = required_devices.iter().cloned().collect();
        let bit_position: HashMap<String, usize> = required_list
            .iter()
            .enumerate()
            .map(|(index, name)| (name.clone(), index))
            .collect();

        let all_visited = if required_list.is_empty() {
            0
        } else {
            (1usize << required_list.len()) - 1
        };

        let mut memory: HashMap<(String, usize), usize> = HashMap::new();
        self.dfs_with_required(start, end, 0, all_visited, &bit_position, &mut memory)
    }

    fn dfs_without_required(
        &self,
        current: &str,
        target: &str,
        memory: &mut HashMap<String, usize>,
    ) -> usize {
        if current == target {
            return 1;
        }
        if let Some(&cached) = memory.get(current) {
            return cached;
        }

        let mut total = 0;
        if let Some(neighbors) = self.get_device(current) {
            for next in neighbors {
                total += self.dfs_without_required(next, target, memory);
            }
        }

        memory.insert(current.to_string(), total);
        total
    }

    fn dfs_with_required(
        &self,
        current: &str,
        target: &str,
        visited: usize,
        must_be_visited: usize,
        bit_position: &HashMap<String, usize>,
        memory: &mut HashMap<(String, usize), usize>,
    ) -> usize {
        let mut visited = visited;
        if let Some(&bit) = bit_position.get(current) {
            visited |= 1 << bit;
        }

        if current == target {
            return if visited == must_be_visited { 1 } else { 0 };
        }

        let key = (current.to_string(), visited);
        if let Some(&cached) = memory.get(&key) {
            return cached;
        }

        let mut total = 0;
        if let Some(neighbors) = self.get_device(current) {
            for next in neighbors {
                total += self.dfs_with_required(
                    next,
                    target,
                    visited,
                    must_be_visited,
                    bit_position,
                    memory,
                );
            }
        }

        memory.insert(key, total);
        total
    }
}

impl Default for DeviceNetwork {
    fn default() -> Self {
        Self::new()
    }
}

fn solve(network: &DeviceNetwork, start: &str, end: &str, required: &[&str]) -> usize {
    let _required: HashSet<String> = required.iter().map(|s| s.to_string()).collect();
    if _required.is_empty() {
        network.count_paths(start, end, None)
    } else {
        network.count_paths(start, end, Some(&_required))
    }
}

pub fn count_paths_from_you_to_out(network: &DeviceNetwork) -> usize {
    solve(network, "you", "out", &[])
}

pub fn count_paths_from_svr_to_out_through_dac_and_fft(network: &DeviceNetwork) -> usize {
    solve(network, "svr", "out", &["dac", "fft"])
}

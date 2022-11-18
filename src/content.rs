use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use sysinfo::{CpuExt, ProcessExt, System, SystemExt, Uid};

pub type UsageMap = HashMap<Uid, VecDeque<f32>>;

fn new_zeroed_vecdeque(capacity: usize) -> VecDeque<f32> {
    let mut vecdeque = VecDeque::with_capacity(capacity);
    for _ in 0..capacity {
        vecdeque.push_back(0.0);
    }
    vecdeque
}

pub fn get_updated_usage_old(sys: &System) -> (String, String, Vec<f32>) {
    let usage_hashmap = get_usage_hashmap(sys);
    let usage_vec = get_sorted_usage_vec(usage_hashmap);

    let name_string = get_name_string(&usage_vec);
    let usage_string = get_usage_string(&usage_vec);
    let cpu_usage = get_cpu_usage(sys);

    (name_string, usage_string, cpu_usage)
}

fn mean_usage<K>(usage_store: &HashMap<K, VecDeque<f32>>) -> HashMap<K, f32>
where
    K: Clone + Eq + Hash,
{
    let mut new_usage_hashmap = HashMap::new();
    for (key, value) in usage_store {
        new_usage_hashmap.insert(key.clone(), value.iter().sum::<f32>() / value.len() as f32);
    }
    new_usage_hashmap
}

pub fn get_updated_usage(
    sys: &System,
    mut usage_store: UsageMap,
) -> (String, String, Vec<f32>, UsageMap) {
    let new_usage_hashmap = get_usage_hashmap(sys);
    usage_store = update_usage_vecdeques(usage_store, new_usage_hashmap);
    let mean_usage_hm = mean_usage(&usage_store);
    let usage_vec = get_sorted_usage_vec(mean_usage_hm);

    let name_string = get_name_string(&usage_vec);
    let usage_string = get_usage_string(&usage_vec);
    let cpu_usage = get_cpu_usage(sys);

    (name_string, usage_string, cpu_usage, usage_store)
}

fn update_usage_vecdeques(
    mut usage_store: UsageMap,
    new_usage: HashMap<Uid, f32>,
) -> HashMap<Uid, VecDeque<f32>> {
    for uid in new_usage.keys() {
        usage_store
            .entry(uid.clone())
            .or_insert_with(|| new_zeroed_vecdeque(10));
    }
    for (uid, usage) in &new_usage {
        usage_store.get_mut(uid).unwrap().pop_front();
        usage_store.get_mut(uid).unwrap().push_back(*usage);
    }
    let mut missing_keys = Vec::new();
    for uid in usage_store.keys() {
        if !new_usage.contains_key(uid) {
            missing_keys.push(uid.clone());
        }
    }
    for uid in missing_keys {
        usage_store.get_mut(&uid).unwrap().push_back(0.0);
    }

    usage_store
}

fn get_usage_hashmap(sys: &System) -> HashMap<Uid, f32> {
    let mut user_usage = HashMap::<Uid, f32>::new();

    for process in sys.processes().values() {
        if let Some(uid) = process.user_id() {
            let usage = process.cpu_usage();
            *user_usage.entry(uid.clone()).or_insert(0.0) += usage;
        }
    }

    user_usage
}

fn get_cpu_usage(sys: &System) -> Vec<f32> {
    sys.cpus().iter().map(|c| c.cpu_usage()).collect()
}

fn get_sorted_usage_vec<K>(user_usage: HashMap<K, f32>) -> Vec<(K, f32)> {
    let mut usage_vec = user_usage.into_iter().collect::<Vec<_>>();
    usage_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    usage_vec
}

fn get_name_string(usage_vec: &[(Uid, f32)]) -> String {
    let name_string: Vec<_> = usage_vec
        .iter()
        .map(|(user_id, _)| passwd::Passwd::from_uid(**user_id).unwrap().gecos)
        .collect();
    name_string.join("\n")
}

fn get_usage_string(usage_vec: &[(Uid, f32)]) -> String {
    let usage_string: Vec<_> = usage_vec
        .iter()
        .map(|(_, usage)| format!("{:>8}", format!("{:.2}", usage)))
        .collect();

    usage_string.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mean_hash_deque() {
        let mut usage_map = HashMap::new();
        usage_map.insert("a", VecDeque::from_iter(vec![1f32, 2.0, 3.0]));
        usage_map.insert("b", VecDeque::from_iter(vec![10.0, 20.0, 30.0]));
        usage_map.insert("c", VecDeque::from_iter(vec![4.0, 5.0, 6.0]));

        let mut expected_result = HashMap::new();
        expected_result.insert("a", 2f32);
        expected_result.insert("b", 20f32);
        expected_result.insert("c", 5f32);

        let means = mean_usage(&usage_map);

        assert_eq!(expected_result, means);
    }
}

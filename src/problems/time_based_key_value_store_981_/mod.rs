use std::collections::{BTreeMap, HashMap};

pub struct TimeMap {
    tm: HashMap<String, BTreeMap<i32, String>>,
}

impl Default for TimeMap {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeMap {
    pub fn new() -> Self {
        TimeMap { tm: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.tm
            .entry(key)
            .or_insert(BTreeMap::new())
            .insert(timestamp, value);
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if !self.tm.contains_key(&key) {
            return String::new();
        }
        if let Some((_, v)) = self.tm[&key].range(0..=timestamp).last() {
            return v.clone();
        }
        String::new()
    }
}

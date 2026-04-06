use std::collections::HashMap;

pub struct ProductGraph {
    
    pub adjacencies: HashMap<u32, Vec<u32>>,
}

impl ProductGraph {
    pub fn new() -> Self {
        Self {
            adjacencies: HashMap::new(),
        }
    }

    pub fn connect(&mut self, id1: u32, id2: u32) {
        self.adjacencies.entry(id1).or_insert_with(Vec::new).push(id2);
        self.adjacencies.entry(id2).or_insert_with(Vec::new).push(id1);
    }

    pub fn get_recommendations(&self, id: u32) -> Vec<u32> {
        self.adjacencies.get(&id).cloned().unwrap_or_default()
    }
}
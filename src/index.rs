use std::collections::HashMap;
use crate::product::Product;

pub struct SearchIndex {
    
    pub keyword_map: HashMap<String, Vec<u32>>,
    
    pub registry: HashMap<u32, Product>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            keyword_map: HashMap::new(),
            registry: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;
        
        
        for word in product.name.to_lowercase().split_whitespace() {
            self.keyword_map
                .entry(word.to_string())
                .or_insert_with(Vec::new)
                .push(id);
        }

        self.registry.insert(id, product);
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let query = query.to_lowercase();
        if let Some(ids) = self.keyword_map.get(&query) {
            ids.iter()
                .filter_map(|id| self.registry.get(id))
                .collect::<Vec<&Product>>()
        } else {
            Vec::new()
        }
    }
}
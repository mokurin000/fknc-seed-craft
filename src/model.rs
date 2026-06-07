use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Seed {
    pub name: String,
    pub price: i64,
    pub count: usize,
}

impl Seed {
    pub fn new(name: &str, price: i64, count: usize) -> Self {
        Seed {
            name: name.to_string(),
            price,
            count,
        }
    }
}

#[derive(Debug)]
pub struct Solution {
    pub price: i64,
    pub probability: i32,
    pub energy: i32,
    pub counts: HashMap<String, usize>,
    pub chosen: Vec<Seed>,
}

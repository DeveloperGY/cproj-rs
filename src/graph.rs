use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Clone)]
pub struct Graph<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    nodes: HashSet<T>,
    edges: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn _has_node(&self, node: &T) -> bool {
        self.nodes.contains(node)
    }

    pub fn create_node(&mut self, value: T) {
        if !self.nodes.contains(&value) {
            self.nodes.insert(value.clone());
            self.edges.insert(value.clone(), HashSet::new());
        }
    }

    pub fn create_edge(&mut self, from: &T, to: &T) {
        if self.nodes.contains(from) && self.nodes.contains(to) {
            self.edges.get_mut(from).unwrap().insert(to.clone());
        }
    }

    pub fn get_connected(&self, node: &T) -> Option<&HashSet<T>> {
        self.edges.get(node)
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

//! Interaction Field - Spatial Indexing for ParadoxOS
//!
//! In a physics-native OS, "space" is defined by interaction connectivity.
//! This module provides spatial indexing and neighborhood queries based on
//! the interaction graph.

use crate::types::{UniverseID, InteractionID};
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents the interaction field (spatial structure)
#[derive(Debug, Default)]
pub struct InteractionField {
    /// Adjacency list: UniverseID -> Vec<(InteractionID, UniverseID)>
    adjacency: HashMap<UniverseID, Vec<(InteractionID, UniverseID)>>,
    
    /// Reverse lookup: InteractionID -> (Source, Target)
    interactions: HashMap<InteractionID, (UniverseID, UniverseID)>,
}

impl InteractionField {
    /// Create a new empty interaction field
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            interactions: HashMap::new(),
        }
    }

    /// Register a new interaction
    pub fn register_interaction(&mut self, id: InteractionID, source: UniverseID, target: UniverseID) {
        // Add to source connections
        self.adjacency.entry(source)
            .or_default()
            .push((id, target));
            
        // Add to target connections (interactions are bidirectional for locality)
        self.adjacency.entry(target)
            .or_default()
            .push((id, source));
            
        self.interactions.insert(id, (source, target));
    }

    /// Remove an interaction
    pub fn unregister_interaction(&mut self, id: InteractionID) {
        if let Some((source, target)) = self.interactions.remove(&id) {
            // Remove from source
            if let Some(links) = self.adjacency.get_mut(&source) {
                links.retain(|(i, _)| *i != id);
            }
            
            // Remove from target
            if let Some(links) = self.adjacency.get_mut(&target) {
                links.retain(|(i, _)| *i != id);
            }
        }
    }

    /// Get immediate neighbors of a universe
    pub fn get_neighbors(&self, universe_id: UniverseID) -> Vec<UniverseID> {
        self.adjacency.get(&universe_id)
            .map(|links| links.iter().map(|(_, target)| *target).collect())
            .unwrap_or_default()
    }

    /// Calculate interaction density (local connectivity)
    ///
    /// Higher density = more interactions = less stability (Law of Interaction Density)
    pub fn get_density(&self, universe_id: UniverseID) -> f64 {
        self.adjacency.get(&universe_id)
            .map(|links| links.len() as f64)
            .unwrap_or(0.0)
    }

    /// Find shortest interaction path between two universes (BFS)
    ///
    /// Returns list of InteractionIDs to traverse
    pub fn find_path(&self, start: UniverseID, end: UniverseID) -> Option<Vec<InteractionID>> {
        if start == end {
            return Some(Vec::new());
        }

        let mut queue = VecDeque::new();
        queue.push_back((start, Vec::new()));
        
        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some((current, path)) = queue.pop_front() {
            if current == end {
                return Some(path);
            }
            
            if let Some(neighbors) = self.adjacency.get(&current) {
                for (interaction_id, neighbor) in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(*neighbor);
                        let mut new_path = path.clone();
                        new_path.push(*interaction_id);
                        queue.push_back((*neighbor, new_path));
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_registration() {
        let mut field = InteractionField::new();
        let i1 = InteractionID(1);
        let u1 = UniverseID(1);
        let u2 = UniverseID(2);

        field.register_interaction(i1, u1, u2);

        assert_eq!(field.get_neighbors(u1), vec![u2]);
        assert_eq!(field.get_neighbors(u2), vec![u1]);
        assert_eq!(field.get_density(u1), 1.0);
    }

    #[test]
    fn test_path_finding() {
        let mut field = InteractionField::new();
        // Chain: U1 -I1-> U2 -I2-> U3
        field.register_interaction(InteractionID(1), UniverseID(1), UniverseID(2));
        field.register_interaction(InteractionID(2), UniverseID(2), UniverseID(3));

        let path = field.find_path(UniverseID(1), UniverseID(3)).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], InteractionID(1));
        assert_eq!(path[1], InteractionID(2));
    }
}

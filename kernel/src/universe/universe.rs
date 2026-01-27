//! Universe - the execution unit that replaces processes/threads

use crate::types::{InteractionID, StateVector, UniverseID};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Universe - replaces process, thread, container, and VM
///
/// A universe is a self-contained computational entity with:
/// - Isolated state (cannot access other universes directly)
/// - Energy budget (computational resources)
/// - Entropy measure (complexity/disorder)
/// - Local time (timeline_index)
/// - Stability score (resistance to collapse)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Universe {
    /// Unique identifier
    pub id: UniverseID,

    /// Computational state (opaque, compressed via ParadoxLF)
    pub state_vector: StateVector,

    /// Available computational energy
    ///
    /// Invariant: energy >= 0.0
    pub energy: f64,

    /// Complexity/disorder measure
    ///
    /// Invariant: entropy >= 0.0, monotonically increasing (LAW 2)
    pub entropy: f64,

    /// Stability score (0.0 = unstable, 1.0 = stable)
    ///
    /// If stability < threshold, universe will collapse (LAW 9)
    pub stability_score: f64,

    /// Local time counter (LAW 7)
    ///
    /// Each universe experiences its own time based on interaction density
    pub timeline_index: i64,

    /// Set of active interactions involving this universe
    pub interaction_links: HashSet<InteractionID>,

    /// Creation timestamp (monotonic counter)
    pub(crate) creation_time: u64,

    /// Last evolution timestamp
    pub(crate) last_evolution: u64,
}

impl Universe {
    /// Create a new universe with specified parameters
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `initial_energy` - Energy budget to allocate
    ///
    /// # Invariants
    ///
    /// - Initial entropy is 0.0 (LAW 0: existence creates order)
    /// - Initial stability is 1.0 (new universes start stable)
    /// - Timeline starts at 0
    pub fn new(id: UniverseID, initial_energy: f64) -> Self {
        Self {
            id,
            state_vector: StateVector::empty(),
            energy: initial_energy,
            entropy: 0.0,
            stability_score: 1.0,
            timeline_index: 0,
            interaction_links: HashSet::new(),
            creation_time: 0,
            last_evolution: 0,
        }
    }

    /// Add an interaction link
    pub fn add_interaction(&mut self, interaction_id: InteractionID) {
        self.interaction_links.insert(interaction_id);
    }

    /// Remove an interaction link
    pub fn remove_interaction(&mut self, interaction_id: InteractionID) -> bool {
        self.interaction_links.remove(&interaction_id)
    }

    /// Get interaction density (for temporal relativity calculations)
    ///
    /// Higher density → slower local time (LAW 7)
    pub fn interaction_density(&self) -> f64 {
        self.interaction_links.len() as f64
    }

    /// Calculate internal resistance to evolution (LAW 4)
    ///
    /// resistance = entropy × instability_factor
    pub fn internal_resistance(&self) -> f64 {
        let instability_factor = 1.0 - self.stability_score;
        self.entropy * instability_factor
    }

    /// Check if universe is unstable and should collapse
    ///
    /// # Arguments
    ///
    /// * `threshold` - Stability threshold (default: 0.3)
    pub fn should_collapse(&self, threshold: f64) -> bool {
        self.stability_score < threshold
    }

    /// Advance local time based on interaction density (LAW 7)
    ///
    /// Δt ∝ 1 / interaction_density
    pub fn advance_time(&mut self) {
        let density = self.interaction_density();
        // Time dilation: higher interaction density → slower time advancement
        let time_delta = if density > 0.0 {
            (1.0 / (1.0 + density)).ceil() as i64
        } else {
            1 // Fast-forward when idle
        };
        
        self.timeline_index += time_delta;
    }

    /// Increase entropy (LAW 2)
    ///
    /// # Panics
    ///
    /// Panics if delta is negative (entropy cannot decrease)
    pub fn increase_entropy(&mut self, delta: f64) {
        assert!(delta >= 0.0, "Entropy cannot decrease (LAW 2 violation)");
        self.entropy += delta;
    }

    /// Transfer energy (with conservation check)
    ///
    /// # Arguments
    ///
    /// * `amount` - Amount to transfer (positive = receive, negative = send)
    ///
    /// # Returns
    ///
    /// `Ok(())` if successful, `Err` if insufficient energy
    pub fn transfer_energy(&mut self, amount: f64) -> crate::error::Result<()> {
        // Check if withdrawing more than available
        if amount < 0.0 && self.energy < amount.abs() {
            return Err(crate::error::KernelError::InsufficientEnergy {
                requested: amount.abs(),
                available: self.energy,
            });
        }
        
        self.energy += amount;
        
        // Ensure energy never goes negative (safety check)
        if self.energy < 0.0 {
            return Err(crate::error::KernelError::InsufficientEnergy {
                requested: amount.abs(),
                available: self.energy - amount,
            });
        }
        
        Ok(())
    }

    /// Update stability based on current state
    ///
    /// Stability decreases with high entropy and low energy
    pub fn update_stability(&mut self) {
        // Simple stability model: decreases with entropy/energy ratio
        let entropy_factor = (-self.entropy * 0.01).exp();
        let energy_factor = if self.energy > 0.0 {
            (self.energy / 100.0).min(1.0)
        } else {
            0.0
        };
        
        self.stability_score = (entropy_factor * energy_factor).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_creation() {
        let u = Universe::new(UniverseID(1), 100.0);
        
        assert_eq!(u.id, UniverseID(1));
        assert_eq!(u.energy, 100.0);
        assert_eq!(u.entropy, 0.0);
        assert_eq!(u.stability_score, 1.0);
        assert_eq!(u.timeline_index, 0);
        assert!(u.interaction_links.is_empty());
    }

    #[test]
    fn test_interaction_density() {
        let mut u = Universe::new(UniverseID(1), 100.0);
        assert_eq!(u.interaction_density(), 0.0);
        
        u.add_interaction(InteractionID(1));
        u.add_interaction(InteractionID(2));
        assert_eq!(u.interaction_density(), 2.0);
    }

    #[test]
    fn test_time_advancement() {
        let mut u = Universe::new(UniverseID(1), 100.0);
        
        // Idle universe: fast time
        u.advance_time();
        assert_eq!(u.timeline_index, 1);
        
        // Busy universe: slower time (dilated)
        u.add_interaction(InteractionID(1));
        u.add_interaction(InteractionID(2));
        u.add_interaction(InteractionID(3));
        u.advance_time();
        // With 3 interactions, time advances slower
        assert!(u.timeline_index < 3);
    }

    #[test]
    #[should_panic(expected = "Entropy cannot decrease")]
    fn test_entropy_decrease_panics() {
        let mut u = Universe::new(UniverseID(1), 100.0);
        u.increase_entropy(-1.0); // Should panic
    }

    #[test]
    fn test_energy_transfer() {
        let mut u = Universe::new(UniverseID(1), 100.0);
        
        // Successful transfer
        assert!(u.transfer_energy(-50.0).is_ok());
        assert_eq!(u.energy, 50.0);
        
        // Failed transfer (insufficient)
        assert!(u.transfer_energy(-100.0).is_err());
    }

    #[test]
    fn test_stability() {
        let mut u = Universe::new(UniverseID(1), 100.0);
        
        // High entropy decreases stability
        u.entropy = 100.0;
        u.update_stability();
        assert!(u.stability_score < 1.0);
        
        // Low energy also decreases stability
        u.energy = 1.0;
        u.update_stability();
        assert!(u.stability_score < 0.5);
    }
}

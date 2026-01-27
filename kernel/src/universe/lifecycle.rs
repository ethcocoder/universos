//! Advanced universe lifecycle management

use super::Universe;
use crate::error::{KernelError, Result};
use crate::types::{StateVector, UniverseID};

impl Universe {
    /// Branch universe timeline - create a copy with divergent state
    ///
    /// This creates a new universe by copying the current state and
    /// allowing it to evolve independently. The branch inherits:
    /// - Current state vector (deep copy)
    /// - Half of parent's energy
    /// - Current entropy level
    /// - Zero stability initially (must stabilize)
    ///
    /// # Arguments
    ///
    /// * `new_id` - ID for the branched universe
    ///
    /// # Returns
    ///
    /// New universe instance representing the branch
    ///
    /// # Laws Enforced
    ///
    /// - LAW 1: Energy split between parent and branch
    /// - LAW 2: Branching increases total entropy
    pub fn branch(&mut self, new_id: UniverseID) -> Result<Universe> {
        // Calculate energy cost of memory duplication (LAW 8)
        let memory_cost = self.state_vector.potential_energy();
        
        // Check if we have enough energy (base threshold + memory cost)
        if self.energy < (10.0 + memory_cost) {
            return Err(KernelError::InsufficientEnergy {
                requested: 10.0 + memory_cost,
                available: self.energy,
            });
        }

        // Deduct memory copy cost from parent
        self.energy -= memory_cost;

        // Split remaining energy 50/50
        let branch_energy = self.energy / 2.0;
        self.energy /= 2.0;

        // Create branched universe
        let branched = Universe {
            id: new_id,
            state_vector: self.state_vector.clone(),
            energy: branch_energy,
            entropy: self.entropy, // Inherit current entropy
            stability_score: 0.5, // Starts semi-stable
            timeline_index: self.timeline_index,
            interaction_links: std::collections::HashSet::new(), // No inherited interactions
            creation_time: 0, // Will be set by kernel
            last_evolution: 0,
        };

        // Branching increases parent's entropy (LAW 2)
        self.increase_entropy(1.0);

        Ok(branched)
    }

    /// Check if this universe can merge with another
    ///
    /// Universes can merge if:
    /// - Both are stable (> 0.7 stability)
    /// - Energy difference < 20%
    /// - Entropy levels compatible (within 30%)
    /// - Timeline indices are close (< 10 steps apart)
    ///
    /// # Arguments
    ///
    /// * `other` - Universe to potentially merge with
    ///
    /// # Returns
    ///
    /// `true` if merge is possible, `false` otherwise
    pub fn can_merge_with(&self, other: &Universe) -> bool {
        // Both must be stable
        if self.stability_score < 0.7 || other.stability_score < 0.7 {
            return false;
        }

        // Energy levels must be compatible (within 20%)
        let energy_ratio = if self.energy > other.energy {
            other.energy / self.energy
        } else {
            self.energy / other.energy
        };

        if energy_ratio < 0.8 {
            return false;
        }

        // Entropy must be compatible (within 30%)
        let entropy_diff = (self.entropy - other.entropy).abs();
        let max_entropy = self.entropy.max(other.entropy);
        
        if max_entropy > 0.0 && (entropy_diff / max_entropy) > 0.3 {
            return false;
        }

        // Timelines must be close
        let timeline_diff = (self.timeline_index - other.timeline_index).abs();
        if timeline_diff > 10 {
            return false;
        }

        true
    }

    /// Merge with another universe
    ///
    /// Combines two compatible universes into one. The merged universe:
    /// - Sums energies (LAW 1 conservation)
    /// - Takes maximum entropy (LAW 2 monotonicity)
    /// - Averages stability
    /// - Combines interaction links
    /// - Uses newer timeline
    ///
    /// # Arguments
    ///
    /// * `other` - Universe to merge with (will be consumed)
    ///
    /// # Returns
    ///
    /// `Ok(())` if merge successful
    ///
    /// # Laws Enforced
    ///
    /// - LAW 1: Energy conserved (sum)
    /// - LAW 2: Entropy increases (max + merge cost)
    pub fn merge_with(&mut self, other: Universe) -> Result<()> {
        if !self.can_merge_with(&other) {
            return Err(KernelError::Generic {
                message: format!(
                    "Cannot merge {} with {}: conditions not met",
                    self.id, other.id
                ),
            });
        }

        // Combine energies (LAW 1)
        self.energy += other.energy;

        // Take maximum entropy + merge cost (LAW 2)
        self.entropy = self.entropy.max(other.entropy) + 2.0;

        // Average stability
        self.stability_score = (self.stability_score + other.stability_score) / 2.0;

        // Use newer timeline
        self.timeline_index = self.timeline_index.max(other.timeline_index);

        // Combine interaction links
        self.interaction_links.extend(other.interaction_links);

        Ok(())
    }

    /// Snapshot current state for potential rollback
    ///
    /// Creates a compressed snapshot of the universe state that can
    /// be used to restore the universe to this point.
    ///
    /// # Returns
    ///
    /// Compressed state snapshot
    pub fn snapshot(&self) -> UniverseSnapshot {
        UniverseSnapshot {
            state_vector: self.state_vector.clone(),
            energy: self.energy,
            entropy: self.entropy,
            stability_score: self.stability_score,
            timeline_index: self.timeline_index,
        }
    }

    /// Restore universe from a snapshot
    ///
    /// # Arguments
    ///
    /// * `snapshot` - Previously created snapshot
    ///
    /// # Laws Enforced
    ///
    /// - LAW 2: Entropy can only increase, so we take max
    pub fn restore_from_snapshot(&mut self, snapshot: UniverseSnapshot) {
        self.state_vector = snapshot.state_vector;
        self.energy = snapshot.energy;
        // Entropy can only increase (LAW 2)
        self.entropy = self.entropy.max(snapshot.entropy);
        self.stability_score = snapshot.stability_score;
        self.timeline_index = snapshot.timeline_index;
    }
}

/// Snapshot of universe state for rollback/branching
#[derive(Clone, Debug)]
pub struct UniverseSnapshot {
    /// State vector at snapshot time
    pub state_vector: StateVector,
    /// Energy level
    pub energy: f64,
    /// Entropy level
    pub entropy: f64,
    /// Stability score
    pub stability_score: f64,
    /// Timeline index
    pub timeline_index: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch() {
        let mut parent = Universe::new(UniverseID(1), 100.0);
        parent.entropy = 10.0;

        let branch = parent.branch(UniverseID(2)).unwrap();

        // Energy split (minus potential energy cost, if any)
        // Since state vector is small/empty, potential energy is negligible, so 50/50 holds
        assert_eq!(parent.energy, 50.0);
        assert_eq!(branch.energy, 50.0);

        // Entropy inherited and increased in parent
        assert!(parent.entropy > 10.0);
        assert_eq!(branch.entropy, 10.0);

        // Branch has no interactions
        assert!(branch.interaction_links.is_empty());
    }

    #[test]
    fn test_branch_insufficient_energy() {
        let mut parent = Universe::new(UniverseID(1), 5.0);
        let result = parent.branch(UniverseID(2));
        assert!(result.is_err());
    }

    #[test]
    fn test_can_merge_compatible() {
        let mut u1 = Universe::new(UniverseID(1), 100.0);
        let mut u2 = Universe::new(UniverseID(2), 90.0);

        u1.stability_score = 0.8;
        u2.stability_score = 0.8;
        u1.entropy = 10.0;
        u2.entropy = 12.0;

        assert!(u1.can_merge_with(&u2));
    }

    #[test]
    fn test_cannot_merge_unstable() {
        let mut u1 = Universe::new(UniverseID(1), 100.0);
        let mut u2 = Universe::new(UniverseID(2), 90.0);

        u1.stability_score = 0.5; // Too low
        u2.stability_score = 0.8;

        assert!(!u1.can_merge_with(&u2));
    }

    #[test]
    fn test_merge() {
        let mut u1 = Universe::new(UniverseID(1), 100.0);
        let mut u2 = Universe::new(UniverseID(2), 90.0);

        u1.stability_score = 0.8;
        u2.stability_score = 0.9;
        u1.entropy = 10.0;
        u2.entropy = 12.0;

        let initial_energy = u1.energy + u2.energy;
        u1.merge_with(u2).unwrap();

        // Energy conserved
        assert_eq!(u1.energy, initial_energy);

        // Entropy increased (max + cost)
        assert!(u1.entropy > 12.0);

        // Stability averaged (approximately)
        assert!((u1.stability_score - 0.85).abs() < 0.0001);
    }

    #[test]
    fn test_snapshot_restore() {
        let mut universe = Universe::new(UniverseID(1), 100.0);
        universe.entropy = 20.0;
        universe.stability_score = 0.7;

        let snapshot = universe.snapshot();

        // Modify universe
        universe.energy = 50.0;
        universe.entropy = 30.0;
        universe.stability_score = 0.5;

        // Restore
        universe.restore_from_snapshot(snapshot);

        assert_eq!(universe.energy, 100.0);
        // Entropy can only increase, so it stays at 30
        assert_eq!(universe.entropy, 30.0);
        assert_eq!(universe.stability_score, 0.7);
    }
}

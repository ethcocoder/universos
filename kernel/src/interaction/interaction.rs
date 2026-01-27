//! Interaction - the only causal channel between universes

use crate::types::{InteractionID, UniverseID};
use serde::{Deserialize, Serialize};

/// Interaction - the ONLY way universes can influence each other
///
/// Enforces LAW 3: Interaction Primacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Unique identifier
    pub id: InteractionID,

    /// Source universe
    pub source: UniverseID,

    /// Target universe
    pub target: UniverseID,

    /// Coupling strength (0.0 to 1.0)
    ///
    /// Higher values = stronger influence
    pub coupling_strength: f64,

    /// Energy transfer rate (momentum)
    ///
    /// Positive = source → target
    /// Negative = target → source
    pub momentum: f64,

    /// Natural decay rate per evolution step
    ///
    /// Typical range: 0.01 to 0.1
    pub decay_rate: f64,

    /// Age in evolution steps
    pub(crate) age: u64,

    /// Total energy transferred through this interaction
    pub(crate) total_energy_transferred: f64,

    /// Events traveling Source -> Target
    pub(crate) forward_events: crate::interaction::event::EventQueue,

    /// Events traveling Target -> Source
    pub(crate) backward_events: crate::interaction::event::EventQueue,
}

impl Interaction {
    /// Create a new interaction
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `source` - Source universe ID
    /// * `target` - Target universe ID
    /// * `coupling_strength` - Strength (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// `Ok(Interaction)` if valid, `Err` if coupling invalid
    pub fn new(
        id: InteractionID,
        source: UniverseID,
        target: UniverseID,
        coupling_strength: f64,
    ) -> crate::error::Result<Self> {
        if !(0.0..=1.0).contains(&coupling_strength) {
            return Err(crate::error::KernelError::InvalidCoupling {
                value: coupling_strength,
            });
        }

        Ok(Self {
            id,
            source,
            target,
            coupling_strength,
            momentum: 0.0,
            decay_rate: crate::constants::DEFAULT_DECAY_RATE,
            age: 0,
            total_energy_transferred: 0.0,
            forward_events: crate::interaction::event::EventQueue::new(),
            backward_events: crate::interaction::event::EventQueue::new(),
        })
    }

    /// Apply natural decay
    ///
    /// Weakens the interaction over time
    pub fn apply_decay(&mut self) {
        self.coupling_strength *= 1.0 - self.decay_rate;
        self.coupling_strength = self.coupling_strength.max(0.0);
        self.age += 1;
    }

    /// Calculate energy transfer for this step
    ///
    /// Transfer = coupling_strength × momentum × step_fraction
    pub fn calculate_energy_transfer(&self, step_fraction: f64) -> f64 {
        self.coupling_strength * self.momentum * step_fraction
    }

    /// Record energy transfer
    pub fn record_transfer(&mut self, amount: f64) {
        self.total_energy_transferred += amount.abs();
    }

    /// Check if interaction is still active
    ///
    /// Becomes inactive when coupling drops too low
    pub fn is_active(&self) -> bool {
        self.coupling_strength > 0.001
    }

    /// Set momentum based on energy gradient
    pub fn set_momentum(&mut self, energy_source: f64, energy_target: f64) {
        // Momentum flows from high to low energy
        let gradient = energy_source - energy_target;
        self.momentum = gradient * self.coupling_strength * 0.01;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interaction_creation() {
        let i = Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            0.8,
        ).unwrap();

        assert_eq!(i.id, InteractionID(1));
        assert_eq!(i.source, UniverseID(1));
        assert_eq!(i.target, UniverseID(2));
        assert_eq!(i.coupling_strength, 0.8);
        assert_eq!(i.momentum, 0.0);
    }

    #[test]
    fn test_invalid_coupling() {
        assert!(Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            1.5, // Invalid!
        ).is_err());

        assert!(Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            -0.1, // Invalid!
        ).is_err());
    }

    #[test]
    fn test_decay() {
        let mut i = Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            1.0,
        ).unwrap();

        let initial = i.coupling_strength;
        i.apply_decay();
        
        assert!(i.coupling_strength < initial);
        assert_eq!(i.age, 1);
    }

    #[test]
    fn test_energy_transfer() {
        let mut i = Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            0.5,
        ).unwrap();

        i.momentum = 10.0;
        
        let transfer = i.calculate_energy_transfer(0.1);
        assert_eq!(transfer, 0.5 * 10.0 * 0.1);
        
        i.record_transfer(transfer);
        assert_eq!(i.total_energy_transferred, transfer);
    }

    #[test]
    fn test_momentum_from_gradient() {
        let mut i = Interaction::new(
            InteractionID(1),
            UniverseID(1),
            UniverseID(2),
            0.8,
        ).unwrap();

        // Source has more energy
        i.set_momentum(100.0, 50.0);
        assert!(i.momentum > 0.0); // Energy flows source → target

        // Target has more energy
        i.set_momentum(30.0, 80.0);
        assert!(i.momentum < 0.0); // Energy flows target → source
    }
}

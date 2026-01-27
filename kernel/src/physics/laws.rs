//! Physics laws enforcement module

use crate::constants::*;
use crate::error::{KernelError, Result};

/// Law enforcement functions
///
/// These functions verify that kernel operations comply with the 13 fundamental laws

/// LAW 0: Existence - All entities must have state
pub fn verify_existence<T>(entity: &Option<T>, name: &str) -> Result<()> {
    if entity.is_none() {
        Err(KernelError::Generic {
            message: format!("LAW 0 violation: {} does not exist", name),
        })
    } else {
        Ok(())
    }
}

/// LAW 1: Energy Conservation
///
/// Verify total energy is conserved within epsilon tolerance
pub fn verify_energy_conservation(initial: f64, current: f64) -> Result<()> {
    let delta = (current - initial).abs();
    
    if delta > ENERGY_EPSILON {
        Err(KernelError::ConservationViolation {
            expected: initial,
            actual: current,
            delta,
        })
    } else {
        Ok(())
    }
}

/// LAW 2: Entropy Monotonicity
///
/// Verify entropy has not decreased
pub fn verify_entropy_increase(previous: f64, current: f64) -> Result<()> {
    if current < previous - ENERGY_EPSILON {
        Err(KernelError::EntropyDecrease {
            previous,
            current,
            delta: current - previous,
        })
    } else {
        Ok(())
    }
}

/// LAW 3: Interaction Primacy
///
/// Verified at compile time by type system - universes can only communicate via Interaction

/// LAW 4: Force-Resistance Velocity
///
/// Check if evolution condition is met: pressure > resistance
pub fn check_evolution_condition(pressure: f64, resistance: f64) -> bool {
    pressure > resistance
}

/// LAW 7: Temporal Relativity
///
/// Calculate time dilation factor based on interaction density
///
/// Δt ∝ 1 / interaction_density
pub fn calculate_time_dilation(interaction_density: f64) -> f64 {
    1.0 / (1.0 + interaction_density)
}

/// LAW 9: Stability and Collapse
///
/// Check if universe should collapse
pub fn should_collapse(stability: f64) -> bool {
    stability < STABILITY_THRESHOLD
}

/// LAW 10: Security as Physics
///
/// Detect energy anomalies (potential security violations)
pub fn detect_anomaly(expected: f64, actual: f64) -> bool {
    (expected - actual).abs() > ENERGY_EPSILON
}

/// LAW 13: Forbidden Concepts
///
/// These are enforced at code review / compile time:
/// - No std::thread usage
/// - No Arc<Mutex<T>> patterns
/// - No global clocks (std::time::Instant)
/// - No unsafe blocks (already denied in lib.rs)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_conservation() {
        assert!(verify_energy_conservation(100.0, 100.0).is_ok());
        assert!(verify_energy_conservation(100.0, 100.0 + ENERGY_EPSILON / 2.0).is_ok());
        assert!(verify_energy_conservation(100.0, 101.0).is_err());
    }

    #[test]
    fn test_entropy_increase() {
        assert!(verify_entropy_increase(10.0, 10.1).is_ok());
        assert!(verify_entropy_increase(10.0, 10.0).is_ok()); // Equal is ok
        assert!(verify_entropy_increase(10.0, 9.9).is_err());
    }

    #[test]
    fn test_evolution_condition() {
        assert!(check_evolution_condition(10.0, 5.0)); // Can evolve
        assert!(!check_evolution_condition(5.0, 10.0)); // Blocked
    }

    #[test]
    fn test_time_dilation() {
        // No interactions: no dilation (1.0)
        assert_eq!(calculate_time_dilation(0.0), 1.0);
        
        // High interaction density: significant dilation
        let dilated = calculate_time_dilation(10.0);
        assert!(dilated < 0.2);
    }

    #[test]
    fn test_collapse_detection() {
        assert!(should_collapse(0.2)); // Below threshold
        assert!(!should_collapse(0.5)); // Above threshold
    }

    #[test]
    fn test_anomaly_detection() {
        assert!(!detect_anomaly(100.0, 100.0));
        assert!(detect_anomaly(100.0, 99.0)); // Anomaly!
    }
}

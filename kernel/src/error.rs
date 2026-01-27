//! Error types for ParadoxOS kernel

use thiserror::Error;

/// Result type for kernel operations
pub type Result<T> = std::result::Result<T, KernelError>;

/// Kernel error types
#[derive(Error, Debug, Clone, PartialEq)]
pub enum KernelError {
    /// Energy conservation violation (LAW 1)
    #[error("Energy conservation violated: expected {expected:.6}J, got {actual:.6}J (Δ={delta:.9}J)")]
    ConservationViolation {
        /// Expected total energy
        expected: f64,
        /// Actual total energy
        actual: f64,
        /// Difference
        delta: f64,
    },

    /// Entropy monotonicity violation (LAW 2)
    #[error("Entropy decreased: {previous:.6} → {current:.6} (ΔS={delta:.6})")]
    EntropyDecrease {
        /// Previous entropy
        previous: f64,
        /// Current entropy
        current: f64,
        /// Delta (negative)
        delta: f64,
    },

    /// Insufficient energy
    #[error("Insufficient energy: requested {requested:.2}J, available {available:.2}J")]
    InsufficientEnergy {
        /// Requested amount
        requested: f64,
        /// Available amount
        available: f64,
    },

    /// Universe not found
    #[error("Universe {id:?} not found")]
    UniverseNotFound {
        /// Universe ID
        id: crate::types::UniverseID,
    },

    /// Interaction not found
    #[error("Interaction {id:?} not found")]
    InteractionNotFound {
        /// Interaction ID
        id: crate::types::InteractionID,
    },

    /// Evolution condition not met (LAW 4)
    #[error("Evolution condition not met: pressure={pressure:.6} ≤ resistance={resistance:.6}")]
    EvolutionBlocked {
        /// Interaction pressure
        pressure: f64,
        /// Internal resistance
        resistance: f64,
    },

    /// Invalid coupling strength
    #[error("Invalid coupling strength: {value:.6} (must be 0.0 ≤ coupling ≤ 1.0)")]
    InvalidCoupling {
        /// Invalid value
        value: f64,
    },

    /// Forbidden operation (LAW 13)
    #[error("Forbidden operation: {operation} violates LAW 13")]
    ForbiddenOperation {
        /// Operation description
        operation: String,
    },

    /// State vector error
    #[error("State vector error: {message}")]
    StateVectorError {
        /// Error message
        message: String,
    },

    /// Generic kernel error
    #[error("Kernel error: {message}")]
    Generic {
        /// Error message
        message: String,
    },
}

impl KernelError {
    /// Check if this is a critical law violation
    pub fn is_law_violation(&self) -> bool {
        matches!(
            self,
            KernelError::ConservationViolation { .. }
                | KernelError::EntropyDecrease { .. }
                | KernelError::ForbiddenOperation { .. }
        )
    }

    /// Get severity level (0-10, higher = more severe)
    pub fn severity(&self) -> u8 {
        match self {
            KernelError::ConservationViolation { .. } => 10,
            KernelError::EntropyDecrease { .. } => 10,
            KernelError::ForbiddenOperation { .. } => 10,
            KernelError::EvolutionBlocked { .. } => 3,
            KernelError::InsufficientEnergy { .. } => 5,
            KernelError::UniverseNotFound { .. } => 4,
            KernelError::InteractionNotFound { .. } => 4,
            KernelError::InvalidCoupling { .. } => 6,
            KernelError::StateVectorError { .. } => 7,
            KernelError::Generic { .. } => 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_law_violation_detection() {
        let conservation = KernelError::ConservationViolation {
            expected: 100.0,
            actual: 99.0,
            delta: 1.0,
        };
        assert!(conservation.is_law_violation());
        assert_eq!(conservation.severity(), 10);

        let not_found = KernelError::UniverseNotFound {
            id: crate::types::UniverseID(1),
        };
        assert!(!not_found.is_law_violation());
        assert_eq!(not_found.severity(), 4);
    }
}

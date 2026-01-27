//! # ParadoxOS Kernel
//!
//! A physics-native operating system kernel where computation emerges from
//! universal laws rather than traditional scheduling.
//!
//! ## Core Principles
//!
//! - **No threads** - Use universes and interactions
//! - **No schedulers** - Evolution is emergent
//! - **No permissions** - Security through conservation laws
//! - **Physics-driven** - All behavior from 13 fundamental laws

// FIXME: Temporarily disabled due to compiler ICE
// #![warn(
//     missing_docs,
//     missing_debug_implementations,
//     rust_2018_idioms,
//     unreachable_pub
// )]
// #![deny(unsafe_code)]

pub mod types;
pub mod physics;
pub mod universe;
pub mod interaction;
pub mod compiler;
pub mod error;

// Re-export main types
pub use physics::{Kernel, Observer};
pub use types::{UniverseID, InteractionID, StateVector};
pub use universe::Universe;
pub use interaction::Interaction;
pub use error::{Result, KernelError};

/// ParadoxOS version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The 13 fundamental laws as constants
pub mod constants {
    /// Energy conservation tolerance (for floating point comparisons)
    pub const ENERGY_EPSILON: f64 = 1e-3;
    
    /// Default stability collapse threshold
    pub const STABILITY_THRESHOLD: f64 = 0.3;
    
    /// Default interaction decay rate
    pub const DEFAULT_DECAY_RATE: f64 = 0.01;
    
    /// Minimum entropy increase per evolution step
    pub const MIN_ENTROPY_DELTA: f64 = 0.0001;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}

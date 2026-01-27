//! Core type definitions for ParadoxOS

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a universe
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UniverseID(pub u64);

impl fmt::Display for UniverseID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U{}", self.0)
    }
}

/// Unique identifier for an interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InteractionID(pub u64);

impl fmt::Display for InteractionID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I{}", self.0)
    }
}

/// State vector - opaque computational state
///
/// This represents the computational state of a universe in compressed
/// (latent) form. The actual structure is implementation-defined and uses
/// ParadoxLF compression.
///
/// # LAW 8: Memory as Potential
///
/// Memory storage is a form of potential energy. Compressed data (ground state)
/// has lower potential energy than expanded data (excited state).
#[derive(Clone, Serialize, Deserialize)]
pub struct StateVector {
    /// Compressed data in ParadoxLF format
    pub(crate) data: Vec<u8>,
    /// Original uncompressed size
    pub(crate) original_size: usize,
    /// Is the data currently compressed?
    pub(crate) is_compressed: bool,
}

impl StateVector {
    /// Create a new state vector from raw bytes (automatically compresses)
    pub fn new(data: Vec<u8>) -> Self {
        Self::compress(&data)
    }

    /// Create an empty state vector
    pub fn empty() -> Self {
        Self { 
            data: Vec::new(),
            original_size: 0,
            is_compressed: false
        }
    }

    /// Get the size in bytes (compressed)
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Compress data using ParadoxLF
    ///
    /// This converts "kinetic" memory (raw) into "potential" memory (compressed).
    pub fn compress(data: &[u8]) -> Self {
        let compressed = paradoxlf::compress(data);
        Self {
            data: compressed,
            original_size: data.len(),
            is_compressed: true,
        }
    }

    /// Expand (decompress) the state vector
    ///
    /// This "excites" the memory, converting potential to kinetic energy.
    pub fn expand(&self) -> Vec<u8> {
        if !self.is_compressed {
            return self.data.clone();
        }
        
        paradoxlf::decompress(&self.data, Some(self.original_size))
            .unwrap_or_else(|_| Vec::new())
    }

    /// Calculate potential energy (LAW 8)
    ///
    /// E_potential = k * (uncompressed_size - compressed_size)
    ///
    /// Highly compressible data stores more potential energy.
    pub fn potential_energy(&self) -> f64 {
        if !self.is_compressed || self.original_size == 0 {
            return 0.0;
        }

        const MEMORY_ENERGY_CONSTANT: f64 = 0.001; // J per byte saved
        let saved_bytes = self.original_size.saturating_sub(self.data.len());
        
        saved_bytes as f64 * MEMORY_ENERGY_CONSTANT
    }
}

impl fmt::Debug for StateVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "StateVector({}B compressed, {}B original, E={:.4}J)", 
            self.data.len(), 
            self.original_size,
            self.potential_energy()
        )
    }
}

impl Default for StateVector {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_id() {
        let id = UniverseID(42);
        assert_eq!(format!("{}", id), "U42");
    }

    #[test]
    fn test_interaction_id() {
        let id = InteractionID(123);
        assert_eq!(format!("{}", id), "I123");
    }

    #[test]
    fn test_state_vector() {
        let data = vec![1, 2, 3, 4];
        let sv = StateVector::new(data.clone());
        
        assert!(!sv.is_empty());
        assert_eq!(sv.expand(), data);
    }

    #[test]
    fn test_state_vector_compress() {
        // Highly compressible data (1000 zeros)
        let data = vec![0u8; 1000];
        let sv = StateVector::compress(&data);
        
        assert!(sv.size() < data.len());
        assert_eq!(sv.expand(), data);
        
        // Should have potential energy
        assert!(sv.potential_energy() > 0.0);
    }
}

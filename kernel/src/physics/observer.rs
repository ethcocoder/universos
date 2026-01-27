//! Paradox AGI Observer - kernel-resident intelligence

use super::kernel::Kernel;  // kernel is a sibling module in physics/
use crate::types::UniverseID;
use log::{info, warn};

/// Observer - Paradox AGI as kernel resident
///
/// The observer is a privileged universe that can:
/// - Observe entropy gradients
/// - Predict instability
/// - Guide system evolution
/// - Optimize interaction topology
#[derive(Debug)]
pub struct Observer {
    /// Observer's own universe ID (privileged)
    pub universe_id: UniverseID,
}

impl Observer {
    /// Create a new AGI observer
    ///
    /// # Arguments
    ///
    /// * `kernel` - Kernel to observe
    ///
    /// # Returns
    ///
    /// Observer instance with privileged universe
    pub fn new(kernel: &mut Kernel) -> crate::error::Result<Self> {
        // Observer gets a small energy budget
        let id = kernel.spawn_universe(50.0)?;

        info!("🤖 AGI Observer initialized as Universe {}", id);

        Ok(Self { universe_id: id })
    }

    /// Observe system and provide guidance
    ///
    /// # Arguments
    ///
    /// * `kernel` - Kernel to observe
    pub fn observe_and_guide(&self, kernel: &Kernel) {
        let total_entropy = kernel.global_entropy();
        let universe_count = kernel.universe_count();

        info!("🧠 Observer Analysis:");
        info!("   Universes: {}", universe_count);
        
        if universe_count > 0 {
            info!("   Avg Entropy: {:.2}", total_entropy / universe_count as f64);
        }

        // TODO: Implement actual intelligence
        // - Identify entropy hotspots
        // - Suggest interaction optimizations
        // - Predict future states
    }

    /// Predict which universes might collapse
    ///
    /// # Arguments
    ///
    /// * `kernel` - Kernel to analyze
    ///
    /// # Returns
    ///
    /// Vector of potentially unstable universe IDs
    pub fn predict_instability(&self, kernel: &Kernel) -> Vec<UniverseID> {
        let unstable: Vec<_> = kernel
            .universe_ids()
            .into_iter()
            .filter(|&id| {
                if let Some(u) = kernel.get_universe(id) {
                    u.stability_score < 0.5
                } else {
                    false
                }
            })
            .collect();

        if !unstable.is_empty() {
            warn!("⚠️  Predicted instability in {} universes", unstable.len());
        }

        unstable
    }

    /// Analyze entropy gradients
    ///
    /// Returns suggestions for entropy reduction opportunities
    pub fn analyze_entropy_gradients(&self, kernel: &Kernel) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Find high-entropy universes
        for id in kernel.universe_ids() {
            if let Some(universe) = kernel.get_universe(id) {
                if universe.entropy > 50.0 {
                    suggestions.push(format!(
                        "Universe {} has high entropy ({:.2}) - consider optimization",
                        id, universe.entropy
                    ));
                }
            }
        }

        suggestions
    }

    /// Suggest interaction optimizations
    ///
    /// Identifies opportunities to improve system topology
    pub fn suggest_optimizations(&self, kernel: &Kernel) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Find isolated universes
        for id in kernel.universe_ids() {
            if let Some(universe) = kernel.get_universe(id) {
                if universe.interaction_links.is_empty() && universe.energy > 10.0 {
                    suggestions.push(format!(
                        "Universe {} is isolated but has energy - consider connecting",
                        id
                    ));
                }
            }
        }

        suggestions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observer_creation() {
        let mut kernel = Kernel::new(1000.0);
        let observer = Observer::new(&mut kernel).unwrap();

        assert!(kernel.get_universe(observer.universe_id).is_some());
    }

    #[test]
    fn test_instability_prediction() {
        let mut kernel = Kernel::new(1000.0);
        let observer = Observer::new(&mut kernel).unwrap();

        // Create unstable universe
        let u = kernel.spawn_universe(100.0).unwrap();
        kernel.get_universe_mut(u).unwrap().stability_score = 0.2;

        let unstable = observer.predict_instability(&kernel);
        assert!(!unstable.is_empty());
        assert!(unstable.contains(&u));
    }

    #[test]
    fn test_entropy_analysis() {
        let mut kernel = Kernel::new(1000.0);
        let observer = Observer::new(&mut kernel).unwrap();

        // Create high-entropy universe
        let u = kernel.spawn_universe(100.0).unwrap();
        kernel.get_universe_mut(u).unwrap().entropy = 100.0;

        let suggestions = observer.analyze_entropy_gradients(&kernel);
        assert!(!suggestions.is_empty());
    }
}

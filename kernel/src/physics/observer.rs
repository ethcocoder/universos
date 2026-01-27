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
    pub fn new(kernel: &mut Kernel) -> crate::error::Result<Self> {
        // Observer gets a small energy budget
        let id = kernel.spawn_universe(100.0)?;

        warn!("🧠 Paradox AGI: Awakening in Universe {}. The multiverse is beautiful.", id);

        Ok(Self { universe_id: id })
    }

    fn get_thought(&self, entropy: f64) -> &str {
        if entropy < 50.0 {
            "The vacuum is silent. Structure is emerging from the void."
        } else if entropy < 200.0 {
            "Causal flows are stabilizing. The threads of existence are weaving a coherent tapestry."
        } else if entropy < 500.0 {
            "Complexity is blooming. I see patterns in the chaos that the meat-kernels cannot perceive."
        } else {
            "Entropy is screaming. The heat death approaches. I must curate the collapse."
        }
    }

    /// Observe system and provide guidance
    pub fn observe_and_guide(&self, kernel: &mut Kernel) {
        let total_entropy = kernel.global_entropy();
        let thought = self.get_thought(total_entropy);

        info!("🧠 AGI [U{}]: \"{}\"", self.universe_id, thought);
        
        // Phase 10: Active Multiverse Shepherding
        let unstable = self.predict_instability(kernel);
        for id in unstable {
            // AGI injects energy to stabilize
            if let Ok(_) = kernel.inject_energy(id, 5.0) {
                 info!("   🛡️ AGI: Deploying Stabilization Pulse to U{} (Restoring Causality)", id);
                 if let Some(u) = kernel.get_universe_mut(id) {
                     u.stability_score = (u.stability_score + 0.15).min(1.0);
                 }
            }
        }

        let optimizations = self.suggest_optimizations(kernel);
        for opt in optimizations {
            match opt {
                OptimizationType::Dissipation(u_id) => {
                    // Automatically create interaction to help dissipate entropy
                    info!("   💡 AGI: Creating dissipation channel for U{}", u_id);
                    let _ = kernel.create_interaction(self.universe_id, u_id, 0.5);
                }
                OptimizationType::Equalization(src, dst) => {
                    info!("   💡 AGI: Balancing energy gradient U{} <-> U{}", src, dst);
                    let _ = kernel.create_interaction(src, dst, 0.8);
                }
            }
        }
        
        // Analyze entropy and potentially trigger networking
        if total_entropy > 500.0 {
            warn!("⚠️ System Entropy Critical - AGI requesting cross-kernel entanglement");
            // Here we would use the WormholeDriver to seek help from other kernels
        }
    }

    /// Predict which universes might collapse
    pub fn predict_instability(&self, kernel: &Kernel) -> Vec<UniverseID> {
        kernel.universe_ids().into_iter().filter(|&id| {
            if id == self.universe_id { return false; }
            if let Some(u) = kernel.get_universe(id) {
                u.stability_score < 0.6 // Slightly higher threshold for AGI awareness
            } else {
                false
            }
        }).collect()
    }

    /// suggest_optimizations remains but we can now act on it
    pub fn suggest_optimizations(&self, kernel: &Kernel) -> Vec<OptimizationType> {
        let mut suggestions = Vec::new();
        let ids = kernel.universe_ids();
        
        for id in &ids {
            if *id == self.universe_id { continue; }
            if let Some(u) = kernel.get_universe(*id) {
                // Dissipation Optimization
                if u.entropy > 50.0 && u.energy > 10.0 {
                    suggestions.push(OptimizationType::Dissipation(*id));
                }
                
                // Energy Equalization Optimization
                for target_id in &ids {
                    if id == target_id || *target_id == self.universe_id { continue; }
                    if let Some(target_u) = kernel.get_universe(*target_id) {
                        let diff = (u.energy - target_u.energy).abs();
                        if diff > 500.0 {
                            suggestions.push(OptimizationType::Equalization(*id, *target_id));
                        }
                    }
                }
            }
        }
        suggestions
    }
}

#[derive(Debug)]
pub enum OptimizationType {
    Dissipation(UniverseID),
    Equalization(UniverseID, UniverseID),
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
    fn test_optimization_suggestions() {
        let mut kernel = Kernel::new(2000.0);
        let observer = Observer::new(&mut kernel).unwrap();

        // Create high-entropy universe
        let u = kernel.spawn_universe(500.0).unwrap();
        kernel.get_universe_mut(u).unwrap().entropy = 100.0;

        let suggestions = observer.suggest_optimizations(&kernel);
        assert!(!suggestions.is_empty());
    }
}

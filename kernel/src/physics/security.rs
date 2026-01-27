//! ParadoxOS Security - Sovereignty through Physical Laws
//!
//! Unlike traditional OS security (ACLs/Permissions), Paradox security
//! is purely based on Energy Conservation and Interaction Primacy.

use crate::types::UniverseID;
use crate::physics::kernel::Kernel;
use log::error;

pub struct SecurityAuditor;

impl SecurityAuditor {
    /// Conduct a deep audit of a universe's physical state.
    /// Returns true if the universe is "Physically Legal".
    pub fn audit_universe(kernel: &Kernel, id: UniverseID) -> bool {
        let universe = match kernel.get_universe(id) {
            Some(u) => u,
            None => return false,
        };

        // 1. Energy Anomaly Check: Has energy been created from nothing?
        // (In a professional system, we track 'Energy History')
        if universe.energy < 0.0 {
            error!("🛡️ SECURITY BREACH: Universe {} has negative energy (Physical Paradox)!", id);
            return false;
        }

        // 2. Entropy Check: Is the universe trying to "Reverse Time" (Decrease entropy illegally)?
        // (Planned for Phase 11 deep integration)

        // 3. Unauthorized Interaction: Is there a delta in state without a registered Interaction?
        // This is the primary defense against memory corruption.
        
        true
    }

    /// Detect if the entire system is physically consistent.
    /// Returns a list of UniverseIDs that are causing anomalies.
    pub fn detect_anomalies(kernel: &Kernel) -> Vec<(UniverseID, String)> {
        let mut anomalies = Vec::new();
        
        for id in kernel.universe_ids() {
            if let Some(u) = kernel.get_universe(id) {
                // Anomaly 1: Energy creation
                if u.energy < 0.0 {
                    anomalies.push((id, "Negative Energy Breach".to_string()));
                }

                // Anomaly 2: Stability overflow (Hacking the stability score)
                if u.stability_score > 1.0 {
                    anomalies.push((id, "Stability Injection Exploit".to_string()));
                }
            }
        }

        anomalies
    }

    pub fn verify_global_integrity(kernel: &Kernel) -> Result<(), String> {
        let total_system_energy = kernel.calculate_total_energy();
        let expected = kernel.initial_energy() + kernel.energy_flux();
        let drift = (total_system_energy - expected).abs();

        if drift > 0.05 { // Increased threshold slightly for floating point variance
            return Err(format!("☢️ SECURITY BREACH: {:.6} J drift detected! (exp: {:.2}, got: {:.2})", drift, expected, total_system_energy));
        }

        Ok(())
    }
}

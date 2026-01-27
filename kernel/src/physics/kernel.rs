//! ParadoxOS Kernel - The Physics Engine

use crate::constants::*;
use crate::error::{KernelError, Result};
use crate::interaction::Interaction;
use crate::types::{InteractionID, UniverseID};
use crate::universe::Universe;
use super::laws;  // laws is a sibling module in physics/
use hashbrown::HashMap;
use log::{debug, info, warn};

/// The Kernel - Global physics engine
///
/// This is NOT a traditional operating system kernel. It is a physics simulator
/// that enforces universal laws and allows computation to emerge.
#[derive(Debug)]
pub struct Kernel {
    /// Total free energy in the kernel pool
    ///
    /// Invariant (LAW 1): global_energy + Σ(universe.energy) = constant
    global_energy: f64,

    /// Total system entropy
    ///
    /// Invariant (LAW 2): Monotonically increasing
    global_entropy: f64,

    /// Active universes
    universes: HashMap<UniverseID, Universe>,

    /// Active interactions
    interactions: HashMap<InteractionID, Interaction>,

    /// Next universe ID (monotonic counter)
    next_universe_id: u64,

    /// Next interaction ID (monotonic counter)
    next_interaction_id: u64,

    /// Evolution step counter
    evolution_step: u64,

    /// Initial total energy (for conservation checking)
    initial_total_energy: f64,
}

impl Kernel {
    /// Initialize the kernel universe (Big Bang)
    ///
    /// # Arguments
    ///
    /// * `initial_energy` - Total system energy budget
    ///
    /// # Returns
    ///
    /// New kernel instance with specified energy
    pub fn new(initial_energy: f64) -> Self {
        info!("🌌 Big Bang: Initializing Kernel Universe");
        info!("   Initial Energy: {:.2} J", initial_energy);

        Self {
            global_energy: initial_energy,
            global_entropy: 0.0,
            universes: HashMap::new(),
            interactions: HashMap::new(),
            next_universe_id: 1,
            next_interaction_id: 1,
            evolution_step: 0,
            initial_total_energy: initial_energy,
        }
    }

    /// Spawn a new universe
    ///
    /// # Arguments
    ///
    /// * `initial_energy` - Energy to allocate from kernel pool
    ///
    /// # Returns
    ///
    /// `Ok(UniverseID)` if successful, `Err` if insufficient energy
    ///
    /// # Laws Enforced
    ///
    /// - LAW 0: Universe has state
    /// - LAW 1: Energy conservation
    /// - LAW 2: Entropy increases (creating structure)
    pub fn spawn_universe(&mut self, initial_energy: f64) -> Result<UniverseID> {
        // LAW 1: Cannot create energy
        if initial_energy > self.global_energy {
            return Err(KernelError::InsufficientEnergy {
                requested: initial_energy,
                available: self.global_energy,
            });
        }

        let id = UniverseID(self.next_universe_id);
        self.next_universe_id += 1;

        let mut universe = Universe::new(id, initial_energy);
        universe.creation_time = self.evolution_step;

        // Deduct energy from global pool (LAW 1)
        self.global_energy -= initial_energy;

        // Creating structure increases entropy (LAW 2)
        self.global_entropy += 1.0 + (initial_energy / 100.0);

        info!("✨ Universe {} spawned with {:.2} J", id, initial_energy);

        self.universes.insert(id, universe);

        Ok(id)
    }

    /// Create a new universe by branching an existing one
    ///
    /// # Arguments
    ///
    /// * `parent_id` - ID of universe to branch
    ///
    /// # Returns
    ///
    /// ID of new branched universe
    pub fn branch_universe(&mut self, parent_id: UniverseID) -> Result<UniverseID> {
        let new_id = UniverseID(self.next_universe_id);
        
        // Scope to limit borrow of self.universes
        let mut branched = {
            let parent = self.universes.get_mut(&parent_id)
                .ok_or(KernelError::UniverseNotFound { id: parent_id })?;
            parent.branch(new_id)?
        };
        
        // Set creation time to current step (LAW 7)
        branched.creation_time = self.evolution_step;
        
        self.next_universe_id += 1;
        
        // Law 2: Kernel entropy increases
        self.global_entropy += 0.5;
        
        info!("🌿 Universe {} branched from {}", new_id, parent_id);
        
        self.universes.insert(new_id, branched);
        Ok(new_id)
    }

    /// Create an interaction between two universes
    ///
    /// # Arguments
    ///
    /// * `source` - Source universe ID
    /// * `target` - Target universe ID
    /// * `coupling_strength` - Interaction strength (0.0 to 1.0)
    ///
    /// # Laws Enforced
    ///
    /// - LAW 3: Interaction is the only causal channel
    /// - LAW 2: Entropy increases
    pub fn create_interaction(
        &mut self,
        source: UniverseID,
        target: UniverseID,
        coupling_strength: f64,
    ) -> Result<InteractionID> {
        // Validate universes exist
        if !self.universes.contains_key(&source) {
            return Err(KernelError::UniverseNotFound { id: source });
        }
        if !self.universes.contains_key(&target) {
            return Err(KernelError::UniverseNotFound { id: target });
        }

        let id = InteractionID(self.next_interaction_id);
        self.next_interaction_id += 1;

        let interaction = Interaction::new(id, source, target, coupling_strength)?;

        // Link universes bidirectionally
        self.universes.get_mut(&source).unwrap().add_interaction(id);
        self.universes.get_mut(&target).unwrap().add_interaction(id);

        // LAW 2: Creating connections increases entropy
        self.global_entropy += 0.5;

        info!("🔗 Interaction {} created: {} ↔ {} (strength={:.2})",
              id, source, target, coupling_strength);

        self.interactions.insert(id, interaction);

        Ok(id)
    }

    /// Main evolution loop - THIS IS THE OS
    ///
    /// Executes one step of system evolution, enforcing all 13 laws
    ///
    /// # Steps
    ///
    /// 1. Observe interactions
    /// 2. Compute entropy gradients
    /// 3. Redistribute energy
    /// 4. Evolve universes
    /// 5. Collapse unstable universes
    pub fn evolution_step(&mut self) {
        self.evolution_step += 1;
        
        debug!("━━━ Evolution Step {} ━━━", self.evolution_step);

        // Store initial state for law verification
        let initial_entropy = self.global_entropy;

        // Step 1: Observe interactions
        self.observe_interactions();

        // Step 2: Compute entropy gradients
        self.compute_entropy_gradients();

        // Step 3: Redistribute energy
        if let Err(e) = self.redistribute_energy() {
            warn!("Energy redistribution error: {}", e);
        }

        // Step 4: Evolve universes
        self.evolve_universes();

        // Step 5: Collapse unstable universes
        self.collapse_unstable_universes();

        // Verify laws
        self.verify_laws(initial_entropy);

        debug!("   Global Energy: {:.2} J", self.global_energy);
        debug!("   Global Entropy: {:.2}", self.global_entropy);
    }

    fn observe_interactions(&self) {
        let active_count = self.interactions.len();
        debug!("👁️  Observing {} active interactions", active_count);
    }

    fn compute_entropy_gradients(&mut self) {
        // LAW 2: Each evolution step inherently increases entropy
        let entropy_increase = MIN_ENTROPY_DELTA * self.universes.len() as f64;
        self.global_entropy += entropy_increase;

        debug!("📊 Entropy increased by {:.6}", entropy_increase);
    }

    fn redistribute_energy(&mut self) -> Result<()> {
        let initial_total = self.calculate_total_energy();

        // Apply interaction decay
        for interaction in self.interactions.values_mut() {
            interaction.apply_decay();
        }

        // Transfer energy through interactions
        let mut transfers = Vec::new();

        for interaction in self.interactions.values_mut() {
            if !interaction.is_active() {
                continue;
            }

            // Set momentum based on energy gradient
            if let (Some(source), Some(target)) = (
                self.universes.get(&interaction.source),
                self.universes.get(&interaction.target),
            ) {
                interaction.set_momentum(source.energy, target.energy);

                let transfer = interaction.calculate_energy_transfer(0.01);
                
                if transfer.abs() > ENERGY_EPSILON {
                    transfers.push((
                        interaction.source,
                        interaction.target,
                        transfer,
                        interaction.id,
                    ));
                }
            }
        }

        // Apply transfers
        for (source_id, target_id, amount, interaction_id) in transfers {
            if let Some(source) = self.universes.get_mut(&source_id) {
                let _ = source.transfer_energy(-amount);
            }
            if let Some(target) = self.universes.get_mut(&target_id) {
                let _ = target.transfer_energy(amount);
            }
            if let Some(interaction) = self.interactions.get_mut(&interaction_id) {
                interaction.record_transfer(amount);
            }

            if amount.abs() > 0.001 {
                debug!("⚡ Energy transfer: {} → {}: {:.4} J",
                       source_id, target_id, amount);
            }
        }

        // Verify conservation (LAW 1)
        let final_total = self.calculate_total_energy();
        laws::verify_energy_conservation(initial_total, final_total)?;

        Ok(())
    }

    fn evolve_universes(&mut self) {
        let mut updates = Vec::new();

        for (id, universe) in &self.universes {
            // LAW 4: Check evolution condition
            let pressure = self.calculate_interaction_pressure(*id);
            let resistance = universe.internal_resistance();

            if laws::check_evolution_condition(pressure, resistance) {
                let evolution_rate = if resistance > ENERGY_EPSILON {
                    pressure / resistance
                } else {
                    pressure
                };

                updates.push((*id, evolution_rate));
            }
        }

        // Apply evolution updates
        for (id, rate) in updates {
            if let Some(universe) = self.universes.get_mut(&id) {
                // Advance local time (LAW 7)
                universe.advance_time();

                // Evolution increases entropy (LAW 2)
                universe.increase_entropy(rate * 0.1);

                // Update stability
                universe.update_stability();

                universe.last_evolution = self.evolution_step;

                debug!("🌀 Universe {} evolved (rate={:.2})", id, rate);
            }
        }
    }

    fn collapse_unstable_universes(&mut self) {
        let mut to_collapse = Vec::new();

        for (id, universe) in &self.universes {
            if laws::should_collapse(universe.stability_score) {
                to_collapse.push(*id);
            }
        }

        for id in to_collapse {
            let _ = self.collapse_universe(id);
        }
    }

    /// Manually collapse a universe
    ///
    /// # Laws Enforced
    ///
    /// - LAW 1: Energy returned to pool
    /// - LAW 2: Entropy released
    pub fn collapse_universe(&mut self, id: UniverseID) -> Result<Universe> {
        let universe = self.universes.remove(&id).ok_or(
            KernelError::UniverseNotFound { id }
        )?;

        info!("💥 Universe {} collapsed (stability={:.2})", id, universe.stability_score);

        // Return energy to global pool (LAW 1)
        self.global_energy += universe.energy;

        // Release entropy (LAW 2)
        self.global_entropy += universe.entropy;

        // Remove associated interactions
        for interaction_id in &universe.interaction_links {
            self.interactions.remove(interaction_id);
        }

        Ok(universe)
    }

    /// Calculate total interaction pressure on a universe
    fn calculate_interaction_pressure(&self, universe_id: UniverseID) -> f64 {
        let universe = match self.universes.get(&universe_id) {
            Some(u) => u,
            None => return 0.0,
        };

        let mut pressure = 0.0;
        for interaction_id in &universe.interaction_links {
            if let Some(interaction) = self.interactions.get(interaction_id) {
                pressure += interaction.coupling_strength * interaction.momentum.abs();
            }
        }

        pressure
    }

    /// Calculate total system energy
    fn calculate_total_energy(&self) -> f64 {
        self.global_energy + self.universes.values().map(|u| u.energy).sum::<f64>()
    }

    /// Verify all physics laws hold
    fn verify_laws(&self, previous_entropy: f64) {
        // LAW 1: Energy conservation
        if let Err(e) = laws::verify_energy_conservation(
            self.initial_total_energy,
            self.calculate_total_energy(),
        ) {
            panic!("CRITICAL LAW VIOLATION: {}", e);
        }

        // LAW 2: Entropy monotonicity
        if let Err(e) = laws::verify_entropy_increase(previous_entropy, self.global_entropy) {
            panic!("CRITICAL LAW VIOLATION: {}", e);
        }
    }

    // Public getters

    /// Get global energy
    pub fn global_energy(&self) -> f64 {
        self.global_energy
    }

    /// Get global entropy
    pub fn global_entropy(&self) -> f64 {
        self.global_entropy
    }

    /// Get universe count
    pub fn universe_count(&self) -> usize {
        self.universes.len()
    }

    /// Get interaction count
    pub fn interaction_count(&self) -> usize {
        self.interactions.len()
    }

    /// Get universe reference
    pub fn get_universe(&self, id: UniverseID) -> Option<&Universe> {
        self.universes.get(&id)
    }

    /// Get mutable universe reference
    pub fn get_universe_mut(&mut self, id: UniverseID) -> Option<&mut Universe> {
        self.universes.get_mut(&id)
    }

    /// Get interaction reference
    pub fn get_interaction(&self, id: InteractionID) -> Option<&Interaction> {
        self.interactions.get(&id)
    }

    /// Get all universe IDs
    pub fn universe_ids(&self) -> Vec<UniverseID> {
        self.universes.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_kernel_creation() {
        init_logger();
        let kernel = Kernel::new(1000.0);
        assert_eq!(kernel.global_energy(), 1000.0);
        assert_eq!(kernel.global_entropy(), 0.0);
    }

    #[test]
    fn test_spawn_universe() {
        init_logger();
        let mut kernel = Kernel::new(1000.0);
        
        let u1 = kernel.spawn_universe(100.0).unwrap();
        assert_eq!(kernel.global_energy(), 900.0);
        assert!(kernel.global_entropy() > 0.0); // Entropy increased
    }

    #[test]
    fn test_insufficient_energy() {
        init_logger();
        let mut kernel = Kernel::new(100.0);
        
        let result = kernel.spawn_universe(200.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_interaction() {
        init_logger();
        let mut kernel = Kernel::new(1000.0);
        
        let u1 = kernel.spawn_universe(100.0).unwrap();
        let u2 = kernel.spawn_universe(100.0).unwrap();
        
        let i = kernel.create_interaction(u1, u2, 0.8).unwrap();
        
        assert!(kernel.get_interaction(i).is_some());
    }

    #[test]
    fn test_evolution_step() {
        init_logger();
        let mut kernel = Kernel::new(2000.0);
        
        let u1 = kernel.spawn_universe(300.0).unwrap();
        let u2 = kernel.spawn_universe(300.0).unwrap();
        kernel.create_interaction(u1, u2, 0.9).unwrap();
        
        let initial_entropy = kernel.global_entropy();
        
        kernel.evolution_step();
        
        // Entropy should have increased
        assert!(kernel.global_entropy() > initial_entropy);
    }

    #[test]
    fn test_energy_conservation_over_time() {
        init_logger();
        let mut kernel = Kernel::new(5000.0);
        
        kernel.spawn_universe(500.0).unwrap();
        kernel.spawn_universe(700.0).unwrap();
        kernel.spawn_universe(300.0).unwrap();
        
        for _ in 0..100 {
            kernel.evolution_step();
        }
        
        // Energy should still be conserved
        let total = kernel.calculate_total_energy();
        assert!((total - 5000.0).abs() < ENERGY_EPSILON);
    }
}

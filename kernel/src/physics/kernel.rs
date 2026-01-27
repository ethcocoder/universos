//! ParadoxOS Kernel - The Physics Engine

use crate::constants::*;
use crate::error::{KernelError, Result};
use crate::interaction::Interaction;
use crate::types::{InteractionID, UniverseID};
use crate::universe::Universe;
use super::laws;  // laws is a sibling module in physics/
use super::security;
use hashbrown::HashMap;
use log::{debug, info, warn};
use std::collections::VecDeque;

/// A snapshot of the kernel state for time reversal (Phase 13)
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct KernelSnapshot {
    pub global_energy: f64,
    pub global_entropy: f64,
    pub universes: HashMap<crate::types::UniverseID, crate::universe::Universe>,
    pub interactions: HashMap<crate::types::InteractionID, Interaction>,
    pub evolution_step: u64,
    pub energy_radiated: f64,
    pub energy_materialized: f64,
}

/// The Kernel - Global physics engine
///
/// This is NOT a traditional operating system kernel. It is a physics simulator
/// that enforces universal laws and allows computation to emerge.
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

    /// Spatial indexing for interactions
    interaction_field: crate::interaction::InteractionField,

    /// Registered Hardware Drivers (HAL)
    drivers: Vec<Box<dyn super::drivers::HardwareDriver>>,
    
    /// Next event ID
    next_event_id: u64,

    /// Rolling history for time reversal (Phase 13)
    history: VecDeque<KernelSnapshot>,

    /// Multiversal Accounting: Energy entering/leaving this kernel node
    energy_radiated: f64,
    energy_materialized: f64,

    /// Gravity-Based Scheduler (Phase 18)
    scheduler: super::scheduler::GravityScheduler,
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
            interaction_field: crate::interaction::InteractionField::new(),
            drivers: Vec::new(),
            next_event_id: 1,
            history: VecDeque::with_capacity(100),
            energy_radiated: 0.0,
            energy_materialized: 0.0,
            scheduler: super::scheduler::GravityScheduler::new(),
        }
    }

    /// Add a hardware driver to the system
    pub fn add_driver(&mut self, driver: Box<dyn super::drivers::HardwareDriver>) {
        self.drivers.push(driver);
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

    pub fn inject_energy(&mut self, target_id: UniverseID, amount: f64) -> Result<()> {
        if amount > self.global_energy {
            return Err(KernelError::InsufficientEnergy { requested: amount, available: self.global_energy });
        }
        let universe = self.universes.get_mut(&target_id)
            .ok_or(KernelError::UniverseNotFound { id: target_id })?;
        
        universe.energy += amount;
        self.global_energy -= amount;
        Ok(())
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

    /// Add an interaction between two universes
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
        
        // Register in field
        self.interaction_field.register_interaction(id, source, target);

        // LAW 2: Creating connections increases entropy
        self.global_entropy += 0.5;

        info!("🔗 Interaction {} created: {} ↔ {} (strength={:.2})",
              id, source, target, coupling_strength);

        self.interactions.insert(id, interaction);

        Ok(id)
    }

    /// Spawn a causal event (signal/energy transfer) between universes
    ///
    /// # Arguments
    ///
    /// * `source` - Origin universe
    /// * `target` - Destination universe
    /// * `event_type` - Type of event
    /// * `data` - Information payload
    /// * `energy` - Energy to transfer (deducted from source)
    ///
    /// # Laws Enforced
    ///
    /// - LAW 1: Energy is conserved (deducted from source immediately)
    /// - LAW 3: Must move through an interaction channel or Hardware Driver
    pub fn spawn_event(
        &mut self,
        source: UniverseID,
        target: UniverseID,
        event_type: crate::interaction::EventType,
        data: Vec<u8>,
        energy: f64,
    ) -> Result<crate::interaction::EventID> {
        // LAW 1: Deduct energy from source universe
        if let Some(source_u) = self.universes.get_mut(&source) {
            source_u.transfer_energy(-energy)?;
        }

        let id = crate::interaction::EventID(self.next_event_id);
        self.next_event_id += 1;

        let event = crate::interaction::CausalEvent::new(
            id,
            event_type,
            source,
            target,
            energy,
            crate::types::StateVector::from_raw(data),
            self.evolution_step,
        );
        
        self.route_event(event)?;
        Ok(id)
    }

    /// Load a program (Universal Bytecode) into a universe
    pub fn load_program(&mut self, universe_id: UniverseID, code: Vec<u8>) -> Result<()> {
        let universe = self.universes.get_mut(&universe_id)
            .ok_or(KernelError::UniverseNotFound { id: universe_id })?;
            
        // Overwrite state vector with raw executable code
        universe.state_vector = crate::types::StateVector::new_raw(code);
        universe.instruction_pointer = 0;
        
        info!("💾 Program loaded into {:?}", universe_id);
        Ok(())
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
    pub fn evolution_step(&mut self) -> super::drivers::SystemPulse {
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

        // Step 3.5: Propagate causal events (interaction primacy)
        self.propagate_events();

        // Step 4: Evolve universes
        self.evolve_universes();

        // Step 5: Collapse unstable universes
        self.collapse_unstable_universes();

        // Capture snapshot before hardware interactions (Phase 13)
        self.capture_snapshot();

        // Step 6: Synchronize Hardware Drivers (HAL)
        let mut incoming_events = Vec::new();
        let pulse = self.sync_drivers(&mut incoming_events);

        // Process incoming network events (materialization)
        for event in incoming_events {
            self.energy_materialized += event.energy_payload;
            let _ = self.route_event(event);
        }

        // Step 7: Physics-Based Security Audit (Phase 11)
        let anomalies = security::SecurityAuditor::detect_anomalies(self);
        for (id, reason) in anomalies {
            warn!("🛡️ SECURITY BLOCK: Anomalous activity in U{} ({})", id, reason);
            let _ = self.collapse_universe(id);
        }

        if let Err(e) = security::SecurityAuditor::verify_global_integrity(self) {
             warn!("🛡️ GLOBAL SECURITY ALERT: {}", e);
        }

        // Verify laws
        self.verify_laws(initial_entropy);

        debug!("   Global Energy: {:.2} J", self.global_energy);
        debug!("   Global Entropy: {:.2}", self.global_entropy);

        pulse
    }

    fn capture_snapshot(&mut self) {
        let snapshot = KernelSnapshot {
            global_energy: self.global_energy,
            global_entropy: self.global_entropy,
            universes: self.universes.clone(),
            interactions: self.interactions.clone(),
            evolution_step: self.evolution_step,
            energy_radiated: self.energy_radiated,
            energy_materialized: self.energy_materialized,
        };
        
        self.history.push_back(snapshot);
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Rewind the kernel state by a certain number of steps
    pub fn rewind(&mut self, steps: usize) -> bool {
        if self.history.is_empty() {
            return false;
        }

        let target_index = self.history.len().saturating_sub(steps.max(1));
        if let Some(snapshot) = self.history.get(target_index).cloned() {
            info!("⏳ CHRONOS: Rewinding multiverse to step {}", snapshot.evolution_step);
            
            self.global_energy = snapshot.global_energy;
            self.global_entropy = snapshot.global_entropy;
            self.universes = snapshot.universes;
            self.interactions = snapshot.interactions;
            self.evolution_step = snapshot.evolution_step;
            self.energy_radiated = snapshot.energy_radiated;
            self.energy_materialized = snapshot.energy_materialized;
            
            // Truncate history forward
            self.history.truncate(target_index);
            
            true
        } else {
            false
        }
    }

    /// Synchronize all registered hardware drivers
    fn sync_drivers(&mut self, incoming_events: &mut Vec<crate::interaction::CausalEvent>) -> super::drivers::SystemPulse {
        let mut combined_pulse = super::drivers::SystemPulse::None;
        for driver in &mut self.drivers {
            match driver.sync(&self.universes, incoming_events) {
                Ok(pulse) => {
                    if pulse != super::drivers::SystemPulse::None {
                        combined_pulse = pulse;
                    }
                }
                Err(e) => warn!("Driver '{}' sync error: {}", driver.name(), e),
            }
        }
        combined_pulse
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

    /// Propagate causal events through the interaction network
    fn propagate_events(&mut self) {
        let mut delivered = Vec::new();

        // 1. Process interactions
        for interaction in self.interactions.values_mut() {
            let arrived = interaction.process_events();
            delivered.extend(arrived);
        }

        if !delivered.is_empty() {
             debug!("⚡ Propagating {} causal events", delivered.len());
        }

        // 2. Deliver events to universes
        for event in delivered {
            if let Some(target) = self.universes.get_mut(&event.target) {
                // Apply energy payload (LAW 1)
                target.energy += event.energy_payload;
                
                // Log event
                info!("📬 Event {} ({:?}) delivered to {} (Data: {} bytes, E={:.2}J)", 
                      event.id, event.event_type, event.target, 
                      event.data.size(), event.energy_payload);
                      
                // In a full implementation, `target.handle_event(event)` would be called here
                // to update internal state (LAW 0).
                // For now, energy conservation is the primary effect.
            }
        }
    }

    fn evolve_universes(&mut self) {
        // Phase 18: Gravity-Based Scheduling
        // First, calculate all interaction pressures (The 'Why' for evolution)
        let mut pressures = HashMap::with_capacity(self.universes.len());
        for id in self.universes.keys() {
            pressures.insert(*id, self.calculate_interaction_pressure(*id));
        }

        // Prioritize universes by physical 'fit' (Stability / Entropy) and Pressure
        self.scheduler.schedule(&self.universes, &pressures);
        
        // Take the top N universes for this tick
        let updates = self.scheduler.next_tasks(self.universes.len());

        let mut generated_events = Vec::new();

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

                // Phase 5: Execution
                let (event, execution_cost) = universe.execute_step();
                
                // Add execution heat to global energy (Law 1: Energy Conservation)
                // The cost was deducted from the universe, so it goes to the global pool
                self.global_energy += execution_cost;
                
                if let Some(e) = event {
                    generated_events.push(e);
                }

                debug!("🌀 Universe {} evolved (rate={:.2})", id, rate);
            }
        }

        // Route generated events
        for event in generated_events {
            if let Err(e) = self.route_event(event) {
                warn!("Failed to route execution event: {}", e);
            }
        }
    }

    /// Route an event generated by execution to the appropriate interaction
    fn route_event(&mut self, event: crate::interaction::CausalEvent) -> Result<()> {
        // Phase 15: Handle Quantum Instruction Set Events (System-Level)
        match event.event_type {
            crate::interaction::EventType::Entangle => {
                let strength = event.data.raw()[0] as f64 / 255.0;
                let _ = self.create_interaction(event.source, event.target, strength);
                self.global_energy += event.energy_payload; // Recycle to system pool
                return Ok(());
            }
            crate::interaction::EventType::Observation => {
                // Synchronous metadata query
                if let Some(target) = self.universes.get(&event.target) {
                    let meta_type = event.data.raw()[0];
                    let dest_addr = event.data.raw()[1] as usize;
                    let val = match meta_type {
                        0 => (target.energy / 10.0) as u8,
                        1 => (target.entropy / 10.0) as u8,
                        2 => (target.stability_score * 255.0) as u8,
                        _ => 0,
                    };
                    if let Some(source) = self.universes.get_mut(&event.source) {
                        if dest_addr < source.state_vector.raw().len() {
                             source.state_vector.raw_mut()[dest_addr] = val;
                        }
                    }
                }
                self.global_energy += event.energy_payload; // Recycle to system pool
                return Ok(());
            }
            crate::interaction::EventType::Reversion => {
                let steps = event.data.raw()[0] as usize;
                self.rewind(steps);
                self.global_energy += event.energy_payload; // Recycle to system pool
                return Ok(());
            }
            crate::interaction::EventType::Branch => {
                let energy = event.energy_payload;
                let dest_addr = event.data.raw()[0] as usize;
                if let Ok(new_id) = self.branch_universe(event.source) {
                    // Inject initial energy if available
                    if energy > 0.0 {
                        let _ = self.inject_energy(new_id, energy);
                    }
                    if let Some(source) = self.universes.get_mut(&event.source) {
                        if dest_addr < source.state_vector.raw().len() {
                             source.state_vector.raw_mut()[dest_addr] = new_id.0 as u8;
                        }
                    }
                } else {
                    // Branching failed (likely low energy), return payload to global
                    self.global_energy += energy;
                }
                return Ok(());
            }
            _ => {}
        }

        // Step 1: Check if target is local
        if self.universes.contains_key(&event.target) {
            // Find local interaction ID
            let interaction_id = self.interactions.values()
                .find(|i| (i.source == event.source && i.target == event.target) || 
                        (i.source == event.target && i.target == event.source))
                .map(|i| i.id);

            if let Some(id) = interaction_id {
                // Push to interaction
                if let Some(interaction) = self.interactions.get_mut(&id) {
                    interaction.push_event(event)?;
                    info!("⚡ Routed signal via Interaction {}", id);
                }
            } else {
                // Local target but no interaction? This is a "Spontaneous Entanglement" (Phase 12)
                warn!("⚠️ Spontaneous Entanglement: U{} -> U{} without interaction", event.source, event.target);
            }
        } else {
            // Step 2: Target is remote. Hand over to Hardware Drivers (Wormholes)
            info!("🛰️ Projecting signal U{} -> U{} to remote multiverse", event.source, event.target);
            self.energy_radiated += event.energy_payload;
            for driver in &mut self.drivers {
                let _ = driver.handle_event(&event);
            }
        }
        
        Ok(())
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
        // Clamp to 0 to prevent "Energy Sucking" attacks (Phase 11)
        self.global_energy += universe.energy.max(0.0);

        // Release entropy (LAW 2)
        self.global_entropy += universe.entropy;

        // Remove associated interactions
        for interaction_id in &universe.interaction_links {
            self.interactions.remove(interaction_id);
        }

        Ok(universe)
    }

    /// Sabotage a universe (Phase 14 Stress Testing ONLY)
    pub fn sabotage_universe(&mut self, id: UniverseID, energy_drain: f64) -> Result<()> {
        let universe = self.universes.get_mut(&id)
            .ok_or(KernelError::UniverseNotFound { id })?;
        
        // Siphon energy to global pool (LAW 1)
        let actual_drain = energy_drain.min(universe.energy);
        universe.energy -= actual_drain;
        self.global_energy += actual_drain;

        // Damage stability
        universe.stability_score = (universe.stability_score - 0.2).max(0.0);
        
        warn!("🐒 SABOTAGE: U{} energy drained by {:.2}J and stability corrupted", id, actual_drain);
        Ok(())
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

    pub fn calculate_total_energy(&self) -> f64 {
        let universe_energy: f64 = self.universes.values().map(|u| u.energy).sum();
        
        // Include energy in transit (in event queues)
        let transit_energy: f64 = self.interactions.values()
            .map(|i| i.pending_energy())
            .sum();
            
        self.global_energy + universe_energy + transit_energy
    }

    pub fn initial_energy(&self) -> f64 {
        self.initial_total_energy
    }

    pub fn energy_flux(&self) -> f64 {
        self.energy_materialized - self.energy_radiated
    }
    
    /// Verify all physics laws hold (Phase 11/12/13)
    fn verify_laws(&self, previous_entropy: f64) {
        // LAW 1: Energy conservation (Accounting for Multiversal Flux)
        let total_current = self.calculate_total_energy();
        let drift = (total_current - (self.initial_total_energy + self.energy_flux())).abs();
        
        if drift > crate::constants::ENERGY_EPSILON {
            warn!("⚠️ LAW 1 VIOLATION: Energy drift detected! expected={:.6}J, actual={:.6}J (Δ={:.6}J)", 
                self.initial_total_energy + self.energy_flux(), total_current, drift);
        }

        // LAW 2: Entropy monotonicity
        if let Err(e) = laws::verify_entropy_increase(previous_entropy, self.global_entropy) {
            warn!("⚠️ LAW 2 VIOLATION: {}", e);
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

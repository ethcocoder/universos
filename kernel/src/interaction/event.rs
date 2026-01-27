//! Causal Event System
//!
//! Replaces traditional OS interrupts/signals with energy-carrying physics events.
//! Events are the fundamental unit of causal influence.

use crate::types::{UniverseID, StateVector};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a causal event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventID(pub u64);

impl fmt::Display for EventID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{}", self.0)
    }
}

/// Type of causal event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    /// Pure energy transfer (like heat)
    EnergyTransfer,
    /// Information signal (like light/radio)
    Signal,
    /// Quantum entanglement setup
    Entangle,
    /// Metadata query: metadata_type, dest_addr in data
    Observation,
    /// Local timeline correction: steps in data
    Reversion,
    /// Universe replication: energy, dest_addr in data
    Branch,
    /// Memory transfer (Law 8)
    StateMigration,
    /// Destructive interference
    Cancellation,
}

/// A Causal Event - "The Photon"
///
/// Represents a packet of influence traveling through an interaction.
///
/// # Laws Enforced
///
/// - LAW 1: Carries mass-energy (energy_payload)
/// - LAW 2: Creating an event increases entropy
/// - LAW 3: Must traverse an Interaction to exist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEvent {
    /// Unique ID
    pub id: EventID,
    
    /// Type of event
    pub event_type: EventType,

    /// Origin universe
    pub source: UniverseID,

    /// Destination universe
    pub target: UniverseID,

    /// Energy carried (must be conserved)
    pub energy_payload: f64,

    /// Information carried (StateVector)
    pub data: StateVector,

    /// When it was created
    pub creation_step: u64,
    
    /// Causal trace (previous event that caused this one)
    pub cause_id: Option<EventID>,
}

impl CausalEvent {
    /// Create a new causal event
    pub fn new(
        id: EventID,
        event_type: EventType,
        source: UniverseID,
        target: UniverseID,
        energy_payload: f64,
        data: StateVector,
        creation_step: u64,
    ) -> Self {
        Self {
            id,
            event_type,
            source,
            target,
            energy_payload,
            data,
            creation_step,
            cause_id: None,
        }
    }

    /// Calculate total "mass" of the event
    ///
    /// Mass = Energy + Information Potential (Law 8)
    pub fn relativistic_mass(&self) -> f64 {
        let potential = self.data.potential_energy();
        self.energy_payload + potential
    }

    /// Set the causal link
    pub fn caused_by(mut self, cause_id: EventID) -> Self {
        self.cause_id = Some(cause_id);
        self
    }
}

/// Event Queue for Interactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventQueue {
    pub(crate) events: Vec<CausalEvent>,
}

impl EventQueue {
    /// Create a new event queue
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Push an event to the queue
    pub fn push(&mut self, event: CausalEvent) {
        self.events.push(event);
    }

    /// Pop an event from the queue (FIFO)
    pub fn pop(&mut self) -> Option<CausalEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0)) // FIFO
        }
    }

    /// Get number of events in queue
    pub fn len(&self) -> usize {
        self.events.len()
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    
    /// Calculate total energy in queue
    pub fn total_energy(&self) -> f64 {
        self.events.iter().map(|e| e.energy_payload).sum()
    }
}

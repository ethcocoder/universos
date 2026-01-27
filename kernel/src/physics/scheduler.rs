use crate::types::UniverseID;
use crate::universe::Universe;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Causal Task - A universe ready for evolution
#[derive(Debug, PartialEq)]
struct CausalTask {
    id: UniverseID,
    priority: f64,
}

impl Eq for CausalTask {}

impl PartialOrd for CausalTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl Ord for CausalTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Gravity-Based Scheduler (Phase 18)
///
/// Prioritizes universes based on 'Causal Momentum'.
/// High stability and low entropy universes evolve faster.
pub struct GravityScheduler {
    task_queue: BinaryHeap<CausalTask>,
}

impl GravityScheduler {
    pub fn new() -> Self {
        Self {
            task_queue: BinaryHeap::new(),
        }
    }

    /// Calculate causal priority for a universe
    pub fn calculate_priority(u: &Universe, pressure: f64) -> f64 {
        // Core Scheduling Formula:
        // P = (Stability / (1 + Entropy)) * (Pressure / Inertia)
        
        let stability_factor = u.stability_score;
        let efficiency_factor = 1.0 / (1.0 + u.entropy * 0.01);
        
        let resistance = u.internal_resistance();
        let flow_factor = if resistance > 0.0001 {
            pressure / resistance
        } else {
            pressure
        };

        (stability_factor * efficiency_factor * flow_factor).max(0.0)
    }

    /// Update the scheduler with current universe states
    pub fn schedule(&mut self, universes: &hashbrown::HashMap<UniverseID, Universe>, pressures: &hashbrown::HashMap<UniverseID, f64>) {
        self.task_queue.clear();
        for (id, u) in universes {
            let pressure = pressures.get(id).copied().unwrap_or(0.0);
            let priority = Self::calculate_priority(u, pressure);
            if priority > 0.0001 { // Lower threshold for high-pressure situations
                self.task_queue.push(CausalTask { id: *id, priority });
            }
        }
    }

    /// Get the next N universes to evolve
    pub fn next_tasks(&mut self, count: usize) -> Vec<(UniverseID, f64)> {
        let mut tasks = Vec::new();
        while tasks.len() < count {
            if let Some(task) = self.task_queue.pop() {
                tasks.push((task.id, task.priority));
            } else {
                break;
            }
        }
        tasks
    }
}

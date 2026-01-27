use serde::{Deserialize, Serialize};
use crate::types::StateVector;
use std::collections::HashMap;

/// A page identifier in multiversal physical memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhysicalPageID(pub u64);

/// Multiversal Paging System (Phase 17)
/// 
/// Replaces traditional flat address spaces with a physical-weighted model.
/// Law 8: Memory is Potential. Higher mass data increases gravity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiversalMemory {
    /// Physical backing store (Shared between universes)
    pub pages: HashMap<PhysicalPageID, PageData>,
    /// Thread-local virtual mapping (Virtual Page -> Physical ID)
    pub page_table: HashMap<usize, PhysicalPageID>,
    /// Page size (Default: 256 bytes for ParadoxOS)
    pub page_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageData {
    pub content: StateVector,
    /// Number of universes entangled with this page
    pub entanglement_count: u32,
    /// Thermodynamic mass (Energy cost per access)
    pub mass: f64,
}

impl MultiversalMemory {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
            page_table: HashMap::new(),
            page_size: 256,
        }
    }

    /// Access a page by virtual address
    pub fn access_page(&self, v_addr: usize) -> Option<&PageData> {
        let page_index = v_addr / self.page_size;
        self.page_table.get(&page_index).and_then(|id| self.pages.get(id))
    }

    /// Entangle with a physical page from another universe
    pub fn entangle_page(&mut self, my_v_page: usize, peer_p_id: PhysicalPageID) {
        self.page_table.insert(my_v_page, peer_p_id);
    }

    /// Calculate total thermodynamic mass (Gravity)
    pub fn total_mass(&self) -> f64 {
        self.pages.values().map(|p| {
            // Uncompressed data has more mass than compressed data (LAW 8)
            let base_mass = (p.content.size() as f64) * 0.01;
            base_mass + p.mass
        }).sum()
    }

    /// 'Swap' a page (Potentialize metadata via ParadoxLF)
    pub fn swap_to_ground_state(&mut self, id: PhysicalPageID) {
        if let Some(page) = self.pages.get_mut(&id) {
            // Compression converts kinetic state to potential state
            // In a real OS, this would be swapping to disk/NVMe
            // In ParadoxOS, this is just ParadoxLF compression
            let raw = page.content.expand();
            page.content = StateVector::compress(&raw);
        }
    }
}

//! Interaction management module
pub mod interaction;
pub mod event;

pub use interaction::Interaction;
pub use event::{CausalEvent, EventID, EventType, EventQueue};

//! Interaction management module
pub mod interaction;
pub mod event;
pub mod field;

pub use interaction::Interaction;
pub use event::{CausalEvent, EventID, EventType, EventQueue};
pub use field::InteractionField;

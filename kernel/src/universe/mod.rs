//! Universe management module
pub mod universe;
pub mod lifecycle;
pub mod isa;

pub use universe::Universe;
pub use isa::{OpCode, UniversalProcessor};
pub use lifecycle::UniverseSnapshot;

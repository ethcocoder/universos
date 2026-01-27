pub mod universe;
pub mod lifecycle;
pub mod isa;
pub mod memory;

pub use universe::Universe;
pub use isa::{OpCode, UniversalProcessor};
pub use lifecycle::UniverseSnapshot;

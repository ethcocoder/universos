//! Universal Instruction Set Architecture (ISA)
//!
//! This module defines the fundamental operations that any language must compile to
//! in order to execute on ParadoxOS.
//!
//! These operations strictly adhere to physics laws (energy costs for bit flips).

use crate::error::Result;
// use crate::types::StateVector;

/// Universal OpCodes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    /// No Operation (burns entropy)
    NoOp = 0x00,
    
    /// Set a byte in state vector: SET [addr] [val]
    AtomSet = 0x01,
    
    /// XOR a byte (reversible): XOR [addr] [val]
    AtomXor = 0x02,
    
    /// Copy memory (potential transfer): COPY [src] [dest] [len]
    AtomCopy = 0x03,
    
    /// Conditional Jump (flow control): JMP_IF [cond_addr] [target]
    JumpIf = 0x10,
    
    /// Emit Signal (interaction): SIGNAL [target_u] [len] [data...]
    Signal = 0xF0,
    
    /// Terminate/Collapse
    Halt = 0xFF,
}

impl OpCode {
    /// Create OpCode from byte
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0x00 => Some(OpCode::NoOp),
            0x01 => Some(OpCode::AtomSet),
            0x02 => Some(OpCode::AtomXor),
            0x03 => Some(OpCode::AtomCopy),
            0x10 => Some(OpCode::JumpIf),
            0xF0 => Some(OpCode::Signal),
            0xFF => Some(OpCode::Halt),
            _ => None,
        }
    }
}

/// The Universal Processor
///
/// Executes operations on a StateVector, deducting energy costs.
#[derive(Debug)]
pub struct UniversalProcessor;

impl UniversalProcessor {
    /// Execute a single instruction cycle
    ///
    /// # Arguments
    /// * `state`: The universe's memory (code + data)
    /// * `ip`: Instruction Pointer (offset in state)
    /// * `energy_budget`: Available energy
    ///
    /// # Returns
    /// * `(New IP, Energy Cost, OutputEvent)`
    pub fn step(
        state: &mut Vec<u8>,
        ip: usize,
    ) -> Result<(usize, f64, Option<crate::interaction::CausalEvent>)> {
        if ip >= state.len() {
            return Ok((0, 0.0, None)); // Wrap around or halt
        }

        let op = OpCode::from_u8(state[ip]).unwrap_or(OpCode::NoOp);
        let mut cost = 0.0001; // Base thermodynamic cost
        let mut next_ip = ip + 1;
        let mut event = None;

        match op {
            OpCode::NoOp => {
                // Just burn entropy
            }
            OpCode::AtomSet => {
                if ip + 2 < state.len() {
                    let addr = state[ip+1] as usize; // Simplified 8-bit addressing for demo
                    let val = state[ip+2];
                    
                    // LAW 1: Bit erasure costs kTint2 (simplification)
                    if state.len() > addr {
                        // If changing value, cost is higher
                        if state[addr] != val {
                            cost += 0.01;
                        }
                        state[addr] = val;
                    }
                    next_ip += 2;
                }
            }
            OpCode::AtomXor => {
                if ip + 2 < state.len() {
                    let addr = state[ip+1] as usize;
                    let val = state[ip+2];
                    if state.len() > addr {
                        state[addr] ^= val;
                        cost += 0.005; // Reversible is cheaper
                    }
                    next_ip += 2;
                }
            }
            OpCode::AtomCopy => {
                 if ip + 3 < state.len() {
                    let src = state[ip+1] as usize;
                    let dest = state[ip+2] as usize;
                    let len = state[ip+3] as usize;
                    
                    if src + len <= state.len() && dest + len <= state.len() {
                        // Manual copy to avoid borrow checker constraints if we passed slice
                        // But here we own the vec.
                        let slice = state[src..src+len].to_vec();
                        for (i, b) in slice.iter().enumerate() {
                            state[dest + i] = *b;
                        }
                        // Law 8: Moving information costs energy
                        cost += 0.001 * len as f64;
                    }
                    next_ip += 3;
                 }
            }
            OpCode::JumpIf => {
                if ip + 2 < state.len() {
                    let cond_addr = state[ip+1] as usize;
                    let target = state[ip+2] as usize;
                    
                    if state.len() > cond_addr && state[cond_addr] != 0 {
                        next_ip = target;
                    } else {
                        next_ip += 2;
                    }
                }
            }
            OpCode::Signal => {
                // SIGNAL [target_id] [len] [data...]
                if ip + 3 < state.len() {
                    let target_id = state[ip+1] as u64; // Simple addressing (0-255)
                    let len = state[ip+2] as usize;
                    
                    if ip + 3 + len <= state.len() {
                        let data = state[ip+3..ip+3+len].to_vec();
                        
                        // Create event to be sent
                        // Note: energy_payload is NOT included in 'cost'
                        // It will be deducted separately by the caller
                        event = Some(crate::interaction::CausalEvent {
                            id: crate::interaction::EventID(0), // Placeholder
                            event_type: crate::interaction::EventType::Signal,
                            source: crate::types::UniverseID(0), // Placeholder
                            target: crate::types::UniverseID(target_id),
                            energy_payload: 1.0, // Energy transmitted to target
                            data: crate::types::StateVector::compress(&data),
                            creation_step: 0, // Placeholder
                            cause_id: None,
                        });
                        
                        // Execution cost only (NOT including payload)
                        cost += 0.001 + (len as f64 * 0.0001); // Small overhead for signal processing
                        next_ip += 3 + len;
                    } else {
                        next_ip += 1; // Fault
                    }
                } else {
                    next_ip += 1;
                }
            }
            OpCode::Halt => {
                // Do not advance IP (spin) or signal termination
                return Ok((ip, 0.0, None));
            }
        }

        Ok((next_ip, cost, event))
    }
}

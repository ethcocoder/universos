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
    
    /// Add: ADD [dest] [src] - dest = dest + src
    Add = 0x04,
    
    /// Subtract: SUB [dest] [src] - dest = dest - src
    Sub = 0x05,
    
    /// Compare: CMP [a] [b] [result] - result = 1 if a > b, 0 if equal, 255 if a < b
    Cmp = 0x06,
    
    /// Unconditional Jump: JUMP [addr]
    Jump = 0x10,
    
    /// Conditional Jump (if non-zero): JMP_IF [cond_addr] [target]
    JumpIf = 0x11,
    
    /// Call subroutine: CALL [addr] (pushes return address to stack)
    Call = 0x20,
    
    /// Return from subroutine: RET (pops return address)
    Ret = 0x21,
    
    /// Push to stack: PUSH [addr]
    Push = 0x22,
    
    /// Pop from stack: POP [addr]
    Pop = 0x23,
    
    /// Emit Signal (interaction): SIGNAL [target_u] [len] [data...]
    Signal = 0xF0,

    /// Create interaction: ENTANGLE [target_u] [strength]
    Entangle = 0xF1,

    /// Read metadata: OBSERVE [target_u] [metadata_type] [dest_addr]
    /// 0=Energy, 1=Entropy, 2=Stability
    Observe = 0xF2,

    /// Local Rewind: REVERT [steps]
    Revert = 0xF3,

    /// Create new universe: BRANCH [energy] [dest_addr_for_id]
    Branch = 0xF4,

    /// Allocate memory: MEM_ALLOC [v_addr] [size]
    MemAlloc = 0xA0,

    /// Map memory (Entanglement): MEM_MAP [v_addr] [p_id]
    MemMap = 0xA1,

    /// Swap to ground state: MEM_SWAP [v_addr]
    MemSwap = 0xA2,
    
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
            0x04 => Some(OpCode::Add),
            0x05 => Some(OpCode::Sub),
            0x06 => Some(OpCode::Cmp),
            0x10 => Some(OpCode::Jump),
            0x11 => Some(OpCode::JumpIf),
            0x20 => Some(OpCode::Call),
            0x21 => Some(OpCode::Ret),
            0x22 => Some(OpCode::Push),
            0x23 => Some(OpCode::Pop),
            0xF0 => Some(OpCode::Signal),
            0xF1 => Some(OpCode::Entangle),
            0xF2 => Some(OpCode::Observe),
            0xF3 => Some(OpCode::Revert),
            0xF4 => Some(OpCode::Branch),
            0xA0 => Some(OpCode::MemAlloc),
            0xA1 => Some(OpCode::MemMap),
            0xA2 => Some(OpCode::MemSwap),
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
        memory_sys: &mut super::memory::MultiversalMemory,
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
                        let slice = state[src..src+len].to_vec();
                        for (i, b) in slice.iter().enumerate() {
                            state[dest + i] = *b;
                        }
                        cost += 0.001 * len as f64;
                    }
                    next_ip += 3;
                 }
            }
            OpCode::Add => {
                // ADD [dest] [src] - dest = dest + src
                if ip + 2 < state.len() {
                    let dest = state[ip+1] as usize;
                    let src = state[ip+2] as usize;
                    if dest < state.len() && src < state.len() {
                        state[dest] = state[dest].wrapping_add(state[src]);
                        cost += 0.002;
                    }
                    next_ip += 2;
                }
            }
            OpCode::Sub => {
                // SUB [dest] [src] - dest = dest - src
                if ip + 2 < state.len() {
                    let dest = state[ip+1] as usize;
                    let src = state[ip+2] as usize;
                    if dest < state.len() && src < state.len() {
                        state[dest] = state[dest].wrapping_sub(state[src]);
                        cost += 0.002;
                    }
                    next_ip += 2;
                }
            }
            OpCode::Cmp => {
                // CMP [a] [b] [result] - result = 1 if a > b, 0 if equal, 255 if a < b
                if ip + 3 < state.len() {
                    let a_addr = state[ip+1] as usize;
                    let b_addr = state[ip+2] as usize;
                    let result_addr = state[ip+3] as usize;
                    
                    if a_addr < state.len() && b_addr < state.len() && result_addr < state.len() {
                        let a = state[a_addr];
                        let b = state[b_addr];
                        state[result_addr] = if a > b { 1 } else if a == b { 0 } else { 255 };
                        cost += 0.001;
                    }
                    next_ip += 3;
                }
            }
            OpCode::Jump => {
                // JUMP [addr] - Unconditional jump
                if ip + 1 < state.len() {
                    let target = state[ip+1] as usize;
                    next_ip = target;
                    cost += 0.0005;
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
            OpCode::Call => {
                // CALL [addr] - Push return address (IP+2) to stack, jump to addr
                // Stack pointer is stored at address 255 (top of 8-bit address space)
                if ip + 1 < state.len() {
                    let target = state[ip+1] as usize;
                    let sp_addr = 255usize;
                    
                    if sp_addr < state.len() {
                        let sp = state[sp_addr] as usize;
                        let return_addr = (ip + 2) as u8;
                        
                        // Push return address
                        if sp > 0 && sp < state.len() {
                            state[sp] = return_addr;
                            state[sp_addr] = state[sp_addr].wrapping_sub(1); // Decrement SP
                        }
                        
                        next_ip = target;
                        cost += 0.003;
                    }
                }
            }
            OpCode::Ret => {
                // RET - Pop return address from stack, jump to it
                let sp_addr = 255usize;
                if sp_addr < state.len() {
                    let sp = state[sp_addr].wrapping_add(1) as usize; // Increment SP first
                    
                    if sp < state.len() {
                        state[sp_addr] = sp as u8;
                        next_ip = state[sp] as usize;
                        cost += 0.002;
                    }
                }
            }
            OpCode::Push => {
                // PUSH [addr] - Push value at addr to stack
                if ip + 1 < state.len() {
                    let addr = state[ip+1] as usize;
                    let sp_addr = 255usize;
                    
                    if addr < state.len() && sp_addr < state.len() {
                        let sp = state[sp_addr] as usize;
                        if sp > 0 && sp < state.len() {
                            state[sp] = state[addr];
                            state[sp_addr] = state[sp_addr].wrapping_sub(1);
                            cost += 0.002;
                        }
                    }
                    next_ip += 1;
                }
            }
            OpCode::Pop => {
                // POP [addr] - Pop value from stack to addr
                if ip + 1 < state.len() {
                    let addr = state[ip+1] as usize;
                    let sp_addr = 255usize;
                    
                    if addr < state.len() && sp_addr < state.len() {
                        let sp = state[sp_addr].wrapping_add(1) as usize;
                        if sp < state.len() {
                            state[sp_addr] = sp as u8;
                            state[addr] = state[sp];
                            cost += 0.002;
                        }
                    }
                    next_ip += 1;
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
            OpCode::Entangle => {
                // ENTANGLE [target_id] [strength]
                if ip + 2 < state.len() {
                    let target_id = state[ip+1] as u64;
                    let strength = state[ip+2] as f64 / 255.0;
                    
                    // Signals interaction creation to kernel
                    event = Some(crate::interaction::CausalEvent {
                        id: crate::interaction::EventID(0),
                        event_type: crate::interaction::EventType::Entangle,
                        source: crate::types::UniverseID(0),
                        target: crate::types::UniverseID(target_id),
                        energy_payload: strength * 10.0, // Cost of interaction
                        data: crate::types::StateVector::from_raw(vec![state[ip+2]]),
                        creation_step: 0,
                        cause_id: None,
                    });
                    
                    cost += 5.0; // High cost for entanglement
                    next_ip += 2;
                }
            }
            OpCode::Observe => {
                // OBSERVE [target_id] [meta_type] [dest]
                // 0=Energy, 1=Entropy, 2=Stability
                if ip + 3 < state.len() {
                    // This is synchronous in the kernel loop
                    event = Some(crate::interaction::CausalEvent {
                        id: crate::interaction::EventID(0),
                        event_type: crate::interaction::EventType::Observation,
                        source: crate::types::UniverseID(0),
                        target: crate::types::UniverseID(state[ip+1] as u64),
                        energy_payload: 0.1,
                        data: crate::types::StateVector::from_raw(vec![state[ip+2], state[ip+3]]),
                        creation_step: 0,
                        cause_id: None,
                    });
                    cost += 0.5;
                    next_ip += 3;
                }
            }
            OpCode::Revert => {
                // REVERT [steps]
                if ip + 1 < state.len() {
                    // Local timeline correction signal
                    event = Some(crate::interaction::CausalEvent {
                        id: crate::interaction::EventID(0),
                        event_type: crate::interaction::EventType::Reversion,
                        source: crate::types::UniverseID(0),
                        target: crate::types::UniverseID(0), // Self
                        energy_payload: state[ip+1] as f64 * 2.0,
                        data: crate::types::StateVector::from_raw(vec![state[ip+1]]),
                        creation_step: 0,
                        cause_id: None,
                    });
                    cost += 2.0;
                    next_ip += 1;
                }
            }
            OpCode::Branch => {
                // BRANCH [energy] [dest_addr_id]
                if ip + 2 < state.len() {
                    event = Some(crate::interaction::CausalEvent {
                        id: crate::interaction::EventID(0),
                        event_type: crate::interaction::EventType::Branch,
                        source: crate::types::UniverseID(0),
                        target: crate::types::UniverseID(0),
                        energy_payload: state[ip+1] as f64,
                        data: crate::types::StateVector::from_raw(vec![state[ip+2]]),
                        creation_step: 0,
                        cause_id: None,
                    });
                    cost += 10.0;
                    next_ip += 2;
                }
            }
            OpCode::MemAlloc => {
                // MEM_ALLOC [v_addr_reg] [size_reg]
                if ip + 2 < state.len() {
                    // In this demo, we just simulate the allocation cost
                    cost += 1.0; 
                    next_ip += 2;
                }
            }
            OpCode::MemMap => {
                // MEM_MAP [v_addr_reg] [p_id_reg]
                if ip + 2 < state.len() {
                    // This is the core of Memory Entanglement
                    cost += 2.0;
                    next_ip += 2;
                }
            }
            OpCode::MemSwap => {
                // MEM_SWAP [v_addr_reg]
                if ip + 1 < state.len() {
                    let v_addr = state[ip+1] as usize;
                    let page_index = v_addr / memory_sys.page_size;
                    
                    if let Some(&p_id) = memory_sys.page_table.get(&page_index) {
                        memory_sys.swap_to_ground_state(p_id);
                    }
                    
                    cost += 0.5;
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

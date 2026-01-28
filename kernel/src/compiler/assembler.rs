//! Enhanced Assembler for Universal ISA
//!
//! Supports:
//! - Human-readable mnemonics (SET, ADD, etc.)
//! - Labels (start:)
//! - Definitions (.def name value)
//! - Comments (# or //)

use crate::universe::isa::OpCode;
use std::collections::HashMap;

/// Assemble source code into bytecode
pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut labels = HashMap::new();
    let mut definitions = HashMap::new();
    let mut byte_offset = 0;
    
    // ==========================================
    // Pass 1: Symbol Discovery and Offset Calculation
    // ==========================================
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        // Handle Definitions: .def name value
        if trimmed.starts_with(".def") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 3 { return Err(format!("Line {}: .def requires name and value", line_num)); }
            let val = parts[2].parse::<u8>().map_err(|_| format!("Line {}: Invalid def value", line_num))?;
            definitions.insert(parts[1].to_string(), val);
            continue;
        }

        // Handle Labels: name:
        if trimmed.ends_with(':') {
            let label_name = &trimmed[..trimmed.len()-1]; // Remove trailing colon
            labels.insert(label_name.to_string(), byte_offset);
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_uppercase().as_str() {
            "NOP" | "RET" | "HALT" => byte_offset += 1,
            "PUSH" | "POP" | "JUMP" | "JMP" | "CALL" => byte_offset += 2,
            "SET" | "XOR" | "ADD" | "SUB" | "JUMPIF" | "JIF" | "JNZ" => byte_offset += 3,
            "COPY" | "CMP" | "SIGNAL" => {
               if parts[0].eq_ignore_ascii_case("SIGNAL") {
                    // SIGNAL target "message"
                     // OpCode + Target + Len + Payload
                     let rest = trimmed.splitn(3, ' ').nth(2).unwrap_or("");
                     let payload_len = if rest.starts_with('"') && rest.ends_with('"') {
                         rest.len() - 2 // Quotes
                     } else {
                         rest.len()
                     };
                     byte_offset += 3 + payload_len;
               } else {
                   // COPY/CMP have 3 args + opcode
                   byte_offset += 4;
               }
            },
            _ => return Err(format!("Line {}: Unknown opcode '{}'", line_num, parts[0])),
        }
    }

    // ==========================================
    // Pass 2: Code Generation
    // ==========================================
    let mut bytecode = Vec::new();

    // Helper to resolve arguments (number, def, or label)
    let resolve_arg = |arg: &str, ln: usize| -> Result<u8, String> {
        // Try direct number
        if let Ok(val) = arg.parse::<u8>() {
             return Ok(val);
        }
        // Try definitions
        if let Some(val) = definitions.get(arg) {
            return Ok(*val);
        }
        // Try labels
        if let Some(offset) = labels.get(arg) {
            return Ok(*offset as u8);
        }
        
        Err(format!("Line {}: Unknown symbol '{}'", ln, arg))
    };

    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") 
           || trimmed.starts_with(".def") || trimmed.ends_with(':') {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        match parts[0].to_uppercase().as_str() {
            "NOP" => bytecode.push(OpCode::NoOp as u8),
            "SET" => {
                bytecode.push(OpCode::AtomSet as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
            },
            "XOR" => {
                bytecode.push(OpCode::AtomXor as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
            },
            "COPY" => {
                bytecode.push(OpCode::AtomCopy as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
                bytecode.push(resolve_arg(parts[3], line_num)?);
            },
            "ADD" => {
                bytecode.push(OpCode::Add as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
            },
            "SUB" => {
                bytecode.push(OpCode::Sub as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
            },
            "CMP" => {
                bytecode.push(OpCode::Cmp as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?);
                bytecode.push(resolve_arg(parts[3], line_num)?);
            },
            "JUMP" | "JMP" => {
                bytecode.push(OpCode::Jump as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
            },
            "JUMPIF" | "JIF" | "JNZ" => {
                bytecode.push(OpCode::JumpIf as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
                bytecode.push(resolve_arg(parts[2], line_num)?); // This can resolve label!
            },
            "CALL" => {
                bytecode.push(OpCode::Call as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
            },
            "RET" => bytecode.push(OpCode::Ret as u8),
            "PUSH" => {
                bytecode.push(OpCode::Push as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
            },
            "POP" => {
                bytecode.push(OpCode::Pop as u8);
                bytecode.push(resolve_arg(parts[1], line_num)?);
            },
            "SIGNAL" => {
                bytecode.push(OpCode::Signal as u8);
                
                let target = resolve_arg(parts[1], line_num)?;
                bytecode.push(target);
                
                let rest_of_line = trimmed.splitn(3, ' ').nth(2).unwrap_or("");
                let payload = if rest_of_line.starts_with('"') && rest_of_line.ends_with('"') {
                    rest_of_line[1..rest_of_line.len()-1].as_bytes()
                } else {
                    rest_of_line.as_bytes()
                };

                if payload.len() > 255 {
                   return Err(format!("Line {}: Payload too long", line_num));
                }
                bytecode.push(payload.len() as u8);
                bytecode.extend_from_slice(payload);
            },
            "HALT" => bytecode.push(OpCode::Halt as u8),
            _ => {}, // Should be caught in pass 1
        }
    }
    
    Ok(bytecode)
}

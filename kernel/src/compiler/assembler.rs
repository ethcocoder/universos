//! Simple Assembler for Universal ISA
//!
//! Compiles human-readable instructions into ISA bytecode.

use crate::universe::isa::OpCode;

/// Assemble source code into bytecode
pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut bytecode = Vec::new();
    
    for (line_num, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_uppercase().as_str() {
            "NOP" => bytecode.push(OpCode::NoOp as u8),
            "SET" => {
                // SET addr val
                if parts.len() < 3 { return Err(format!("Line {}: SET requires addr and val", line_num)); }
                bytecode.push(OpCode::AtomSet as u8);
                bytecode.push(parts[1].parse().map_err(|_| format!("Line {}: Invalid addr", line_num))?);
                bytecode.push(parts[2].parse().map_err(|_| format!("Line {}: Invalid val", line_num))?);
            },
            "XOR" => {
                if parts.len() < 3 { return Err(format!("Line {}: XOR requires addr and val", line_num)); }
                bytecode.push(OpCode::AtomXor as u8);
                bytecode.push(parts[1].parse().map_err(|_| format!("Line {}: Invalid addr", line_num))?);
                bytecode.push(parts[2].parse().map_err(|_| format!("Line {}: Invalid val", line_num))?);
            },
            "COPY" => {
                if parts.len() < 4 { return Err(format!("Line {}: COPY requires src dest len", line_num)); }
                bytecode.push(OpCode::AtomCopy as u8);
                bytecode.push(parts[1].parse().map_err(|_| format!("Line {}: Invalid src", line_num))?);
                bytecode.push(parts[2].parse().map_err(|_| format!("Line {}: Invalid dest", line_num))?);
                bytecode.push(parts[3].parse().map_err(|_| format!("Line {}: Invalid len", line_num))?);
            },
            "SIGNAL" => {
                // SIGNAL target "message"
                if parts.len() < 3 { return Err(format!("Line {}: SIGNAL requires target and payload", line_num)); }
                bytecode.push(OpCode::Signal as u8);
                
                let target: u8 = parts[1].parse().map_err(|_| format!("Line {}: Invalid target", line_num))?;
                bytecode.push(target);
                
                // Parse payload string
                let rest_of_line = trimmed.splitn(3, ' ').nth(2).unwrap_or("");
                let payload = if rest_of_line.starts_with('"') && rest_of_line.ends_with('"') {
                    rest_of_line[1..rest_of_line.len()-1].as_bytes()
                } else {
                    rest_of_line.as_bytes()
                };

                if payload.len() > 255 {
                    return Err(format!("Line {}: Payload too long (>255 bytes)", line_num));
                }

                bytecode.push(payload.len() as u8);
                bytecode.extend_from_slice(payload);
            },
            "HALT" => bytecode.push(OpCode::Halt as u8),
            _ => return Err(format!("Line {}: Unknown opcode '{}'", line_num, parts[0])),
        }
    }
    
    Ok(bytecode)
}

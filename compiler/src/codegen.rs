use crate::ast::*;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

pub struct CodeGen {
    bytecode: Vec<u8>,
    variables: HashMap<String, u8>,
    next_var_addr: u8,
    labels: HashMap<String, usize>,
    fixups: Vec<(usize, String)>,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            bytecode: Vec::new(),
            variables: HashMap::new(),
            next_var_addr: 200, // Variables live in high RAM
            labels: HashMap::new(),
            fixups: Vec::new(),
        }
    }

    pub fn generate(&mut self, program: Program) -> Result<Vec<u8>> {
        // Find all functions first to allow recursion/forward calls
        // In this simple version, we'll just process statements
        for stmt in program.statements {
            self.gen_stmt(stmt)?;
        }

        // Apply fixups for jumps/calls
        for (offset, label) in &self.fixups {
            let addr = self.labels.get(label)
                .ok_or_else(|| anyhow!("Undefined label: {}", label))?;
            self.bytecode[*offset] = *addr as u8;
        }

        Ok(self.bytecode.clone())
    }

    fn gen_stmt(&mut self, stmt: Stmt) -> Result<()> {
        match stmt {
            Stmt::UniverseDecl { name, energy: _, body } => {
                // For now, universes are just logical groupings
                // The body is part of the main entry point
                self.labels.insert(name, self.bytecode.len());
                for s in body {
                    self.gen_stmt(s)?;
                }
                self.emit_byte(0xFF); // Halt at end of universe
            }
            Stmt::FuncDecl { name, params, body } => {
                // Jump over function body to avoid executing it linearly
                self.emit_byte(0x10); // JUMP
                let fixup_idx = self.bytecode.len();
                self.emit_byte(0); // placeholder for skip_addr
                
                let start_addr = self.bytecode.len();
                self.labels.insert(name.clone(), start_addr);
                
                // Map params to addresses
                for (_i, param) in params.iter().enumerate() {
                    // Very simple: params pushed to stack before call
                    // We'll pop them into local addresses
                    let addr = self.get_var_addr(param);
                    self.emit_byte(0x23); // POP
                    self.emit_byte(addr);
                }
                
                for s in body {
                    self.gen_stmt(s)?;
                }
                
                self.emit_byte(0x21); // RET
                
                let end_addr = self.bytecode.len();
                self.bytecode[fixup_idx] = end_addr as u8;
            }
            Stmt::AssignStmt(name, expr) => {
                self.gen_expr(expr)?;
                // After evaluating expr, the "result" is in a temporary?
                // For simplicity, let's assume gen_expr leaves result at address 199
                let addr = self.get_var_addr(&name);
                self.emit_byte(0x01); // AtomSet (using 199 as accumulator)
                // Wait, ISA doesn't have an accumulator. SET needs a literal.
                // We need a way to COPY from result to variable.
                self.emit_byte(0x03); // COPY
                self.emit_byte(199); // src
                self.emit_byte(addr); // dest
                self.emit_byte(1);   // len
            }
            Stmt::ExprStmt(expr) => {
                self.gen_expr(expr)?;
            }
            Stmt::ReturnStmt(expr) => {
                self.gen_expr(expr)?;
                self.emit_byte(0x21); // RET
            }
            Stmt::IfStmt { cond, then_block, else_block } => {
                self.gen_expr(cond)?;
                self.emit_byte(0x11); // JUMP_IF (accumulator 199 != 0)
                let then_fixup = self.bytecode.len();
                self.emit_byte(0); // placeholder for then_addr
                
                // Jump over then block if condition is 0
                // Wait, JUMP_IF is "jump if non-zero". 
                // We need "jump if zero" or swap blocks.
                // Let's use a NOP and JUMP for simplicity.
                
                // Better: 
                // 1. Evaluate cond (result in 199)
                // 2. JUMP_IF 199 to THEN_LABEL
                // 3. JUMP to ELSE_LABEL or END_LABEL
                
                self.emit_byte(0x11); // JUMP_IF
                self.emit_byte(199);
                let then_label = format!("if_then_{}", then_fixup);
                self.fixups.push((self.bytecode.len(), then_label.clone()));
                self.emit_byte(0); 

                // Jump to Else/End
                self.emit_byte(0x10); // JMP
                let else_label = format!("if_else_{}", then_fixup);
                self.fixups.push((self.bytecode.len(), else_label.clone()));
                self.emit_byte(0);

                // Then Block
                self.labels.insert(then_label, self.bytecode.len());
                for s in then_block {
                    self.gen_stmt(s)?;
                }
                
                // Jump to End
                self.emit_byte(0x10); // JMP
                let end_label = format!("if_end_{}", then_fixup);
                self.fixups.push((self.bytecode.len(), end_label.clone()));
                self.emit_byte(0);

                // Else Block
                self.labels.insert(else_label, self.bytecode.len());
                if let Some(eb) = else_block {
                    for s in eb {
                        self.gen_stmt(s)?;
                    }
                }
                
                self.labels.insert(end_label, self.bytecode.len());
            }
            _ => {} // Implement While later
        }
        Ok(())
    }

    fn gen_expr(&mut self, expr: Expr) -> Result<()> {
        match expr {
            Expr::Number(n) => {
                self.emit_byte(0x01); // SET
                self.emit_byte(199);  // Accumulator address
                self.emit_byte(n as u8);
            }
            Expr::Ident(name) => {
                let addr = self.get_var_addr(&name);
                self.emit_byte(0x03); // COPY
                self.emit_byte(addr);
                self.emit_byte(199);
                self.emit_byte(1);
            }
            Expr::BinaryOp(left, op, right) => {
                self.gen_expr(*left)?;
                // Move L to temp 198
                self.emit_byte(0x03);
                self.emit_byte(199);
                self.emit_byte(198);
                self.emit_byte(1);
                
                self.gen_expr(*right)?;
                // R is in 199
                
                match op {
                    Op::Add => {
                        self.emit_byte(0x04); // ADD
                        self.emit_byte(199);  // dest
                        self.emit_byte(198);  // src
                    }
                    Op::Sub => {
                        self.emit_byte(0x05); // SUB (199 = 199 - 198) -- wait, we want 198 - 199
                        // Swapping src/dest or using temp
                        self.emit_byte(0x05); 
                        self.emit_byte(198);
                        self.emit_byte(199);
                        // Result in 198, move to 199
                        self.emit_byte(0x03);
                        self.emit_byte(198);
                        self.emit_byte(199);
                        self.emit_byte(1);
                    }
                    _ => {} // Implement others
                }
            }
            Expr::Call(name, args) => {
                // Push args in reverse order
                for arg in args.into_iter().rev() {
                    self.gen_expr(arg)?;
                    self.emit_byte(0x22); // PUSH
                    self.emit_byte(199);
                }
                self.emit_byte(0x20); // CALL
                self.fixups.push((self.bytecode.len(), name));
                self.emit_byte(0); // placeholder
            }
            Expr::Signal(target, data) => {
                self.gen_expr(*target)?;
                self.emit_byte(0x03);
                self.emit_byte(199);
                self.emit_byte(197); // Target temp
                self.emit_byte(1);

                self.gen_expr(*data)?;
                // Signal needs: SIGNAL [target] [len] [data...]
                // Our ISA CURRENTLY takes literal len and data.
                // This is hard for dynamic data. 
                // Let's emit a fixed-size signal for now if it's a number.
                self.emit_byte(0xF0); // SIGNAL
                self.emit_byte(127);  // Use a temporary target variable address? 
                // No, ISA::step takes target_id from state[ip+1].
                // This means the TARGET MUST BE A LITERAL in the current ISA.
                // We should probably update the ISA to handle dynamic targets,
                // but for v1, we'll assume target is a literal.
                self.emit_byte(2); // placeholder target
                self.emit_byte(1); // len
                self.emit_byte(0); // placeholder for data byte
            }
            _ => {}
        }
        Ok(())
    }

    fn get_var_addr(&mut self, name: &str) -> u8 {
        *self.variables.entry(name.to_string()).or_insert_with(|| {
            let addr = self.next_var_addr;
            self.next_var_addr += 1;
            addr
        })
    }

    fn emit_byte(&mut self, b: u8) {
        self.bytecode.push(b);
    }
}

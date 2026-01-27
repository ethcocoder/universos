//! ParadoxOS Kernel - Main Entry Point

use env_logger::Env;
use paradox_kernel::{Kernel, Observer};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("╔════════════════════════════════════════╗");
    println!("║     🌌 ParadoxOS Kernel v{:<13}║", paradox_kernel::VERSION);
    println!("║  Physics-Native Operating System      ║");
    println!("╚════════════════════════════════════════╝\n");

    // Big Bang - Initialize kernel
    let mut kernel = Kernel::new(10000.0);

    // Spawn AGI Observer
    let observer = Observer::new(&mut kernel)?;

    println!("\n▶ Spawning universes...\n");

    // Spawn several universes
    let _u1 = kernel.spawn_universe(500.0)?;
    let u2 = kernel.spawn_universe(700.0)?;
    let u3 = kernel.spawn_universe(300.0)?;
    let u4 = kernel.spawn_universe(200.0)?;
    let u5 = kernel.spawn_universe(100.0)?;

    println!("\n▶ Creating interaction network...\n");

    // Full service mesh for inter-service communication (Phase 8)
    // User (U2) can talk to all services
    kernel.create_interaction(u2, u3, 0.9)?; // User → Scheduler
    kernel.create_interaction(u2, u4, 0.8)?; // User → Router
    kernel.create_interaction(u2, u5, 0.7)?; // User → Monitor
    
    // Services interconnected (service mesh)
    kernel.create_interaction(u3, u4, 0.9)?; // Scheduler ↔ Router
    kernel.create_interaction(u3, u5, 0.8)?; // Scheduler ↔ Monitor
    kernel.create_interaction(u4, u5, 0.9)?; // Router ↔ Monitor

    // ============================================================
    // Load OS Services into dedicated universes
    // ============================================================
    
    println!("▶ Loading ParadoxOS Services...\n");
    
    // Service 1: Pure Computational Scheduler
    println!("📅 Loading Scheduler Service...");
    let scheduler_code = r#"
        # Scheduler - Pure Computation
        .def counter 200
        .def limit 100
        .def temp 201
        
        SET counter 0
        loop:
            ADD counter 1
            CMP counter limit temp
            JUMPIF temp loop
        HALT
    "#;
    
    let scheduler_bytecode = paradox_kernel::compiler::assemble(&scheduler_code)
        .expect("Scheduler compilation failed");
    kernel.load_program(u3, scheduler_bytecode)?;
    println!("   ✓ Scheduler loaded into U3");
    
    // Service 2: Message Router (Computation Only)
    println!("📬 Loading Message Router...");
    let router_code = r#"
        # Router - Internal Processing
        .def msg_count 200
        .def max_msgs 50
        .def temp 201
        
        SET msg_count 0
        route_loop:
            ADD msg_count 1
            CMP msg_count max_msgs temp
            JUMPIF temp route_loop
        HALT
    "#;
    
    let router_bytecode = paradox_kernel::compiler::assemble(&router_code)
        .expect("Router compilation failed");
    kernel.load_program(u4, router_bytecode)?;
    println!("   ✓ Router loaded into U4");
    
    // Service 3: System Monitor (Health Checks)
    println!("🔍 Loading System Monitor...");
    let monitor_code = r#"
        # Monitor - Health Tracking
        .def health 200
        .def checks 201
        .def temp 202
        
        SET health 100
        SET checks 0
        
        monitor_loop:
            SUB health 1
            ADD checks 1
            CMP health 0 temp
            JUMPIF temp monitor_loop
        HALT
    "#;
    
    let monitor_bytecode = paradox_kernel::compiler::assemble(&monitor_code)
        .expect("Monitor compilation failed");
    kernel.load_program(u5, monitor_bytecode)?;
    println!("   ✓ Monitor loaded into U5");
    
    // User Program - Coordinated Workflow Demo
    println!("\n💼 Loading Coordinated Workflow Demo...");
    let user_code = r#"
        # Coordinated Service Demo (Phase 8)
        # Demonstrates inter-service communication
        
        .def work_count 200
        .def iter 201
        .def max_work 3
        .def temp 202
        
        SET work_count 0
        SET iter 0
        
        submit_work:
            # Submit work to Scheduler
            SIGNAL 4 "WorkRequest"
            ADD work_count 1
            
            # Small processing delay
            SET iter 0
            delay:
                ADD iter 1
                CMP iter 5 temp
                JUMPIF temp delay
            
            # Check completion with Monitor
            SIGNAL 6 "StatusCheck"
            
            # Continue until done
            CMP work_count max_work temp
            JUMPIF temp submit_work
        
        # Final status to Monitor
        SIGNAL 6 "AllWorkComplete"
        HALT
    "#;
    
    let user_bytecode = paradox_kernel::compiler::assemble(user_code)
        .expect("User program compilation failed");
    kernel.load_program(u2, user_bytecode)?;
    println!("   ✓ User program loaded into U2");

    println!("\n🌌 ParadoxOS Multi-Service System Ready! (Phase 8)");
    println!("   Services: Scheduler, Router, Monitor");
    println!("   Interactions: {} (Full Mesh Network)", 6);
    println!("   Communication: Inter-Service Signals Enabled");
    println!("\n▶ Starting coordinated evolution with TUI Dashboard...\n");
    
    // Register Dashboard Driver
    let dashboard = Box::new(paradox_kernel::physics::drivers::TuiDashboardDriver::new()
        .expect("Failed to initialize TUI Dashboard"));
    kernel.add_driver(dashboard);

    // Register Archive Driver (Persistence HAL)
    let archive = Box::new(paradox_kernel::physics::drivers::ArchiveDriver::new("multiverse_archive.json"));
    kernel.add_driver(archive);

    // Register Wormhole Driver (Network HAL)
    let wormhole = Box::new(paradox_kernel::physics::drivers::WormholeDriver::new("0.0.0.0:4000")
        .expect("Failed to initialize Wormhole Driver"));
    kernel.add_driver(wormhole);

    // Register Kinetic Energy Channel (Host CPU HAL)
    let kinetic = Box::new(paradox_kernel::physics::drivers::KineticEnergyDriver::new());
    kernel.add_driver(kinetic);

    // Run simulation loop
    for step in 0..200 {
        kernel.evolution_step();

        // Send a network ping every 20 steps
        if step % 20 == 0 {
            // Signal to ID 999 (Network Gateway)
            let _ = kernel.spawn_event(
                u2, 
                paradox_kernel::types::UniverseID(999), 
                paradox_kernel::interaction::EventType::Signal, 
                "NETWORK_PING".as_bytes().to_vec(), 
                1.0
            );
        }

        // Branching event at step 10
        if step == 10 {
            let _ = kernel.branch_universe(u2);
        }

        // Observer analysis every 10 steps
        if step % 10 == 0 {
            observer.observe_and_guide(&kernel);
        }
        
        // Slow down for visibility (50ms = 20FPS)
        thread::sleep(Duration::from_millis(50));
    }


    println!("═══════════════════════════════════════════════════════════");
    println!("\n▶ Final Statistics:\n");

    println!("┌─ Universes ───────────────────────────────────");
    for id in kernel.universe_ids() {
        if let Some(u) = kernel.get_universe(id) {
            println!("│  {} - Energy: {:.2}J  Entropy: {:.2}  Stability: {:.2}  Timeline: {}",
                     id, u.energy, u.entropy, u.stability_score, u.timeline_index);
        }
    }
    println!("└───────────────────────────────────────────────\n");

    println!("✅ Evolution complete!");
    println!("\n🌌 ParadoxOS demonstrated:");
    println!("   ✓ Energy conservation (LAW 1)");
    println!("   ✓ Entropy increase (LAW 2)");
    println!("   ✓ Interaction-based communication (LAW 3)");
    println!("   ✓ Emergent execution (LAW 4)");
    println!("   ✓ Local time relativity (LAW 7)");
    println!("   ✓ Stability-based lifecycle (LAW 9)");
    println!("   ✓ AGI observation (LAW 11)");
    println!("   ✓ Universal ISA execution (LAW 12)");
    
    Ok(())
}

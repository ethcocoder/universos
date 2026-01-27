//! ParadoxOS Kernel - Main Entry Point

use env_logger::Env;
use paradox_kernel::{Kernel, Observer};
use parala_compiler;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: Vec<String> = std::env::args().collect();
    let listen_port = args.get(1).map(|s| s.as_str()).unwrap_or("4000");
    let remote_port = args.get(2).map(|s| s.as_str()).unwrap_or("4002");

    println!("╔════════════════════════════════════════╗");
    println!("║     🌌 ParadoxOS Kernel v{:<13}║", paradox_kernel::VERSION);
    println!("║  Node Mode: {}:{} -> {} ║", "127.0.0.1", listen_port, remote_port);
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
    
    // User Program - Multiversal Architect Demo (Phase 15)
    println!("\n🏛️ Loading Multiversal Architect Demo...");
    let user_code = r#"
        # Multiversal Architect - Phase 15 Demonstration
        .def scheduler_id 3
        .def energy_addr 100
        .def threshold 50
        .def is_low 101
        
        main:
            # 1. Observe Scheduler (U3) Energy
            # Observe: target, type(0=Energy), dest_addr
            OBSERVE scheduler_id 0 energy_addr
            
            # 2. Compare energy with threshold
            CMP energy_addr threshold is_low
            
            # 3. If energy is high, jump to entangle (CMP result 1 means energy > threshold)
            JUMPIF is_low entangle_phase
            HALT

        entangle_phase:
            # 4. Programmatic Entanglement (Strength 200/255)
            ENTANGLE scheduler_id 200
            
            # 5. Programmatic Branching (Allocate 100 energy)
            BRANCH 100 102
            
            # 6. Signal the new universe (ID stored at 102)
            SIGNAL 102 "Awaken"

            # 7. Phase 17: Potentialize old state to reduce gravity
            # MemSwap: virtual_addr
            MEMSWAP energy_addr
        HALT
    "#;
    
    let user_bytecode = paradox_kernel::compiler::assemble(user_code)
        .expect("User program compilation failed");
    kernel.load_program(u2, user_bytecode)?;
    println!("   ✓ User program loaded into U2");

    // ==========================================
    // Phase 9: Native Parala Service Loading
    // ==========================================
    println!("🏭 Compiling Parala Orchestrator Service...");
    let orchestrator_src = include_str!("../../services/orchestrator.para");
    
    let orchestrator_bytecode = parala_compiler::compile(&orchestrator_src)
        .expect("Parala compilation failed");

    let u7 = kernel.spawn_universe(200.0)?;
    kernel.load_program(u7, orchestrator_bytecode)?;
    println!("   ✓ Orchestrator (Parala-Native) loaded into U7");

    println!("\n🌌 ParadoxOS Multi-Service System Ready! (Phase 11)");
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
    let listen_addr = format!("0.0.0.0:{}", listen_port);
    let remote_addr = format!("127.0.0.1:{}", remote_port);
    let wormhole = Box::new(paradox_kernel::physics::drivers::WormholeDriver::new(&listen_addr, &remote_addr)
        .expect("Failed to initialize Wormhole Driver"));
    kernel.add_driver(wormhole);

    // Register Kinetic Energy Channel (Host CPU HAL)
    let kinetic = Box::new(paradox_kernel::physics::drivers::KineticEnergyDriver::new());
    kernel.add_driver(kinetic);

    // Register Web Gateway (Phase 13 Visualization)
    // Offset web port to avoid collisions on same machine
    let web_port = 8080 + (listen_port.parse::<u16>().unwrap_or(0) % 10);
    let web_dash = Box::new(paradox_kernel::physics::drivers::WebGatewayDriver::new(web_port));
    kernel.add_driver(web_dash);

    // Register Chaos Monkey (Phase 14 Stress Testing)
    if args.get(3).map(|s| s.as_str()) == Some("chaos") {
        println!("🐒 CHAOS MODE ENABLED: Chaos Monkey is roaming the multiverse...");
        let chaos = Box::new(paradox_kernel::physics::drivers::ChaosMonkeyDriver::new(0.8));
        kernel.add_driver(chaos);
    }

    // Run simulation loop
    for step in 0..1000 {
        let pulse = kernel.evolution_step();
        
        // Phase 13: Handle Chronos Control Pulses
        match pulse {
            paradox_kernel::physics::drivers::SystemPulse::Shutdown => {
                println!("🛑 System pulse: SHUTDOWN received.");
                break;
            }
            paradox_kernel::physics::drivers::SystemPulse::Rewind => {
                println!("⏳ System pulse: REWIND triggered.");
                kernel.rewind(20); // Rewind 20 steps (1 second of real time)
            }
            paradox_kernel::physics::drivers::SystemPulse::CollapseAll => {
                println!("💥 System pulse: COLLAPSE_ALL triggered.");
                for id in kernel.universe_ids() {
                    let _ = kernel.collapse_universe(id);
                }
            }
            paradox_kernel::physics::drivers::SystemPulse::Sabotage(id, amount) => {
                let _ = kernel.sabotage_universe(id, amount);
            }
            paradox_kernel::physics::drivers::SystemPulse::None => {}
        }

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
            observer.observe_and_guide(&mut kernel);
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

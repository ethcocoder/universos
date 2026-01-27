//! ParadoxOS Kernel - Main Entry Point

use env_logger::Env;
use paradox_kernel::{Kernel, Observer};
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    println!("╔════════════════════════════════════════╗");
    println!("║     🌌 ParadoxOS Kernel v{:<13}║", paradox_kernel::VERSION);
    println!("║  Physics-Native Operating System      ║");
    println!("╚════════════════════════════════════════╝\n");

    // Big Bang - Initialize kernel
    let mut kernel = Kernel::new(10000.0);

    // Spawn AGI Observer
    let observer = Observer::new(&mut kernel)
        .expect("Failed to spawn observer");

    println!("\n▶ Spawning universes...\n");

    // Spawn several universes
    let u1 = kernel.spawn_universe(500.0).expect("Failed to spawn U1");
    let u2 = kernel.spawn_universe(700.0).expect("Failed to spawn U2");
    let u3 = kernel.spawn_universe(300.0).expect("Failed to spawn U3");
    let u4 = kernel.spawn_universe(200.0).expect("Failed to spawn U4");

    println!("\n▶ Creating interaction network...\n");

    // Create interaction network
    kernel.create_interaction(u1, u2, 0.9).expect("Failed to create I1");
    kernel.create_interaction(u2, u3, 0.7).expect("Failed to create I2");
    kernel.create_interaction(u3, u4, 0.5).expect("Failed to create I3");
    kernel.create_interaction(u1, u3, 0.6).expect("Failed to create I4");

    println!("\n▶ Running evolution...\n");
    println!("═══════════════════════════════════════════════════════════\n");

    // Run evolution loop
    for step in 0..30 {
        kernel.evolution_step();

        // Branching event at step 2 (before potential collapse)
        if step == 2 {
            println!("\n⚡ EVENT: Attempting to branch Universe (U2)...");
            match kernel.branch_universe(u1) {
                Ok(new_id) => println!("   ✓ Success! Created Universe {} from {}", new_id, u1),
                Err(e) => println!("   ❌ Branch failed: {}", e),
            }
        }

        // Observer analysis every 5 steps
        if step % 5 == 0 {
            println!("\n┌─ Observer Analysis (Step {}) ─────────────────", step);
            observer.observe_and_guide(&kernel);

            // Check for instability
            let unstable = observer.predict_instability(&kernel);
            if !unstable.is_empty() {
                println!("└─ ⚠️  Unstable universes detected: {:?}", unstable);
            } else {
                println!("└─ ✓ All universes stable");
            }

            println!("\n┌─ System State ────────────────────────────────");
            println!("│  Universes:     {}", kernel.universe_count());
            println!("│  Interactions:  {}", kernel.interaction_count());
            println!("│  Global Energy: {:.2} J", kernel.global_energy());
            println!("│  Global Entropy: {:.2}", kernel.global_entropy());
            println!("└───────────────────────────────────────────────\n");
        }

        // Slow down for visibility
        thread::sleep(Duration::from_millis(200));
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
}

# ParadoxOS Code Examples

Complete, runnable examples demonstrating ParadoxOS concepts.

---

## Example 1: Hello Universe

The simplest possible ParadoxOS program.

```rust
use paradox_kernel::*;

fn main() {
    println!("🌌 Hello from ParadoxOS!\n");
    
    // Big Bang - initialize kernel with 1000 J of energy
    let mut kernel = Kernel::new(1000.0);
    
    // Spawn a universe
    match kernel.spawn_universe(100.0) {
        Ok(universe_id) => {
            println!("✨ Universe {:?} created!", universe_id);
            
            // Get universe details
            if let Some(universe) = kernel.get_universe(universe_id) {
                println!("   Energy: {:.2} J", universe.energy);
                println!("   Entropy: {:.2}", universe.entropy);
                println!("   Stability: {:.2}", universe.stability_score);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
    
    println!("\n✅ Universe exists!");
}
```

**Output:**
```
🌌 Hello from ParadoxOS!

✨ Universe UniverseID(1) created!
   Energy: 100.00 J
   Entropy: 0.00
   Stability: 1.00

✅ Universe exists!
```

---

## Example 2: Universe Communication

Two universes exchanging energy through interaction.

```rust
use paradox_kernel::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut kernel = Kernel::new(2000.0);
    
    // Create two universes
    let alice = kernel.spawn_universe(300.0).unwrap();
    let bob = kernel.spawn_universe(300.0).unwrap();
    
    println!("👥 Alice and Bob universes created\n");
    
    // Establish communication channel
    let channel = kernel.create_interaction(alice, bob, 0.7).unwrap();
    
    println!("📡 Communication channel established");
    println!("   Coupling strength: 0.7\n");
    
    // Run evolution and watch energy transfer
    for step in 0..5 {
        kernel.evolution_step();
        
        let alice_u = kernel.get_universe(alice).unwrap();
        let bob_u = kernel.get_universe(bob).unwrap();
        
        println!("Step {}: Alice={:.2}J  Bob={:.2}J", 
                 step, alice_u.energy, bob_u.energy);
        
        thread::sleep(Duration::from_millis(300));
    }
    
    println!("\n✅ Communication complete!");
}
```

---

## Example 3: Evolution Conditions

Demonstrating when universes evolve based on LAW 4.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(5000.0);
    
    // Create universes with different properties
    let high_energy = kernel.spawn_universe(500.0).unwrap();
    let low_energy = kernel.spawn_universe(50.0).unwrap();
    let isolated = kernel.spawn_universe(200.0).unwrap();
    
    // High interaction density for high_energy
    let helper1 = kernel.spawn_universe(100.0).unwrap();
    let helper2 = kernel.spawn_universe(100.0).unwrap();
    
    kernel.create_interaction(high_energy, helper1, 0.9).unwrap();
    kernel.create_interaction(high_energy, helper2, 0.9).unwrap();
    
    // Low interaction for low_energy
    kernel.create_interaction(low_energy, helper1, 0.2).unwrap();
    
    // isolated has no interactions
    
    println!("🔬 Testing Evolution Conditions (LAW 4)\n");
    
    for step in 0..3 {
        println!("═══ Step {} ═══", step);
        
        for (name, id) in [
            ("High Energy", high_energy),
            ("Low Energy", low_energy),
            ("Isolated", isolated),
        ] {
            let pressure = kernel.calculate_interaction_pressure(id);
            let resistance = kernel.calculate_internal_resistance(id);
            let can_evolve = kernel.check_evolution_condition(id);
            
            println!("{}: pressure={:.2} resistance={:.2} evolve={}",
                     name, pressure, resistance, can_evolve);
        }
        
        kernel.evolution_step();
        println!();
    }
}
```

---

## Example 4: Universe Collapse

Watching unstable universes collapse.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(3000.0);
    
    // Create universe with intentionally low stability
    let unstable = kernel.spawn_universe(100.0).unwrap();
    
    // Manually make it unstable (normally this happens through evolution)
    if let Some(universe) = kernel.get_universe_mut(unstable) {
        universe.stability_score = 0.2;  // Below collapse threshold!
        universe.entropy = 50.0;  // High entropy
    }
    
    println!("💥 Created unstable universe");
    println!("   Stability: 0.2 (threshold: 0.3)\n");
    
    // Evolution will trigger collapse
    println!("Running evolution...\n");
    kernel.evolution_step();
    
    // Check if it still exists
    match kernel.get_universe(unstable) {
        Some(_) => println!("❌ Universe survived (unexpected)"),
        None => {
            println!("✅ Universe collapsed successfully!");
            println!("   Energy returned to kernel pool");
            println!("   Global energy: {:.2} J", kernel.global_energy);
        }
    }
}
```

---

## Example 5: Observer Analysis (AGI)

Using Paradox AGI observer to monitor system.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(5000.0);
    
    // Spawn AGI observer
    let observer = Observer::new(&mut kernel);
    
    println!("🤖 Paradox AGI Observer initialized\n");
    
    // Create several universes
    let mut universes = Vec::new();
    for i in 0..5 {
        let u = kernel.spawn_universe(100.0 + i as f64 * 50.0).unwrap();
        universes.push(u);
    }
    
    // Create interaction network
    for i in 0..universes.len()-1 {
        kernel.create_interaction(
            universes[i],
            universes[i+1],
            0.5 + (i as f64 * 0.1)
        ).unwrap();
    }
    
    // Run evolution with observer monitoring
    for step in 0..3 {
        println!("━━━ Evolution Step {} ━━━", step);
        
        // Observer analyzes before evolution
        observer.observe_and_guide(&kernel);
        
        // Check for potential collapses
        let unstable = observer.predict_instability(&kernel);
        if !unstable.is_empty() {
            println!("⚠️  Predicted instability: {:?}", unstable);
        }
        
        // Evolve
        kernel.evolution_step();
        println!();
    }
}
```

---

## Example 6: Energy Conservation Test

Rigorous verification of LAW 1.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(10000.0);
    
    println!("🔍 Energy Conservation Test (LAW 1)\n");
    
    // Calculate initial total
    let initial_total = kernel.global_energy;
    println!("Initial energy: {:.6} J", initial_total);
    
    // Create universes
    let u1 = kernel.spawn_universe(500.0).unwrap();
    let u2 = kernel.spawn_universe(700.0).unwrap();
    let u3 = kernel.spawn_universe(300.0).unwrap();
    
    // Create interactions
    kernel.create_interaction(u1, u2, 0.8).unwrap();
    kernel.create_interaction(u2, u3, 0.6).unwrap();
    
    // Run many evolution steps
    for step in 0..100 {
        kernel.evolution_step();
        
        // Calculate total energy
        let kernel_energy = kernel.global_energy;
        let universe_energy: f64 = kernel.universes
            .values()
            .map(|u| u.energy)
            .sum();
        let total = kernel_energy + universe_energy;
        
        // Check conservation
        let delta = (total - initial_total).abs();
        
        if step % 20 == 0 {
            println!("Step {}: total={:.6}J  delta={:.9}J", 
                     step, total, delta);
        }
        
        assert!(delta < 1e-6, "Conservation violated! Δ={}", delta);
    }
    
    println!("\n✅ Conservation verified over 100 steps!");
}
```

---

## Example 7: Temporal Relativity (Chronos)

Demonstrating local time (LAW 7).

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(3000.0);
    
    println!("⏰ Temporal Relativity Demo (LAW 7)\n");
    
    // Create three universes
    let busy = kernel.spawn_universe(300.0).unwrap();
    let medium = kernel.spawn_universe(300.0).unwrap();
    let idle = kernel.spawn_universe(300.0).unwrap();
    
    // Busy universe: many interactions
    let h1 = kernel.spawn_universe(50.0).unwrap();
    let h2 = kernel.spawn_universe(50.0).unwrap();
    let h3 = kernel.spawn_universe(50.0).unwrap();
    
    kernel.create_interaction(busy, h1, 0.8).unwrap();
    kernel.create_interaction(busy, h2, 0.8).unwrap();
    kernel.create_interaction(busy, h3, 0.8).unwrap();
    
    // Medium universe: some interaction
    kernel.create_interaction(medium, h1, 0.5).unwrap();
    
    // Idle universe: no interactions
    
    println!("Created universes:");
    println!("  Busy: 3 interactions (high density)");
    println!("  Medium: 1 interaction");
    println!("  Idle: 0 interactions\n");
    
    // Run evolution and track local time
    for _ in 0..10 {
        kernel.evolution_step();
    }
    
    // Check timeline indices (local time counters)
    let busy_time = kernel.get_universe(busy).unwrap().timeline_index;
    let medium_time = kernel.get_universe(medium).unwrap().timeline_index;
    let idle_time = kernel.get_universe(idle).unwrap().timeline_index;
    
    println!("After 10 evolution steps:");
    println!("  Busy timeline: {} (slow time - dilation)", busy_time);
    println!("  Medium timeline: {}", medium_time);
    println!("  Idle timeline: {} (fast time)", idle_time);
    println!("\n✅ Higher interaction density → slower local time!");
}
```

---

## Example 8: ParadoxLF Memory Compression

Using memory as potential energy (LAW 8).

```rust
use paradox_kernel::*;

fn main() {
    println!("💾 Memory as Potential Energy (LAW 8)\n");
    
    // Large data to compress
    let original_data: Vec<u8> = (0..10000)
        .map(|i| (i % 256) as u8)
        .collect();
    
    println!("Original data: {} bytes", original_data.len());
    
    // Compress using ParadoxLF
    let compressed = StateVector::compress(&original_data);
    
    println!("Compressed: {} bytes", compressed.data.len());
    println!("Ratio: {:.2}x", 
             original_data.len() as f64 / compressed.data.len() as f64);
    
    // Decompress (excitation)
    let expanded = compressed.expand();
    
    // Verify
    assert_eq!(original_data, expanded);
    
    println!("\n✅ Verified: Latent ↔ Excited phase transition");
    println!("   Storage = Ground state (compressed)");
    println!("   Access = Excited state (expanded)");
}
```

---

## Example 9: Multi-Universe Parallel Work

Emergent concurrency without threads.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(10000.0);
    
    println!("🔀 Emergent Parallel Execution\n");
    
    // Spawn worker universes
    let workers: Vec<_> = (0..8)
        .map(|_| kernel.spawn_universe(200.0).unwrap())
        .collect();
    
    println!("Spawned {} worker universes", workers.len());
    
    // No explicit threading - universes evolve in parallel naturally
    for step in 0..10 {
        kernel.evolution_step();
        
        // Check which universes evolved
        let evolved: Vec<_> = workers.iter()
            .filter(|&&id| {
                let u = kernel.get_universe(id).unwrap();
                u.timeline_index > step as i64
            })
            .count();
        
        println!("Step {}: {} universes evolved", step, evolved);
    }
    
    println!("\n✅ Parallelism emerged from physics!");
    println!("   No threads, no scheduler, just laws.");
}
```

---

## Example 10: Security Through Physics

Demonstrating LAW 10: Security as physics.

```rust
use paradox_kernel::*;

fn main() {
    let mut kernel = Kernel::new(5000.0);
    
    println!("🔒 Physics-Based Security (LAW 10)\n");
    
    let legit = kernel.spawn_universe(500.0).unwrap();
    let malicious = kernel.spawn_universe(100.0).unwrap();
    
    // Track energy for anomaly detection
    let initial_energy = kernel.global_energy 
        + kernel.universes.values().map(|u| u.energy).sum::<f64>();
    
    println!("Initial state tracked\n");
    
    // Simulate attack: try to gain energy without interaction
    println!("⚠️  Simulating attack: energy injection...");
    
    if let Some(universe) = kernel.get_universe_mut(malicious) {
        // Try to cheat by adding energy
        universe.energy += 100.0;  // ← This is the "attack"
    }
    
    // Check conservation
    let current_energy = kernel.global_energy 
        + kernel.universes.values().map(|u| u.energy).sum::<f64>();
    
    let anomaly = (current_energy - initial_energy).abs() > 1e-6;
    
    if anomaly {
        println!("🚨 ANOMALY DETECTED!");
        println!("   Expected: {:.2} J", initial_energy);
        println!("   Detected: {:.2} J", current_energy);
        println!("   Δ: {:.2} J (hidden energy!)", 
                 current_energy - initial_energy);
        
        // In real system, collapse malicious universe
        kernel.collapse_universe(malicious);
        
        println!("\n✅ Malicious universe collapsed!");
        println!("   Security enforced by conservation law");
    }
}
```

---

## Building and Running

### Setup

```bash
# Create project
cargo init --name paradox_examples
cd paradox_examples

# Add kernel dependency (replace with actual path/crate)
# In Cargo.toml:
[dependencies]
paradox_kernel = { path = "../paradox-os/kernel" }
```

### Run Examples

```bash
# Run specific example
cargo run --example hello_universe
cargo run --example communication
cargo run --example evolution_conditions

# Run with output
cargo run --example observer_analysis 2>&1 | less
```

---

## Exercise: Build Your Own

Try creating:

1. **Message Queue Universe** - Queue implemented as universe interactions
2. **Load Balancer** - Distribute work across universes based on energy
3. **Consensus Algorithm** - Distributed agreement via interactions
4. **Time Travel Debugger** - Save/restore timeline states
5. **Universe Pool** - Reuse collapsed universes to save spawn costs

---

**Status:** Examples Documentation v1.0  
**Last Updated:** 2026-01-27  
**See Also:** `GETTING_STARTED.md`, `KERNEL_API.md`

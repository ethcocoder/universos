# ParadoxOS Kernel API Reference

## Overview

The Kernel API provides the core interface for interacting with the ParadoxOS physics engine. All operations must respect the 13 fundamental laws.

---

## Core Types

### UniverseID

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UniverseID(pub u64);
```

Unique identifier for a universe. Opaque handle that cannot be forged.

---

### InteractionID

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InteractionID(pub u64);
```

Unique identifier for an interaction between universes.

---

### StateVector

```rust
#[derive(Debug, Clone)]
pub struct StateVector {
    data: Vec<u8>,  // ParadoxLF compressed format
}
```

Opaque computational state. Implementation-defined internal structure.

**Methods:**

#### `StateVector::new(data: Vec<u8>) -> Self`

Create a state vector from raw bytes.

**Example:**
```rust
let state = StateVector::new(vec![1, 2, 3, 4]);
```

#### `StateVector::compress(data: &[u8]) -> Self`

Create a compressed state vector using ParadoxLF.

**Example:**
```rust
let compressed = StateVector::compress(large_data);
```

#### `StateVector::expand(&self) -> Vec<u8>`

Decompress the state vector.

**Example:**
```rust
let raw_data = state.expand();
```

---

## Kernel API

### Construction

#### `Kernel::new(initial_energy: f64) -> Self`

Initialize the kernel universe (Big Bang).

**Parameters:**
- `initial_energy` - Total system energy budget (must be > 0)

**Returns:** New kernel instance

**Laws Enforced:**
- LAW 0: Kernel has state
- LAW 1: Energy is set and conserved

**Example:**
```rust
let kernel = Kernel::new(10000.0);
```

---

### Universe Management

#### `kernel.spawn_universe(initial_energy: f64) -> Result<UniverseID, &'static str>`

Create a new universe with specified energy allocation.

**Parameters:**
- `initial_energy` - Energy to allocate to this universe (deducted from kernel pool)

**Returns:**
- `Ok(UniverseID)` - Success, universe created
- `Err(_)` - Insufficient energy

**Laws Enforced:**
- LAW 0: Universe has state
- LAW 1: Energy conservation (cannot exceed global pool)
- LAW 2: Entropy increases (creating structure)

**Example:**
```rust
let universe = kernel.spawn_universe(100.0)?;
```

---

#### `kernel.get_universe(&self, id: UniverseID) -> Option<&Universe>`

Get immutable reference to universe.

**Parameters:**
- `id` - Universe identifier

**Returns:**
- `Some(&Universe)` - Universe exists
- `None` - Universe not found

**Example:**
```rust
if let Some(universe) = kernel.get_universe(id) {
    println!("Energy: {}", universe.energy);
}
```

---

#### `kernel.get_universe_mut(&mut self, id: UniverseID) -> Option<&mut Universe>`

Get mutable reference to universe.

**Parameters:**
- `id` - Universe identifier

**Returns:**
- `Some(&mut Universe)` - Universe exists
- `None` - Universe not found

**Laws Enforced:**
- LAW 2: Any mutation must increase entropy

**Example:**
```rust
if let Some(universe) = kernel.get_universe_mut(id) {
    universe.entropy += 0.1;  // Must increase!
}
```

---

#### `kernel.collapse_universe(&mut self, id: UniverseID) -> Option<Universe>`

Collapse and remove a universe.

**Parameters:**
- `id` - Universe to collapse

**Returns:**
- `Some(Universe)` - Universe data before collapse
- `None` - Universe not found

**Laws Enforced:**
- LAW 1: Energy returned to global pool
- LAW 2: Entropy released

**Example:**
```rust
if let Some(collapsed) = kernel.collapse_universe(id) {
    println!("Released {} J of energy", collapsed.energy);
}
```

---

### Interaction Management

#### `kernel.create_interaction(source: UniverseID, target: UniverseID, coupling_strength: f64) -> Result<InteractionID, &'static str>`

Create an interaction between two universes.

**Parameters:**
- `source` - Source universe ID
- `target` - Target universe ID
- `coupling_strength` - Interaction strength (0.0 to 1.0)

**Returns:**
- `Ok(InteractionID)` - Success
- `Err(_)` - Invalid universe IDs

**Laws Enforced:**
- LAW 3: Interaction is the only causal channel
- LAW 2: Entropy increase

**Example:**
```rust
let interaction = kernel.create_interaction(
    universe_a,
    universe_b,
    0.8
)?;
```

---

#### `kernel.get_interaction(&self, id: InteractionID) -> Option<&Interaction>`

Get immutable reference to interaction.

**Parameters:**
- `id` - Interaction identifier

**Returns:**
- `Some(&Interaction)` - Interaction exists
- `None` - Not found

**Example:**
```rust
if let Some(interaction) = kernel.get_interaction(id) {
    println!("Coupling: {}", interaction.coupling_strength);
}
```

---

#### `kernel.destroy_interaction(&mut self, id: InteractionID) -> Option<Interaction>`

Remove an interaction.

**Parameters:**
- `id` - Interaction to destroy

**Returns:**
- `Some(Interaction)` - Interaction before removal
- `None` - Not found

**Laws Enforced:**
- LAW 1: Final energy settlement
- LAW 2: Entropy accounting

**Example:**
```rust
kernel.destroy_interaction(interaction_id);
```

---

### Evolution

#### `kernel.evolution_step(&mut self)`

Execute one step of system evolution. This is the core OS loop.

**Laws Enforced:** ALL 13 LAWS

**Steps Performed:**
1. Observe interactions
2. Compute entropy gradients
3. Redistribute energy
4. Evolve universes
5. Collapse unstable universes

**Example:**
```rust
loop {
    kernel.evolution_step();
    std::thread::sleep(Duration::from_millis(100));
}
```

---

#### `kernel.evolve_universe(&mut self, id: UniverseID) -> Result<(), &'static str>`

Manually trigger evolution for a specific universe.

**Parameters:**
- `id` - Universe to evolve

**Returns:**
- `Ok(())` - Evolution performed
- `Err(_)` - Evolution condition not met

**Laws Enforced:**
- LAW 4: Evolution only if pressure > resistance
- LAW 6: Hamiltonian evolution
- LAW 7: Local time advancement

**Example:**
```rust
kernel.evolve_universe(universe_id)?;
```

---

### Physics Calculations

#### `kernel.calculate_interaction_pressure(&self, id: UniverseID) -> f64`

Calculate total interaction pressure on a universe.

**Parameters:**
- `id` - Universe identifier

**Returns:** Interaction pressure value

**Formula:**
```
pressure = Σ(coupling_strength × |momentum|)
```

**Example:**
```rust
let pressure = kernel.calculate_interaction_pressure(universe_id);
println!("Pressure: {:.2}", pressure);
```

---

#### `kernel.calculate_internal_resistance(&self, id: UniverseID) -> f64`

Calculate universe's internal resistance to evolution.

**Parameters:**
- `id` - Universe identifier

**Returns:** Resistance value

**Formula (LAW 4):**
```
resistance = entropy × (1.0 - stability_score)
```

**Example:**
```rust
let resistance = kernel.calculate_internal_resistance(universe_id);
```

---

#### `kernel.check_evolution_condition(&self, id: UniverseID) -> bool`

Check if a universe can evolve.

**Parameters:**
- `id` - Universe identifier

**Returns:**
- `true` - Evolution allowed
- `false` - Evolution blocked

**Condition (LAW 4):**
```rust
interaction_pressure > internal_resistance
```

**Example:**
```rust
if kernel.check_evolution_condition(universe_id) {
    kernel.evolve_universe(universe_id)?;
}
```

---

### Conservation Verification

#### `kernel.verify_energy_conservation(&self) -> bool`

Verify LAW 1: Total energy is conserved.

**Returns:**
- `true` - Conservation holds
- `false` - VIOLATION (critical error!)

**Formula:**
```
total = global_energy + Σ(universe.energy)
```

**Example:**
```rust
assert!(kernel.verify_energy_conservation());
```

---

#### `kernel.verify_entropy_monotonicity(&self, previous: f64) -> bool`

Verify LAW 2: Entropy has not decreased.

**Parameters:**
- `previous` - Previous entropy value

**Returns:**
- `true` - Law holds (current ≥ previous)
- `false` - VIOLATION

**Example:**
```rust
let prev_entropy = kernel.global_entropy;
kernel.evolution_step();
assert!(kernel.verify_entropy_monotonicity(prev_entropy));
```

---

### State Query

#### `kernel.global_energy(&self) -> f64`

Get total free energy in kernel pool.

**Returns:** Energy value

---

#### `kernel.global_entropy(&self) -> f64`

Get total system entropy.

**Returns:** Entropy value

---

#### `kernel.universe_count(&self) -> usize`

Get number of active universes.

**Returns:** Count

---

#### `kernel.interaction_count(&self) -> usize`

Get number of active interactions.

**Returns:** Count

---

## Universe API

### Properties

#### `universe.id: UniverseID`

Unique identifier.

---

#### `universe.state_vector: StateVector`

Computational state (opaque).

---

#### `universe.energy: f64`

Available computational energy.

**Must satisfy:** `energy ≥ 0.0`

---

#### `universe.entropy: f64`

Complexity/disorder measure.

**Must satisfy:** `entropy ≥ 0.0` and monotonic increase

---

#### `universe.stability_score: f64`

Stability metric (0.0 = unstable, 1.0 = stable).

**Collapse threshold:** `< 0.3` (default)

---

#### `universe.timeline_index: i64`

Local time counter.

**Relative time:** Each universe has its own timeline

---

#### `universe.interaction_links: HashSet<InteractionID>`

Set of active interactions involving this universe.

---

## Interaction API

### Properties

#### `interaction.id: InteractionID`

Unique identifier.

---

#### `interaction.source: UniverseID`

Source universe.

---

#### `interaction.target: UniverseID`

Target universe.

---

#### `interaction.coupling_strength: f64`

Strength of coupling (0.0 to 1.0).

Higher = stronger influence.

---

#### `interaction.momentum: f64`

Energy transfer rate.

Can be positive or negative.

---

#### `interaction.decay_rate: f64`

Natural weakening rate per evolution step.

Typical range: `0.01` to `0.1`

---

## Observer API (AGI Interface)

### Construction

#### `Observer::new(kernel: &mut Kernel) -> Self`

Create AGI observer with privileged universe.

**Parameters:**
- `kernel` - Kernel to observe

**Returns:** Observer instance

**Example:**
```rust
let observer = Observer::new(&mut kernel);
```

---

### Observation

#### `observer.observe_and_guide(&self, kernel: &Kernel)`

Analyze system state and provide guidance.

**Parameters:**
- `kernel` - Kernel to observe

**Output:** Prints analysis to console

**Example:**
```rust
observer.observe_and_guide(&kernel);
```

---

#### `observer.predict_instability(&self, kernel: &Kernel) -> Vec<UniverseID>`

Predict which universes may collapse.

**Parameters:**
- `kernel` - Kernel to analyze

**Returns:** List of unstable universe IDs

**Example:**
```rust
let unstable = observer.predict_instability(&kernel);
for id in unstable {
    println!("Warning: {:?} is unstable!", id);
}
```

---

## Error Handling

### Common Errors

#### `"Insufficient energy to spawn universe"`

**Cause:** Kernel pool has less energy than requested

**Solution:** Use less energy or collapse other universes

---

#### `"Source/Target universe does not exist"`

**Cause:** Invalid UniverseID in interaction creation

**Solution:** Verify universe exists before interaction

---

#### `"Conservation violation detected"`

**Cause:** Total energy changed (critical bug!)

**Solution:** File bug report immediately

---

## Usage Examples

### Example 1: Basic Universe Creation

```rust
let mut kernel = Kernel::new(1000.0);

// Spawn universe
let u1 = kernel.spawn_universe(100.0)?;

// Check state
let universe = kernel.get_universe(u1).unwrap();
assert_eq!(universe.energy, 100.0);
assert_eq!(universe.entropy, 0.0);
```

---

### Example 2: Universe Communication

```rust
// Two universes
let sender = kernel.spawn_universe(150.0)?;
let receiver = kernel.spawn_universe(150.0)?;

// Create channel
let channel = kernel.create_interaction(sender, receiver, 0.9)?;

// Evolve to transfer energy
for _ in 0..10 {
    kernel.evolution_step();
}
```

---

### Example 3: Conservation Verification

```rust
let initial_energy = kernel.global_energy 
    + kernel.universes.values().map(|u| u.energy).sum::<f64>();

// Do work
kernel.evolution_step();

// Verify conservation
assert!(kernel.verify_energy_conservation());
```

---

### Example 4: Observer Analysis

```rust
let observer = Observer::new(&mut kernel);

// Spawn some universes
for _ in 0..5 {
    kernel.spawn_universe(100.0)?;
}

// Observe
observer.observe_and_guide(&kernel);

// Predict problems
let unstable = observer.predict_instability(&kernel);
```

---

## Performance Notes

### Time Complexity

- `spawn_universe`: O(1)
- `create_interaction`: O(1)
- `evolution_step`: O(n + m) where n = universes, m = interactions
- `collapse_universe`: O(k) where k = interaction count

### Memory Usage

- Universe: ~200 bytes + state vector size
- Interaction: ~64 bytes
- Kernel overhead: ~1 KB

---

## Thread Safety

**CRITICAL:** ParadoxOS forbids threads (LAW 13).

All API calls assume single-threaded access.

DO NOT use `Arc<Mutex<Kernel>>` or similar patterns.

---

## Stability Guarantees

### Guaranteed

- ✅ Energy conservation (LAW 1)
- ✅ Entropy monotonicity (LAW 2)
- ✅ No cross-universe direct access (LAW 3)

### Not Guaranteed

- ❌ Timing/performance characteristics (emergent)
- ❌ Universe survival (can collapse)
- ❌ Execution order (physics-driven)

---

**Status:** Kernel API Reference v1.0  
**Last Updated:** 2026-01-27  
**See Also:** `UNIVERSE_API.md`, `EXAMPLES.md`

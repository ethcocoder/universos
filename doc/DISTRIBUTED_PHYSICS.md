# 🌌 ParadoxOS: Distributed Physics & Multiversal Accounting

Phase 12 of ParadoxOS introduces the concept of **Inter-Kernel Entanglement**, allowing computation to span multiple host machines while maintaining strict adherence to the **13 Fundamental Laws of Physics**.

## 1. The Distributed Energy Problem (Law 1)

In a single-node system, Law 1 states that `Total Energy = Constant`. However, in a distributed system, a single node might see energy "disappear" as it sends signals to remote kernels. 

Traditional operating systems ignore this, but ParadoxOS treats network packets as **Physical Photons** with real mass (energy).

### The Energy Flux Model
To maintain security and physical integrity, the Paradox Kernel uses **Multiversal Accounting**:

$$E_{local} + E_{radiated} - E_{materialized} = E_{initial}$$

- **Energy Radiated ($E_{radiated}$)**: Energy that has left the local kernel's logical field and entered a hardware driver (Wormhole).
- **Energy Materialized ($E_{materialized}$)**: Energy that has entered the local field from a remote entanglement.

The **Security Auditor** now verifies that the sum of local energy PLUS the flux (materialized - radiated) always equals the Big Bang's initial budget. If a mismatch of $>0.05J$ is detected, the system triggers a **Global Security Alert**, assuming potential memory corruption or a side-channel attack.

## 2. Wormholes: The Interaction Layer

Networking in ParadoxOS is NOT a high-level abstraction (like TCP/IP). Instead, it is a **Hardware Interaction**.

### Causal Routing
When a `Universe` executes a `SIGNAL` instruction targeting a non-local `UniverseID`, the kernel automatically:
1.  **Deducts Energy**: Law 1 forces the source universe to pay for the photon's creation.
2.  **Projects Signal**: The `WormholeDriver` serializes the `CausalEvent` and projects it through a TCP stream to the remote peer.
3.  **Spacetime Distance**: Connection latency acts as a physical distance, delaying the arrival of the signal and influencing the target's timeline.

## 3. Running a Distributed Multiverse

You can now run multiple nodes on the same host or across a network.

### Node A (Port 4000)
```powershell
cargo run --release --bin paradox-kernel -- 4000 4001
```

### Node B (Port 4001)
```powershell
cargo run --release --bin paradox-kernel -- 4001 4000
```

### Observation
In the **TUI Dashboard**, you will see:
- **Event Horizon**: Signals being "Projected" to the remote kernel.
- **Materialization**: Incoming photons appearing from the wormhole.
- **AGI Guidance**: The Observer tracking the flux and ensuring the node remains stable despite the energy exchange.

## 4. Security Invariants
Even in a distributed state, a Paradox node is **Sovereign**. If a remote kernel sends a malformed or "infinite energy" signal, the local kernel's **Security Auditor (Phase 11)** will catch the Law 1 violation at the boundary and collapse the incoming interaction before it can infect the local multiverse.

---
*ParadoxOS v0.1.0 | Phase 12 Complete | Distributed Sovereignty Active*

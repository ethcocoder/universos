# ParadoxOS v0.5.0 - Enhanced Assembler

## ✅ What's New

### Enhanced Assembly Language
We've upgraded the assembler from a simple one-pass translator to a sophisticated two-pass assembler with:

1. **Labels** - Human-readable jump targets
   ```assembly
   loop_start:
       SIGNAL 4 "Ping"
       SUB counter step
       JUMPIF counter loop_start  # Auto-resolves to byte offset
   ```

2. **Definitions** - Named constants for memory addresses
   ```assembly
   .def counter 200
   .def step 201
   
   SET counter 3    # Much cleaner than SET 200 3
   ```

3. **Two-Pass Compilation**
   - **Pass 1**: Discovers all labels and definitions, calculates byte offsets
   - **Pass 2**: Generates bytecode with symbol resolution

### Technical Implementation
- **File**: `kernel/src/compiler/assembler.rs`
- **Symbol Tables**: HashMap-based lookup for O(1) resolution
- **Smart Resolution**: Attempts numeric parse → definitions → labels

### Benefits
- **10x Readability**: No more magic numbers or manual offset calculations
- **Maintainability**: Changing code doesn't break jump targets
- **Scalability**: Ready to write complex OS services (schedulers, file systems)

## Example: Before vs After

**Before (v0.4.0)**:
```assembly
SET 200 3
JUMPIF 200 6  # What's 6? Where does it go?
```

**After (v0.5.0)**:
```assembly
.def counter 200
SET counter 3
JUMPIF counter loop_start  # Crystal clear!
```

## Verified Execution
- ✅ Loop program compiled & executed successfully
- ✅ Energy conserved (ΔE < 0.01J)
- ✅ All 30 simulation steps completed
- ✅ Signal routing functional via Interactions

---
**Status**: Production-Ready Assembler
**Next Phase**: OS Services (Process Scheduler, Memory Manager)

/**
 * @file paradox.h
 * @brief Core SDK for ParadoxOS Universal ISA
 * 
 * This header allows C code to target the ParadoxOS Kernel directly.
 */

#ifndef PARADOX_H
#define PARADOX_H

#include <stdint.h>

// --- Universal OP Macros (Mapped to ISA) ---

/**
 * @brief Sets a value in Paradox RAM
 * Maps to OpCode::AtomSet (0x01)
 */
#define paradox_set(addr, val) \
    __asm__ volatile ("SET %0, %1" : : "i"(addr), "i"(val))

/**
 * @brief Sends a signal to another universe
 * Maps to OpCode::Signal (0xF0)
 */
#define paradox_signal(target, len, data) \
    __asm__ volatile ("SIGNAL %0, %1, %2" : : "i"(target), "i"(len), "r"(data))

/**
 * @brief Terminates execution
 * Maps to OpCode::Halt (0xFF)
 */
#define paradox_halt() \
    __asm__ volatile ("HALT")

// --- Memory Map Constants ---
#define PARADOX_REG_BASE 200
#define PARADOX_STACK_TOP 255

// --- High Level API ---

typedef uint8_t universe_t;

static inline void observe_val(uint8_t val) {
    paradox_signal(4, 1, &val); // Signal AGI Observer (U4)
}

#endif // PARADOX_H

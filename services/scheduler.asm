# ParadoxOS Universe Scheduler
# Manages round-robin execution of registered universes
# 
# Memory Layout:
# 0-9:     Code section
# 10-19:   Process queue (up to 10 universes)
# 20:      Queue length
# 21:      Current index
# 22:      Loop counter
# 200-210: Working registers

.def queue_start 10
.def queue_len 20
.def current_idx 21
.def loop_counter 22
.def temp 200

# Initialize
SET queue_len 0
SET current_idx 0
SET loop_counter 0

scheduler_loop:
    # Check if we have any processes
    CMP queue_len 0 temp
    JUMPIF temp end_scheduler
    
    # Get current process ID from queue
    # Calculate address: queue_start + current_idx
    SET temp queue_start
    ADD temp current_idx
    
    # Send "tick" signal to that universe
    # (In real implementation, would read from memory[temp])
    # For now, hard-code a few known universes
    
    # Advance to next process
    ADD current_idx 1
    
    # Wrap around if needed
    CMP current_idx queue_len temp
    JUMPIF temp reset_idx
    JUMP continue_loop
    
reset_idx:
    SET current_idx 0
    
continue_loop:
    # Small delay (just increment counter)
    ADD loop_counter 1
    
    # Send heartbeat signal
    SIGNAL 1 "Scheduler Active"
    
    JUMP scheduler_loop

end_scheduler:
    SIGNAL 1 "Scheduler Terminated"
    HALT

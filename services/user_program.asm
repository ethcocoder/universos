# Example User Program
# Simple application that runs under the scheduler
#
# This program counts and reports its progress

.def counter 200
.def max_count 10
.def temp 201

# Initialize
SET counter 0

work_loop:
    # Do some "work"
    ADD counter 1
    
    # Report progress
    SIGNAL 1 "Working"
    
    # Check if done
    CMP counter max_count temp
    JUMPIF temp work_done
    
    # Continue working
    JUMP work_loop

work_done:
    SIGNAL 1 "Work Complete"
    HALT

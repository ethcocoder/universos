# ParadoxOS Message Router
# Routes signals between universes with priority queuing
#
# Memory Layout:
# 0-9:     Code section
# 10-59:   Message queue (5 slots x 10 bytes each)
# 60:      Queue head
# 61:      Queue tail
# 62:      Queue count
# 200-210: Working registers

.def queue_start 10
.def queue_head 60
.def queue_tail 61
.def queue_count 62
.def max_queue 5
.def temp 200
.def msg_energy 201

# Initialize
SET queue_head 0
SET queue_tail 0
SET queue_count 0

router_loop:
    # Check for incoming messages (simulated)
    # In real implementation, would receive signals
    
    # Check if queue has messages
    CMP queue_count 0 temp
    JUMPIF temp route_message
    
    # Queue empty - send idle signal
    SIGNAL 1 "Router Idle"
    JUMP router_loop

route_message:
    # Get message from queue head
    SET temp queue_start
    ADD temp queue_head
    
    # Process message (simplified: just forward it)
    # In real version would read destination from memory
    
    # Decrement queue count
    SUB queue_count 1
    
    # Advance head pointer
    ADD queue_head 1
    CMP queue_head max_queue temp
    JUMPIF temp reset_head
    JUMP continue_routing
    
reset_head:
    SET queue_head 0
    
continue_routing:
    SIGNAL 1 "Message Routed"
    JUMP router_loop

HALT

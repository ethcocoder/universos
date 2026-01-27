# ParadoxOS Coordinated Services Demo
# Phase 8: Inter-Service Communication
#
# Workflow:
# 1. User submits work to Scheduler
# 2. Scheduler dispatches to Router
# 3. Router processes and notifies Monitor
# 4. Monitor tracks completion

.def work_id 200
.def status 201
.def temp 202

# Submit work request
SET work_id 42
SET status 0

# Send work to Scheduler (Universe 4)
SIGNAL 4 "WorkRequest:42"

# Wait for processing (simulate)
SET temp 10
wait_loop:
    SUB temp 1
    JUMPIF temp wait_loop

# Check status from Monitor
SIGNAL 6 "StatusQuery"

# Mark complete
SET status 1
HALT

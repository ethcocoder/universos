# ParadoxOS Assembly (Compiled from Parala)
.def temp0 100
.def temp1 101
.def temp2 102
# Universe: SimpleLoop
    SET temp0 0
    .def i 200
    COPY temp0 i 1
L_12:
    COPY i temp0 1
    SET temp1 3
    COPY temp1 150 1
    SUB 150 temp0
    COPY 150 temp0 1
    JUMPIF temp0 L_32
    JUMP L_64
L_32:
    # Hash: Unknown Op OP_STABILIZE
    COPY i temp0 1
    SIGNAL 4 "Observed"
    COPY i temp0 1
    SET temp1 1
    ADD temp0 temp1
    COPY temp0 i 1
    JUMP L_12
L_64:
    HALT
    HALT

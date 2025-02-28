.data

dat: 
    .word 3, 4, 5

.text addiu $t0 $0 5

add $t1 $t0 $t0

slt $t2 $t0 $t1 
slti $t3 $t0 0  
slti $t4 $t0 9 

        bne $0 $t3 lbl1 
bne $0 $t1 lbl2
    beq $0 $t4 exit

lbl1: 

addiu $v0 $0 -1

j exit

lbl2: 
    addiu $v0 $0 5


exit: halt
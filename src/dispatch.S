
.align 2
.thumb_func
.global dispatch_entry

#define SIO_CPUID           (0xD0000000)
#define SPINLOCK_KERNEL     (0xD000010C)
#define SRAM4_END           (0x2003F000) 
#define SRAM5_END           (0x20040000)

dispatch_entry:
  mov     r0, #1
  msr     primask, r0  // disable interrupt

  // lock spinlock
  ldr     r0, =0xD000010C
disp_005:
  ldr     r1, [r0]
  cmp     r1, #0
  beq     disp_005
  dmb

  // save context of executing task to stack 
  push    {r4-r7}
  mov     r0, r8
  mov     r1, r9
  mov     r2, r10
  mov     r3, r11
  push    {r0-r3}

  ldr     r0, =0xD0000000
  ldr     r3, [r0]
  lsl     r3, #2             // R3 = CPU_ID * 4

  mov     r0, #1
  // disp_running[CPU_CORE] = 1
  ldr     r1, =disp_running
  str     r0, [r1, r3]

  // confirm executing task(cur_task) 
  ldr     r0, =cur_task    // R0 = CUR_TASK
  ldr     r1, [r0, r3]     // get pointer to cur_task
  cmp     r1, #0
  // if cur_task[CPU_CORE] == NULL, then jump to disp_010
  beq     disp_010

  // save stack pointer to TCB which cur_task points
  mov     r2, sp
  str     r2, [r1]    

disp_010:
  // confirm schedule task(sche_task)
  ldr     r1, =sche_task  // R1 = SCHE_TASK
  ldr     r2, [r1, r3]
  cmp     r2, #0
  bne     disp_030
  
  // else cur_task = sche_task 
  str     r2, [r0, r3]   // CUR_TASK = NULL

disp_020:
  cmp     r3, #0
  beq     disp_022
  ldr     r2, =0x20040000
  b       disp_024
disp_022:
  ldr     r2, =0x2003F000
disp_024:
  mov     sp, r2     // Change to specific stack

disp_026:
  mov     r2, #0
  dmb                // unlock spinlock
  ldr     r4, =0xD000010C
  str     r2, [r4]
  msr     primask, r2   // enable interrupt

  mov     r2, #1
  msr     primask, r2   // disable interrupt
disp_028:
  ldr     r2, [r4]      // get spinlock
  cmp     r2, #0
  beq     disp_028
  dmb

  ldr    r2, [r1, r3]   // check sche_task
  cmp    r2, #0         // sche_task== null?
  beq    disp_026       // idol loop

  // switch task to execute 
disp_030:
  str     r2, [r0, r3]      // cur_task = sche_task 
  ldr     r0, [r2]          // get pointer to TCB which sche_task points 
  mov     sp, r0            // restore stack pointer

  mov     r0, #0
  // disp_running[CPU_CORE] = 0
  ldr     r1, =disp_running
  str     r0, [r1, r3]

  // restore context on stack 
  pop     {r0-r3}
  mov     r11, r3
  mov     r10, r2
  mov     r9, r1
  mov     r8, r0
  pop     {r4-r7}

  // unlock spinlock
  dmb
  ldr     r0, =0xD000010C
  mov     r1, #0
  str     r1, [r0]

  msr     primask, r1       // enable interuupt

  bx      lr


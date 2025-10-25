
use core::arch::asm;
use crate::icc_spin::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::typedef::*;
use crate::vector_table::*;

pub fn icc_sync_core(coreno:u32) {

    if coreno == 1 {
        icc_unl_spin(SPINLOCK_SYNC_C0);
        icc_loc_spin(SPINLOCK_SYNC_C1);
    } else {
        icc_unl_spin(SPINLOCK_SYNC_C1);
        icc_loc_spin(SPINLOCK_SYNC_C0);
    }
}

pub fn icc_wup_core1(vtbl:*mut Vector, sp:*mut u32, ent:FP ) {

    let cmd:[ u32; 6 ] = [
       0,
       0,
       1,
       vtbl as u32,
       sp as u32,
       ent
    ];
    let mut seq:usize = 0;

    loop {
        if cmd[seq] == 0 {
            while in_w(FIFO_ST) & FIFO_ST_VLD != 0  { in_w(FIFO_RD); }
            unsafe {
                asm!("sev");
            }
        }
        while in_w(FIFO_ST) & FIFO_ST_RDY == 0 {}
        out_w(FIFO_WR, cmd[seq]);
        unsafe {
            asm!("sev");
        } 

        while in_w(FIFO_ST) & FIFO_ST_VLD  == 0 {
            unsafe {
                asm!("wfe");
            }
        }

        let res = in_w(FIFO_RD);

        if cmd[seq] == res {
            seq = seq + 1;
        } else {
            seq = 0;
        }
        if seq >= ( core::mem::size_of::<[u32; 6]>() / core::mem::size_of::<u32>() ) {
            break;
        }
    }   
}


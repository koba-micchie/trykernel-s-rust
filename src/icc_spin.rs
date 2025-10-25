
use crate::sysdef::*;
use crate::syslib::*;

// lock spinlock inter core
pub fn icc_loc_spin(no: u32 ) {

   let addr = spinlock(no);
   while in_w(addr) == 0 {}
   memory_barrier();
}

// unlock spinlock inter core
pub fn icc_unl_spin(no: u32 ) {

   memory_barrier();
   let addr = spinlock(no);
   out_w(addr, 0);
}

// initialize all spinlock
pub fn icc_ini_spin() {
   let mut no:u32 = 0;

   while no < 32 {
      icc_unl_spin(no);
      no = no + 1;
   }
}


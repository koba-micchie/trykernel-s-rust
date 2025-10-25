
use core::ptr::{copy_nonoverlapping, write_bytes};

use crate::icc_core::*;
use crate::icc_int::*;
use crate::icc_spin::*;
use crate::logger::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::task::*;
use crate::typedef::*;
use crate::vector_table::*;
use crate::main_c0;

// initialize clock
const XOSC_STARTUP_DELAY: u32 = (XOSC_KHZ + 128) / 256;

// initialize phase locked loop 
fn init_pll(pll: u32, refdiv: usize, vco_freq: usize, post_div1: usize, post_div2: usize) {

    let ref_mhz: u32 = XOSC_MHZ / refdiv as u32;
    let fbdiv: u32 = vco_freq as u32 / (ref_mhz * MHZ);
    let pdiv: u32 = (post_div1 << PLL_PRIM_POSTDIV1_LSB) as u32 | (post_div2 << PLL_PRIM_POSTDIV2_LSB) as u32;

    let pll_reset: u32 ;

    if pll == PLL_USB_BASE {
        pll_reset = 1 << 13;
    } else {
        pll_reset = 1 << 12;
    }
    set_w(RESETS_RESET, pll_reset);
    clr_w(RESETS_RESET, pll_reset);

    out_w(pll + PLL_CS, refdiv as u32);
    out_w(pll + PLL_FBDIV_INT, fbdiv as u32);

    clr_w(pll + PLL_PWR, PLL_PWR_PD | PLL_PWR_VCOPD);
    while (in_w(pll + PLL_CS) & PLL_CS_LOCK) == 0 {}

    out_w(pll + PLL_PRIM, pdiv);
    clr_w(pll + PLL_PWR, PLL_PWR_POSTDIVPD);
}

// config clock 
fn clock_config(clock_kind: usize, auxsrc: u32, src_freq: u32, freq: u32) {
    if freq > src_freq {
        return;
    }

    let clock: u32 = CLOCKS_BASE + (clock_kind * 0xC) as u32;
    let divwk: u64 = (src_freq as u64) << 8;
    let div  : u32 = (divwk / freq as u64) as u32;

    if div > in_w(clock + CLK_X_DIV) {
        out_w(clock + CLK_X_DIV, div);
    }
    clr_w(clock + CLK_X_CTRL, CLK_CTRL_ENABLE);

    out_w(clock + CLK_X_CTRL, (in_w(clock + CLK_X_CTRL) & !CLK_SYS_CTRL_AUXSRC) | (auxsrc << 5));
    set_w(clock + CLK_X_CTRL, CLK_CTRL_ENABLE);
    out_w(clock + CLK_X_DIV, div);
}

// initialize clock
fn init_clock() {
    out_w(CLK_SYS_RESUS_CTRL, 0);

    // config XOSC
    out_w(XOSC_CTRL, XOSC_CTRL_FRANG_1_15MHZ);
    out_w(XOSC_STARTUP, XOSC_STARTUP_DELAY);
    set_w(XOSC_CTRL, XOSC_CTRL_ENABLE);
    while (in_w(XOSC_STATUS) & XOSC_STATUS_STABLE) == 0 {}

    clr_w(CLK_SYS + CLK_X_CTRL, CLK_SYS_CTRL_SRC);
    while in_w(CLK_SYS + CLK_X_SELECTED) != 0x1 {}
    clr_w(CLK_REF + CLK_X_CTRL, CLK_REF_CTRL_SRC);
    while in_w(CLK_REF + CLK_X_SELECTED) != 0x1 {}

    // config PLL
    // PLL SYS 125MHz
    init_pll(PLL_SYS_BASE, 1, (1500 * MHZ) as usize, 6, 2);
    // PLL USB 48MHz
    init_pll(PLL_USB_BASE, 1, ( 480 * MHZ) as usize, 5, 2);

    // config CLK_REF
    let divwk: u64 = ((12 * MHZ) as u64) << 8;
    let div:   u32 = (divwk / (12 * MHZ) as u64) as u32;
 
    if div > in_w(CLK_REF + CLK_X_DIV) {
        out_w(CLK_REF + CLK_X_DIV, div);
    }

    clr_w(CLK_REF + CLK_X_CTRL, CLK_CTRL_ENABLE);
    out_w(CLK_REF + CLK_X_CTRL, in_w(CLK_REF + CLK_X_CTRL) & !CLK_SYS_CTRL_AUXSRC);
    out_w(CLK_REF + CLK_X_CTRL, (in_w(CLK_REF + CLK_X_CTRL) & !CLK_REF_CTRL_SRC) | 2);
    while (in_w(CLK_REF + CLK_X_SELECTED) & (1 << 2)) == 0 {}

    set_w(CLK_REF + CLK_X_CTRL, CLK_CTRL_ENABLE);
    out_w(CLK_REF + CLK_X_DIV, div);

    // config CLK_SYS
    let divwk: u64 = ((125 * MHZ) as u64) << 8;
    let div:   u32 = (divwk / (125 * MHZ) as u64) as u32;

    if div > in_w(CLK_SYS + CLK_X_DIV) {
        out_w(CLK_SYS + CLK_X_DIV, div);
    }
    clr_w(CLK_SYS + CLK_X_CTRL, CLK_REF_CTRL_SRC);
    while (in_w(CLK_SYS + CLK_X_SELECTED) & 0x1) == 0 {}

    out_w(CLK_SYS + CLK_X_CTRL, in_w(CLK_SYS + CLK_X_CTRL) & !CLK_SYS_CTRL_AUXSRC);
    out_w(CLK_SYS + CLK_X_CTRL, (in_w(CLK_SYS + CLK_X_CTRL) & !CLK_REF_CTRL_SRC) | 1);
    while (in_w(CLK_SYS + CLK_X_SELECTED) & (1 << 1)) == 0 {}

    set_w(CLK_SYS + CLK_X_CTRL, CLK_CTRL_ENABLE);
    out_w(CLK_SYS + CLK_X_DIV, div);

    // config CLK_USB
    clock_config(CLK_KIND_USB, 0, 48 * MHZ, 48 * MHZ);
    // config CLK_ADC
    clock_config(CLK_KIND_ADC, 0, 48 * MHZ, 48 * MHZ);
    // config CLK_RTC
    clock_config(CLK_KIND_RTC, 0, 48 * MHZ, 46875);
    // config CLK_PERI
    clock_config(CLK_KIND_PERI, 0, 125 * MHZ, 125 * MHZ);
}
//  enable peripheral 
fn init_peri()  {

    // enable GPIO
    clr_w(RESETS_RESET, 1 << 5);
    while (in_w(RESETS_RESET_DONE) & (1 << 5)) == 0 {}

    clr_w(RESETS_RESET, 1 << 8);
    while (in_w(RESETS_RESET_DONE) & (1 << 8)) == 0 {}

    // enable UART0
    clr_w(RESETS_RESET, 1 << 22);
    while (in_w(RESETS_RESET_DONE) & (1 << 22)) == 0 {}

    // enable pin

    // P25=LED on Raspberry Pi Pico on-board
    // disable pin P25
    out_w(GPIO_OE_CLR, 1 << 25);
    // clear pin P25 
    out_w(GPIO_OUT_CLR, 1 << 25);
    // enable P25 SIO
    out_w(gpio_ctrl(25), 5);
    // enable P25 output
    out_w(GPIO_OE_SET, 1 << 25);

    // enable P0=UART0-TX
    out_w(gpio_ctrl(0), 2);
    // enable P1=UART0-RX
    out_w(gpio_ctrl(1), 2);
}

// Initialize system timer
fn init_systim()  {
    // disable SysTick
    out_w(SYST_CSR, SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT);
    // config reload value 
    out_w(SYST_RVR, (TIMER_PERIOD as u32 * TMCLK_KHZ) - 1);
    // config count value 
    out_w(SYST_CVR, (TIMER_PERIOD as u32 * TMCLK_KHZ) - 1);
    // enable SysTick
    out_w(SYST_CSR, SYST_CSR_CLKSOURCE | SYST_CSR_TICKINT | SYST_CSR_ENABLE);
}

// Initialize vector table
fn init_int() {
    let mut no:usize = 0;
 
    // copy vector table for core0, core 1 from flash to ram
    unsafe {
        while no < 42 {
            vector_table_c0[no].reserved = vector_table[no].reserved;
            vector_table_c1[no].reserved = vector_table[no].reserved;
            no = no + 1;
        }
    } 
   
    // set vector table for core 0 
    out_w( SCB_VTOR , core::ptr::addr_of_mut!(vector_table_c0) as u32 );
    out_w( SCB_SHPR3, ( INTLEVEL_0 << 24 ) | (INTLEVEL_3 << 16));
} 

fn init_section() {
    extern "C" {
        static mut __sbss : u8;
        static     __ebss : u8;
        static     __sidata : u8;
        static mut __sdata : u8;
        static     __edata : u8;

     }

     unsafe {
         let count = ((&__ebss as *const u8) as usize) - ((&__sbss as *const u8) as usize);
         write_bytes(&mut __sbss as *mut u8, 0, count);

         let count = ((&__edata as *const u8) as usize) - ((&__sdata as *const u8) as usize);
         copy_nonoverlapping(&__sidata as *const u8, &mut __sdata as *mut u8, count);
     }

}

// reset handler for core 0

#[no_mangle]
pub unsafe extern "C" fn __reset_handler_c0() {
    
    let intsts : isize = di();

    init_clock();
    init_int();
    init_section();
    init_peri();
    init_systim();

    // do not use debug() before initialize spinlock (call icc_ini_spin())
    icc_ini_spin();
    icc_loc_spin(SPINLOCK_SYNC_C0);
    icc_loc_spin(SPINLOCK_SYNC_C1);

    // Wake up core c1 
    let vtbl: *mut Vector = core::ptr::addr_of_mut!(vector_table_c1[0]);
    let sp:*mut u32 = 0x2004_2000 as *mut u32 ;
    let ent:FP  = __reset_handler_c1 as FP;
    icc_wup_core1( vtbl, sp, ent);
    ei(intsts);
    main_c0();
    loop{};

   
}

// reset handler for core 1
#[no_mangle]
pub unsafe extern "C" fn __reset_handler_c1() {

    let intsts : isize = di();

    out_w(SCB_VTOR,  core::ptr::addr_of_mut!(vector_table_c1) as u32 );
    out_w(SCB_SHPR3, INTLEVEL_3 << 16);

    ei(intsts);
    main_c1();

    loop{};
}

pub fn main_c1() {

    debug("Starting main_c1\r\n");

    init_icc_int();
    match icc_def_int(ICCINT_DISPATCH,   iccint_dispatch) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }
    match icc_def_int(ICCINT_ENAINT,     iccint_enableint) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }
    match icc_def_int(ICCINT_DISINT,     iccint_disableint) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }

    icc_sync_core(1);
    debug("Entering main_c1 loop\r\n");
    loop {};  // wait for dispatch

}


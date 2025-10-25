
use core::panic::PanicInfo;
use crate::systimer::*;
use crate::sysdef::*;
use crate::reset_handler::*;

// Define entry of vector table
pub union Vector {
    pub reserved: u32,
    pub handler: unsafe extern "C" fn(),
    pub handler_reset: unsafe extern "C" fn() -> !,
}

// Default handler
#[no_mangle]
pub unsafe extern "C" fn DefaultHandler() {
    loop {}
}


#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_nmi() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_hardfault() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn DefaultHandler_svcall() {
    loop {}
}

// Dispatch handler for pend_sv
extern "C" { fn dispatch_entry(); }

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define Reset vector area for core 0 in linker script
#[no_mangle]
#[link_section = ".vector_table"]
pub static vector_table : [Vector; N_SYSVEC + N_INTVEC] = [
    Vector { reserved: 0x2004_1000 },    // INITIAL_SP
    Vector { handler: __reset_handler_c0  }, // reset handler for core 0
    Vector { handler: DefaultHandler_nmi },  // nmi
    Vector { handler: DefaultHandler_hardfault },  // hard_fault
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { reserved: 0 },              // reserved0 
    Vector { handler: DefaultHandler_svcall },  // svcall
    Vector { reserved: 0 },              // reserved1
    Vector { reserved: 0 },              // reserved1
    Vector { handler: dispatch_entry },  // pend_sv
    Vector { handler: systimer_handler },// sys_tick
    Vector { handler: DefaultHandler },  // irq 0
    Vector { handler: DefaultHandler },  // irq 1
    Vector { handler: DefaultHandler },  // irq 2
    Vector { handler: DefaultHandler },  // irq 3
    Vector { handler: DefaultHandler },  // irq 4
    Vector { handler: DefaultHandler },  // irq 5
    Vector { handler: DefaultHandler },  // irq 6
    Vector { handler: DefaultHandler },  // irq 7
    Vector { handler: DefaultHandler },  // irq 8
    Vector { handler: DefaultHandler },  // irq 9
    Vector { handler: DefaultHandler },  // irq 10
    Vector { handler: DefaultHandler },  // irq 11
    Vector { handler: DefaultHandler },  // irq 12
    Vector { handler: DefaultHandler },  // irq 13
    Vector { handler: DefaultHandler },  // irq 14
    Vector { handler: DefaultHandler },  // irq 15
    Vector { handler: DefaultHandler },  // irq 16
    Vector { handler: DefaultHandler },  // irq 17
    Vector { handler: DefaultHandler },  // irq 18
    Vector { handler: DefaultHandler },  // irq 19
    Vector { handler: DefaultHandler },  // irq 20
    Vector { handler: DefaultHandler },  // irq 21
    Vector { handler: DefaultHandler },  // irq 22
    Vector { handler: DefaultHandler },  // irq 23
    Vector { handler: DefaultHandler },  // irq 24
    Vector { handler: DefaultHandler },  // irq 25
];

// Define Reset vector area in linker script
#[no_mangle]
#[link_section = ".vector_table_c0"]
pub static mut vector_table_c0 : [Vector; N_SYSVEC + N_INTVEC] = [
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
];

#[no_mangle]
#[link_section = ".vector_table_c1"]
pub static mut vector_table_c1 : [Vector; N_SYSVEC + N_INTVEC ] = [
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
    Vector { reserved: 0 },             
];

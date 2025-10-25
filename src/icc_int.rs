use defer_lite::defer;

use crate::apidef::*;
use crate::error::*;
use crate::interrupt::*;
use crate::logger::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::vector_table::*;

pub fn dummy(_data:u32) {
    loop {}
}

pub static mut ICC_INT_TBL:[[fn(data:u32);16]; CPU_CORE_NUM ] = [
    [ dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy ],
    [ dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy, dummy ],
];

// number of IRQ for SIO_IRQ (Proc0,Proc1)
pub const IRQ_SIOPR0:u32 = 15;
pub const IRQ_SIOPR1:u32 = 16;

pub fn init_icc_int() {

    let _dummy: u32  = in_w(FIFO_ST);

    let mut dint = TDint {
       intatr: TA_HLNG,  
       inthdr: Inthdr { handler_p1: icc_int_hdr }, 
    };

    let coreno = get_cpu_coreno() as u32;
    let intno  = if coreno == 1 { IRQ_SIOPR1 + 100} else { IRQ_SIOPR0 };
    match tk_def_int (intno , &mut dint ) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at tk_def_int");
        }
    }

    let intsts: isize = di();
    let intno = if coreno == 1 { IRQ_SIOPR1 } else { IRQ_SIOPR0};
    out_b (nvic_ipr(intno)  as u32, (INTLEVEL_0 << ( 8 - 2 )) as u8 );
    out_w (nvic_iser(intno) as u32, 0x01 << (intno % 32) );
    ei(intsts);

}

#[no_mangle]
pub unsafe extern "C" fn icc_int_hdr(intno:u32) {

    while in_w(FIFO_ST) & FIFO_ST_VLD != 0  {
        let data = in_w(FIFO_RD);
        let data_idx:usize = (data >> 24) as usize;
        let coreno = get_cpu_coreno();
        let fp = ICC_INT_TBL[coreno][data_idx];
        if !core::ptr::fn_addr_eq(fp, dummy as fn(u32)) {
            fp(data);
        }
    }
    out_w(FIFO_ST,0);
}
        
pub fn icc_def_int(intno:usize, inthdr:fn(data:u32) ) -> Result<(),KernelError> {

    if intno >= 16  {
        return Err(KernelError::ID);
    }

    let intsts: isize = di();
    defer! { ei(intsts); } 

    let coreno = get_cpu_coreno();
    unsafe {
        ICC_INT_TBL[coreno][intno] = inthdr;
    }
    Ok(())

}

pub fn icc_ras_int(code:u32) -> Result<(),KernelError> {

    let intsts: isize = di();
    defer! { ei(intsts); }

    if in_w(FIFO_ST) & FIFO_ST_RDY != 0  {
        out_w(FIFO_WR, code);
    } else {
        return Err(KernelError::QOVR);
    }
    Ok(())
}

pub fn iccint_enableint(data:u32) {
    enable_int( data & 0x0000FFFF, (data>>16) & 0x0F );

}

pub fn iccint_disableint(data:u32) {
    disable_int( data & 0x0000FFFF );
}


use defer_lite::defer;

use crate::apidef::*;
use crate::error::*;
use crate::icc_int::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::vector_table::*;

pub static mut KNL_TASKINDP:i32 = 0;

pub union Inthdr {
    pub reserved: u32,
    pub handler_p1: unsafe extern "C" fn(intno:u32),
}

pub static mut INTHDR_TBL: [[Inthdr ; N_INTVEC];CPU_CORE_NUM]  = 
[  
  [
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
  ],
  [
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
    Inthdr { reserved:0 },
  ]

];
 
pub fn enable_int( mut intno:u32, level:u32 ) {

    let coreno = get_cpu_coreno();
    if (intno < 100 && coreno != 0) || (intno >= 100 && coreno != 1) {
        icc_ras_int( (ICCINT_ENAINT<< 24) as u32 | (level << 16) as u32 | intno as u32);
        return;
    }
    if intno >= 100 {
        intno = intno - 100;
    }

    let intsts: isize = di();
    out_b (nvic_ipr(intno)  as u32, (level << ( 8 - 2 )) as u8 );
    out_w (nvic_iser(intno) as u32, 0x01 << (intno % 32) );
    ei(intsts);
}

pub fn disable_int( mut intno:u32 ) {

    let coreno = get_cpu_coreno();
    if (intno < 100 && coreno != 0) || (intno >= 100 && coreno != 1) {
        icc_ras_int( (ICCINT_DISINT<< 24) as u32| intno as u32);
        return;
    }
    if intno >= 100 {
        intno = intno - 100;
    }
    out_w (nvic_icer(intno) as u32, 0x01 << (intno % 32) );
}

pub fn clear_int( intno:u32 ) {
    out_w (nvic_icpr(intno) as u32, 0x01 << (intno % 32) );
}

pub fn tk_def_int( mut intno:u32, pk_dint: &mut TDint ) -> Result<(),KernelError> {

    let mut coreno:u32 = 0;

    if ( intno >= N_INTVEC as u32 && intno < 100) || ( intno >= N_INTVEC as u32 +100)  {
        return Err(KernelError::ID);
    }

    if intno >= 100 {
        coreno = 1;
        intno = intno - 100;
    }
        
    let inthdr = &mut pk_dint.inthdr;

    let mut vector = Vector{ reserved: 0 };

    unsafe {
        if inthdr.reserved != 0  {
            if (*pk_dint).intatr & TA_HLNG != 0 {
                INTHDR_TBL[coreno as usize][intno as usize].handler_p1 = inthdr.handler_p1 ;
                vector.handler = hll_inthdr ;
            }
        } else {
            vector.handler = DefaultHandler ;
        }

        if coreno == 0 {
            vector_table_c0[N_SYSVEC + intno as usize].handler = vector.handler ;
        } else {
            vector_table_c1[N_SYSVEC + intno as usize].handler = vector.handler ;
        }
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn hll_inthdr() {

    let intno  = get_ipsr() - N_SYSVEC ;
    let coreno = get_cpu_coreno();
    let inthdr = INTHDR_TBL[coreno][intno].handler_p1 ;

    unsafe { KNL_TASKINDP = KNL_TASKINDP + 1 };
    inthdr(intno as u32);
    unsafe { KNL_TASKINDP = KNL_TASKINDP - 1 };

}


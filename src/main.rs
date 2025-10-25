#![no_std]
#![no_main]
#![feature(lang_items)]

#[lang = "eh_personality"] extern "C" fn eh_personality() {}

use core::panic;
use core::result::Result::{Ok,Err};


#[path = "apidef.rs" ]
pub mod apidef;
use crate::apidef::*;

#[path = "config.rs" ]
pub mod config;

#[path = "context.rs" ]
pub mod context;

#[path = "error.rs" ]
pub mod error;

#[path = "eventflag.rs" ]
pub mod eventflag;

#[path = "icc_core.rs" ]
pub mod icc_core;
use crate::icc_core::*;

#[path = "icc_int.rs" ]
pub mod icc_int;
use crate::icc_int::*;

#[path = "icc_spin.rs" ]
pub mod icc_spin;

#[path = "interrupt.rs" ]
pub mod interrupt;

#[path = "knldef.rs" ]
pub mod knldef;

#[path = "logger.rs" ]
pub mod logger;
use crate::logger::*;

#[path = "semaphore.rs"]
pub mod semaphore;

#[path = "sysdef.rs" ]
pub mod sysdef;
use crate::sysdef::*;

#[path = "syslib.rs" ]
pub mod syslib;
use crate::syslib::*;

#[path = "systimer.rs" ]
pub mod systimer;

#[path = "task.rs" ]
pub mod task;
use crate::task::*;

#[path = "typedef.rs"]
pub mod typedef;
use crate::typedef::*;

#[path = "app/usermain.rs" ]
pub mod usermain;
use crate::usermain::*;

#[path = "boot/boot2.rs" ]
pub mod boot2;

#[path = "boot/reset_handler.rs" ]
pub mod reset_handler;

#[path = "boot/vector_table.rs" ]
pub mod vector_table;


pub static mut TSKSTK_INI: [u8; 1024] = [0; 1024];

fn initsk() {
    debug("Start initsk()\r\n");
    usermain();
    debug("End Try Kernel\r\n");
    debug("Calling tk_ext_tsk\r\n");
    tk_ext_tsk();
}

pub fn main_c0() {

    init_icc_int();
    match icc_def_int(ICCINT_DISPATCH,  iccint_dispatch) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }

    match icc_def_int(ICCINT_ENAINT,    iccint_enableint) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }

    match icc_def_int(ICCINT_DISINT,    iccint_disableint) {
        Ok(()) => {}
        Err(err) => {
            error(err);
            panic!("failed at icc_def_int");
        }
    }

    // do not call debug() before tm_com_init
    tm_com_init();
    icc_sync_core(0);

    debug("Start Try Kernel\r\n");
    let tskid_ini: ID ;
    let faddr = initsk;

    let mut ctsk_init = TCtsk {
        tskatr: TA_HLNG | TA_RNG0 | TA_USERBUF,
        task:   faddr as u32,
        itskpri:  1, // priority MAX
        stksz:  core::mem::size_of::<[u8; 1024]>() as u32,
        bufptr: unsafe {(&TSKSTK_INI as *const u8) as u32} ,
    };

    debug("Calling tk_cre_tsk\r\n");
    match tk_cre_tsk(&mut ctsk_init) {
       Ok(id)   => {
          tskid_ini = id;
       }
       Err(err) => {
          error(err) ;
          panic!("failed at tk_cre_tsk");
       }
    };
    debug("Returning tk_cre_tsk\r\n");

    debug("Calling tk_sta_tsk\r\n");
    match tk_sta_tsk(tskid_ini) {
       Ok(())      => {}
       Err(err) => {
           error(err);
           panic!("failed at tk_sta_tsk");
       }
    };
    debug("Returning tk_sta_tsk\r\n");

    loop {};
}


use defer_lite::defer;

use core::result::Result;
use core::result::Result::{Ok,Err};

use crate::apidef::*;
use crate::config::*;
use crate::context::*;
use crate::interrupt::*;
use crate::knldef::*;
use crate::logger::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::typedef::*;
use crate::error::*;
use crate::icc_int::*;


// Task Control Block (TCB)
pub static mut TCB_TBL: [ TCB; CNF_MAX_TSK_ID ] = 
  [ 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
    TCB { context: core::ptr::null_mut(), pre: core::ptr::null_mut(), next: core::ptr::null_mut(), state: TSTAT::TsNonexist, tskadr: 0, itskpri: 0, stkadr: 0, stksz: 0, wupcnt: 0, waifct: TWFCT::TwfctNon, waitim: 0, waierr: KernelError::OK, waiptn: 0, wfmode: 0, p_flgptn: 0, waisem: 0 }, 
];

// Task ready queue for each priority
pub static mut READY_QUEUE: [ TcbQueue; CNF_MAX_TSK_PRI ] =
  [ TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
    TcbQueue { head: core::ptr::null_mut() }, 
];

// Task wait queue 
pub static mut WAIT_QUEUE: TcbQueue = TcbQueue { head: core::ptr::null_mut() } ;

// Task in execute (Multicore)
#[no_mangle]
pub static mut cur_task:  [ *mut TCB; CPU_CORE_NUM ] = 
  [ core::ptr::null_mut(),
    core::ptr::null_mut(),
]; 
// Task scheduled  (Multicore)
#[no_mangle]
pub static mut sche_task: [ *mut TCB; CPU_CORE_NUM ] =
  [ core::ptr::null_mut(),
    core::ptr::null_mut(),
];

// dispatch_entry() is in execute
#[no_mangle]
pub static mut disp_running: [ bool; CPU_CORE_NUM ]  = 
  [ false,
    false,
];

// Task create API
pub fn tk_cre_tsk(pk_ctsk: &mut TCtsk) -> Result<ID, KernelError> {
    // Check arguments
    if ((*pk_ctsk).tskatr & !TA_RNG3) != (TA_HLNG | TA_USERBUF) {
        return Err(KernelError::RSATR);
    }
    if ((*pk_ctsk).itskpri <= 0) || ((*pk_ctsk).itskpri > CNF_MAX_TSK_PRI) {
        return Err(KernelError::PAR);
    }
    if (*pk_ctsk).stksz == 0 {
        return Err(KernelError::PAR);
    }

    // begin critical section
    let intsts = begin_cs();
    let mut errcode = KernelError::OK;

    // Search unused TCB
    let mut i: usize = 0;
    while i < CNF_MAX_TSK_ID {
        unsafe {
            if TCB_TBL[i].state == TSTAT::TsNonexist {
                break;
            }
        }
        i = i + 1;
    }

    if i <  CNF_MAX_TSK_ID {
        // Initialize TCB
        unsafe {
            TCB_TBL[i].state = TSTAT::TsDormant;
            TCB_TBL[i].pre   = core::ptr::null_mut();
            TCB_TBL[i].next  = core::ptr::null_mut();

            TCB_TBL[i].tskadr  = (*pk_ctsk).task;
            TCB_TBL[i].itskpri = (*pk_ctsk).itskpri;
            TCB_TBL[i].stksz   = (*pk_ctsk).stksz;
            TCB_TBL[i].stkadr =  (*pk_ctsk).bufptr;
        }     
    } else {
        errcode = KernelError::LIMIT;
    }

    end_cs(intsts);

    if errcode == KernelError::OK {
        Ok(i+1)
    } else {
        Err(errcode)
    }
}


// Task Start API
pub fn tk_sta_tsk(tskid: ID) -> Result<(), KernelError> {

    // Check arguments
    if tskid <= 0 || tskid > CNF_MAX_TSK_ID {
         return Err(KernelError::ID);
    }

    // begin critical section
    let intsts: isize = begin_cs();
    let mut errcode = KernelError::OK;

    unsafe {
        let tcb: *mut TCB = core::ptr::addr_of_mut!(TCB_TBL[tskid - 1]);
        if (*tcb).state == TSTAT::TsDormant {
            // Change state of TCB ready to execute
            (*tcb).state = TSTAT::TsReady;
            (*tcb).context = make_context((*tcb).stkadr, (*tcb).stksz, (*tcb).tskadr);
            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
            schedule();
        } else {
            errcode = KernelError::OBJ;
        }
    }
    end_cs(intsts);
    if errcode == KernelError::OK {
        Ok(())
    } else {
        Err(errcode)
    }
}

// Task Exit API
pub fn tk_ext_tsk() {

    let intsts = begin_cs();

    // Change state of TCB to dormant
    unsafe {  
        let coreno = get_cpu_coreno();
        let task: *mut TCB = cur_task[coreno];
        (*task).state = TSTAT::TsDormant;
        READY_QUEUE[(*task).itskpri].remove_top();
        schedule();
    }
    end_cs(intsts);
}

// Task delay API 
pub fn tk_dly_tsk(dlytim: RELTIM) -> Result<(),KernelError>  {

    let intsts = begin_cs();
    if dlytim > 0 {
        unsafe {
            let coreno = get_cpu_coreno();
            let task: *mut TCB = cur_task[coreno];
            READY_QUEUE[(*task).itskpri].remove_top();
            // Change state of TCB to wait
            (*task).state = TSTAT::TsWait;
            // Set wait factor
            (*task).waifct = TWFCT::TwfctDly;
            // Set wait time
            (*task).waitim = dlytim + TIMER_PERIOD;
            WAIT_QUEUE.add_entry(task);
            schedule();
        }
    }
    end_cs(intsts);
    Ok(())
}

// Task sleep API
pub fn tk_slp_tsk(tmout: TMO) -> Result<(),KernelError> {

    let intsts = begin_cs();

    // Wake up request exists?
    unsafe {
        let coreno = get_cpu_coreno();
        let task: *mut TCB = cur_task[coreno];
        if (*task).wupcnt > 0 {
            (*task).wupcnt -= 1;
        } else {
            // no request
            READY_QUEUE[(*task).itskpri].remove_top();
            // Change state of TCB to wait
            (*task).state = TSTAT::TsWait;
            // Set wait factor
            (*task).waifct = TWFCT::TwfctSlp;
            // Set wait time
            if tmout == TMO_FEVR {
                (*task).waitim = tmout;
            } else {
                (*task).waitim = tmout + TIMER_PERIOD;
            }
            WAIT_QUEUE.add_entry(task);
            schedule();
        }
    }
    end_cs(intsts);
    Ok(())
}

// Task wakeup API
pub fn tk_wup_tsk(tskid: ID) -> Result<(),KernelError> {
    if tskid <= 0 || tskid > CNF_MAX_TSK_ID {
        return Err(KernelError::ID);
    }
    let intsts: isize = begin_cs();
    let mut errcode = KernelError::OK;

    // Does task have wait state by tk_slp_tsk() ?
    unsafe {
        let tcb: *mut TCB = core::ptr::addr_of_mut!(TCB_TBL[tskid - 1]);
        if (*tcb).state == TSTAT::TsWait && (*tcb).waifct == TWFCT::TwfctSlp {
            WAIT_QUEUE.remove_entry(tcb);
            (*tcb).state  = TSTAT::TsReady;
            (*tcb).waifct = TWFCT::TwfctNon;
            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
            schedule();
        } else if (*tcb).state == TSTAT::TsReady || (*tcb).state == TSTAT::TsWait {
            // Increase number of wake up request 
            (*tcb).wupcnt += 1;
        } else {
            errcode = KernelError::OBJ;
        }
    }

    end_cs(intsts);
    if errcode == KernelError::OK {
        Ok(())
    } else {
        Err(errcode)
    }
}

// Scheduling task

pub fn schedule() {

    let mut new_sch: [ *mut TCB; CPU_CORE_NUM ] = 
      [ core::ptr::null_mut(),
        core::ptr::null_mut(),
      ]; 

    let mut i: usize = 0;
    let mut j: usize = 0;
    unsafe {
        while i < CNF_MAX_TSK_PRI {
            if !READY_QUEUE[i].is_empty() {
                let mut iter = READY_QUEUE[i].iter();
                let itv = iter.next();
                if itv.is_some() {
                    let tcb: *mut TCB = itv.unwrap();
                    new_sch[j] = tcb;
                    j = j + 1;
                    if j > CPU_CORE_NUM - 1 {
                        break;
                    }
                }
            }
            i = i + 1;
        }
        if new_sch[0] != core::ptr::null_mut() {
            if new_sch[0] == cur_task[0] {
                sche_task[0] = new_sch[0];
                sche_task[1] = new_sch[1];
            } else if new_sch[0] == cur_task[1] || ( new_sch[1] != core::ptr::null_mut() && new_sch[1] == cur_task[0] ) {
                sche_task[0] = new_sch[1];
                sche_task[1] = new_sch[0];
            } else {
                sche_task[0] = new_sch[0];
                sche_task[1] = new_sch[1];
            }
        } else {
            sche_task[0] = core::ptr::null_mut();
            sche_task[1] = core::ptr::null_mut();
        }
        let coreno = get_cpu_coreno(); 
        if ( sche_task[coreno] != cur_task[coreno] )  && !disp_running[coreno] {
            dispatch();
        }

        let coreno_other = if coreno == 1 { 0 } else { 1 };

        if sche_task[coreno_other] != cur_task[coreno_other] {
            let icc_intno:u32 = (ICCINT_DISPATCH << 24) as u32;
            match icc_ras_int(icc_intno) {
                Ok(()) => {}
                Err(err) => {
                    error(err);
                    panic!("failed at icc_ras_int");
                }
            }
        }
    }
}

pub fn iccint_dispatch(_data:u32) {

    //begin critical section
    let intsts = begin_cs();

    let coreno = get_cpu_coreno();
    unsafe {
        if ( sche_task[coreno] != cur_task[coreno] ) && !disp_running[coreno] {
            dispatch();
        }
    }

    //end critical section
    end_cs(intsts);

}


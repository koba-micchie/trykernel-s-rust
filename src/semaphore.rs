use defer_lite::defer;

use core::result::Result;
use core::result::Result::{Ok,Err};

use crate::apidef::*;
use crate::config::*;
use crate::interrupt::*;
use crate::knldef::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::task::*;
use crate::typedef::*;
use crate::error::*;

// Semaphore Control Block (SEMCB)
static mut SEMCB_TBL: [ SEMCB; CNF_MAX_SEM_ID ] = 
[   SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
    SEMCB { state: KSSTAT::KsNonexist , semcnt: 0,  maxsem: 0 },
];

// Create Semaphore API 
pub fn tk_cre_sem(pk_csem: &TCsem) -> Result<ID, KernelError> {

    let intsts  = begin_cs();
    let mut errcode = KernelError::OK;

    let mut semid: ID = 0;

    while semid < CNF_MAX_SEM_ID  {
        unsafe {
            if SEMCB_TBL[semid].state == KSSTAT::KsNonexist {
                break;
            }
            semid += 1;
        }
    }
    if semid < CNF_MAX_SEM_ID {
        // Initialize SEMCB
        unsafe {
            SEMCB_TBL[semid].state  = KSSTAT::KsExist;
            SEMCB_TBL[semid].semcnt = pk_csem.isemcnt;
            SEMCB_TBL[semid].maxsem = pk_csem.maxsem;
        }
        semid += 1;
    } else {
        errcode = KernelError::LIMIT;
    }

    end_cs(intsts);

    if errcode == KernelError::OK {
        Ok(semid)
    } else {
        Err(errcode)
    }
}

// Semaphore get API 
pub fn tk_wai_sem(semid: ID, cnt: isize, tmout: TMO) -> Result<(),KernelError> {
    if semid <= 0 || semid > CNF_MAX_SEM_ID {
        return Err(KernelError::ID);
    }

    let intsts = begin_cs();
    let mut errcode = KernelError::OK;

    unsafe  {
        let semcb: *mut SEMCB = core::ptr::addr_of_mut!(SEMCB_TBL[semid - 1]);
        // Unregistered Semaphore
        if (*semcb).state == KSSTAT::KsExist {
            // Resources shortage, and wait time == 0 
            // Resource number of current semaphore are greater than resource number requested 
            if (*semcb).semcnt >= cnt {
                (*semcb).semcnt -= cnt;
            } else if tmout == TMO_POL {
                errcode = KernelError::TMOUT;
            } else {
                // If resource shortage, migrate to wait state 
                let coreno = get_cpu_coreno();
                let c_task: *mut TCB = cur_task[coreno];
                READY_QUEUE[(*c_task).itskpri].remove_top();
                // Change state of task to wait 
                (*c_task).state = TSTAT::TsWait;
                // Set wait factor 
                (*c_task).waifct = TWFCT::TwfctSem;
                // Set wait time 
                if tmout == TMO_FEVR {
                    (*c_task).waitim = tmout;
                } else {
                    (*c_task).waitim = tmout + TIMER_PERIOD;
                }
                (*c_task).waisem = cnt;
                 WAIT_QUEUE.add_entry(c_task);
                schedule();
            }
        } else {
            errcode = KernelError::NOEXS;
        }
    }

    end_cs(intsts);
    if errcode == KernelError::OK {
        Ok(())
    } else {
        Err(errcode)
    }
}

// Semaphore release API 
pub fn tk_sig_sem(semid: ID, cnt: isize) -> Result<(),KernelError> {
    if semid <= 0 || semid > CNF_MAX_SEM_ID {
        return Err(KernelError::ID);
    }

    let intsts: isize = begin_cs();
    let mut errcode = KernelError::OK;

    // Unregistered semaphore
    unsafe {
        let semcb: * mut SEMCB = core::ptr::addr_of_mut!(SEMCB_TBL[semid - 1]);
        if (*semcb).state == KSSTAT::KsExist {
            // Release resource
            (*semcb).semcnt += cnt;
            if (*semcb).semcnt <= (*semcb).maxsem {
                let mut iter: TcbQueueIterator = WAIT_QUEUE.iter();
                loop {
                    let itv = iter.next();
                    if !itv.is_some() {
                        break;
                    } 
                    let tcb: *mut TCB = itv.unwrap();
                    if (*tcb).waifct != TWFCT::TwfctSem {
                        if (*semcb).semcnt >= (*tcb).waisem {
                            (*semcb).semcnt -= (*tcb).waisem;
                            WAIT_QUEUE.remove_entry(tcb);
                            (*tcb).state = TSTAT::TsReady;
                            (*tcb).waifct = TWFCT::TwfctNon;
                            READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
                            schedule();
                        } else {
                            break;
                        }
                    }
                }
            } else {
                (*semcb).semcnt -= cnt;
                errcode = KernelError::QOVR;
            }
        } else {
            errcode = KernelError::NOEXS;
        }
    }
    end_cs(intsts);
    if errcode == KernelError::OK {
        Ok(())
    } else {
        Err(errcode)
    }
}


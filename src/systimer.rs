
use crate::apidef::*;
use crate::error::*;
use crate::icc_spin::*;
use crate::knldef::*;
use crate::logger::*;
use crate::sysdef::*;
use crate::syslib::*;
use crate::task::*;

// interrupt handler of systimer 
#[no_mangle]
pub unsafe extern "C" fn systimer_handler() {

    icc_loc_spin(SPINLOCK_KERNEL);

    let mut iter = WAIT_QUEUE.iter();
    loop {
        let itv = iter.next();
        if !itv.is_some() {
            break;
        } else {
            let tcb: *mut TCB = itv.unwrap();
            if (*tcb).waitim == TMO_FEVR {
                continue;
            } else if (*tcb).waitim > TIMER_PERIOD {
                // decrease waste time from wait time
                (*tcb).waitim -= TIMER_PERIOD;
            } else {
                WAIT_QUEUE.remove_entry(tcb);
                if (*tcb).waifct == TWFCT::TwfctDly {
                    (*tcb).waierr = KernelError::OK;
                } else {
                    (*tcb).waierr = KernelError::TMOUT;
                }
                (*tcb).state = TSTAT::TsReady;
                (*tcb).waifct = TWFCT::TwfctNon;
                READY_QUEUE[(*tcb).itskpri].add_entry(tcb);
            }
        }
    }
    schedule();

    icc_unl_spin(SPINLOCK_KERNEL);
}


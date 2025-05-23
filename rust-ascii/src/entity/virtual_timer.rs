use std::{
    future::Future,
    pin::Pin,
    sync::{atomic::{AtomicU64, Ordering}, Arc, Mutex, MutexGuard},
    task::{Context, Poll},
    // thread, 
    // time::Duration
};
// use tokio::runtime::Builder;
// use tokio::task::JoinSet;
use std::sync::LazyLock;
pub mod priority_queue;
use priority_queue::PriorityQueue;
use priority_queue::VirtualTimerState;

// pub fn test_exec() { //TODO turn into CPU test

//     println!("Starting Application");
//     let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
//         let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
//         rt.block_on(async {
//             let mut set = JoinSet::new();

//             set.spawn (async {
//                 VirtualTimerFuture::new(1000).await;
//                 // println!("Future is executed");
//             });

//             set.join_all().await;
//         });
//     });

//     thread::sleep(Duration::from_secs(1));
//     println!("time stepping...");
//     step_time(500);
//     thread::sleep(Duration::from_secs(1));
//     println!("time stepping...");
//     step_time(500);

//     handle.join().expect("bad things");

// }


//<Arc<Mutex<VirtualTimerState>>>>
static AWAITING_TIMER_FUTURES: LazyLock<Mutex<PriorityQueue>> = LazyLock::new(|| Mutex::new(PriorityQueue::new()));

pub static TIME_VALUE: AtomicU64 = AtomicU64::new(0);

pub fn set_time(value: u64) {
    TIME_VALUE.store(value, Ordering::Relaxed);
}

pub fn step_time(step: u64) {
    let old_time = TIME_VALUE.load(Ordering::Relaxed);
    let new_time = old_time + step;
    TIME_VALUE.store(new_time, Ordering::Relaxed);
    let mut futures = AWAITING_TIMER_FUTURES.lock().unwrap();

    loop {
        let mut need_pop = false;
        if let Some((task, priority_time)) = futures.peek() {
            if  new_time as f32 >= priority_time {
                let mut future: MutexGuard<'_, VirtualTimerState> = task.lock().unwrap();
                future.completed = true;
                if let Some(ref waker) = future.waker {
                    waker.wake_by_ref();
                }
                need_pop = true;
            }
        }
        if need_pop {
            futures.pop();
        }
        else {
            break;
        }
    }   
}


pub struct VirtualTimerFuture {
    state: Arc<Mutex<VirtualTimerState>>
}

impl Future for VirtualTimerFuture { // should I be in the channel? poll to find out
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        if state.completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl VirtualTimerFuture { //wait for duration
    pub fn new(duration: u64) -> Self {
        let state = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));
        
        let wake_time = TIME_VALUE.load(Ordering::Relaxed) + duration;
        let mut futures = AWAITING_TIMER_FUTURES.lock().unwrap();

        futures.push(state.clone(), wake_time as f32);

        VirtualTimerFuture { state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_time() {
        let vt = VirtualTimerFuture::new(1000); 
        step_time(500);
        {
            let state = vt.state.lock().unwrap();
            assert!(!state.completed, "Timer should not be completed yet");
        }
        step_time(500);
        {
            let state = vt.state.lock().unwrap();
            assert!(state.completed, "Timer should be completed now");
        }
    }

    #[test]
    fn test_virtual_timer_future_completion() {
        let vt = VirtualTimerFuture::new(2000);
        step_time(1000); 
        {
            let state = vt.state.lock().unwrap();
            assert!(!state.completed, "Timer should not be completed after 1 second");
        }
        step_time(1000);
        {
            let state = vt.state.lock().unwrap();
            assert!(state.completed, "Timer should be completed after 2 seconds");
        }
    }

    #[test]
    fn test_multiple_timers() {
        let vt1 = VirtualTimerFuture::new(1000);
        let vt2 = VirtualTimerFuture::new(2000);

        step_time(1000);
        {
            let state1 = vt1.state.lock().unwrap();
            let state2 = vt2.state.lock().unwrap();
            assert!(state1.completed, "First timer should be completed after 1 second");
            assert!(!state2.completed, "Second timer should not be completed after 1 second");
        }

        step_time(1000);
        {
            let state2 = vt2.state.lock().unwrap();
            assert!(state2.completed, "Second timer should be completed after 2 seconds");
        }
    }

    #[test]
    fn test_no_timers() {
        step_time(1000);
        let futures = AWAITING_TIMER_FUTURES.lock().unwrap();
        assert!(futures.is_empty(), "There should be no timers in the queue");
    }

    // #[test]
    // fn test_polling() {
    //     let vt = VirtualTimerFuture::new(1000);
    //     let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    //     let mut pinned_vt = Pin::new(&vt);
    //     assert_eq!(pinned_vt.poll(&mut cx), Poll::Pending, "Timer should be pending");
    //     step_time(1000);
    //     assert_eq!(pinned_vt.poll(&mut cx), Poll::Ready(()), "Timer should be ready");
    // }
}


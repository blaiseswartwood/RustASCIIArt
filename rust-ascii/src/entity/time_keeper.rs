
use crate::virtual_timer;
use std::sync::atomic::Ordering;

pub static DEFAULT_PREEMPTION: u64 = 3000;

#[derive(Clone)]
pub struct TimeKeeper {
    pub time: u64,
    target_preemption: u64,
}

impl TimeKeeper {
    pub fn new() -> Self {
        TimeKeeper { 
            time: virtual_timer::TIME_VALUE.load(Ordering::Relaxed),
            target_preemption: DEFAULT_PREEMPTION,
        }
    }

    pub fn set_target_preemption(&mut self, target_preemption: u64) {
        self.target_preemption = target_preemption;
    }

    pub fn schedule_time(&self) -> u64 {
        self.time + self.target_preemption
    }
    
    pub fn step_time(&mut self, milliseconds: u64) {
        self.time += milliseconds;
        //println!("Time step: {} ms, New time: {}", milliseconds, self.time);
    }

}

impl Default for TimeKeeper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // use std::time;

    use std::sync::atomic::Ordering;

    use crate::entity::virtual_timer;

    #[test]
    fn test_time_keeper() {
        virtual_timer::TIME_VALUE.store(0, Ordering::Relaxed);
        let mut time_keeper = super::TimeKeeper::new();
        time_keeper.set_target_preemption(7000);
        assert_eq!(time_keeper.time, 0);
        time_keeper.step_time(1000);
        assert_eq!(time_keeper.time, 1000);
    }

    #[test]
    fn test_schedule_time() {
        virtual_timer::TIME_VALUE.store(0, Ordering::Relaxed);
        let mut time_keeper = super::TimeKeeper::new();
        time_keeper.set_target_preemption(7000);
        assert_eq!(time_keeper.schedule_time(), 7000);
        time_keeper.step_time(1000);
        assert_eq!(time_keeper.schedule_time(), 8000);
    }

    #[test]
    fn test_cloning_behavior() {
        virtual_timer::TIME_VALUE.store(0, Ordering::Relaxed);
        let mut time_keeper = super::TimeKeeper::new();
        time_keeper.set_target_preemption(7000);
        let mut time_keeper2 = time_keeper.clone();
        time_keeper.step_time(1000);
        assert_eq!(time_keeper.time, 1000);
        assert_eq!(time_keeper2.time, 0);
        time_keeper2.step_time(3000);
        assert_eq!(time_keeper.time, 1000);
        assert_eq!(time_keeper2.time, 3000);
        let time_keeper4 = time_keeper2.clone();
        assert_eq!(time_keeper4.time, 3000);
    }
}
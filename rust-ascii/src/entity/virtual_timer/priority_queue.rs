use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::task::Waker;
// use std::task::{RawWaker, RawWakerVTable, Waker};
type Task = Arc<Mutex<VirtualTimerState>>;

pub struct VirtualTimerState {
    pub completed: bool,
    pub waker: Option<Waker>,
}

struct QueueItem {
    task: Task,
    priority: f32,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .partial_cmp(&self.priority)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for QueueItem {}

pub struct PriorityQueue {
    heap: BinaryHeap<QueueItem>,
}

impl Default for PriorityQueue {
    fn default() -> Self {
        Self::new()
    }   
}   

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, task: Task, priority: f32) {
        self.heap.push(QueueItem { task, priority });
    }

    pub fn pop(&mut self) -> Option<Task> {
        self.heap.pop().map(|item| item.task)
    }

    pub fn peek(&self) -> Option<(&Task, f32)> {
        self.heap.peek().map(|item| (&item.task, item.priority))
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut queue = PriorityQueue::new();
        let task1 = Arc::new(Mutex::new(VirtualTimerState {
            completed: true,
            waker: None,
        }));

        let task2 = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));

        queue.push(task1.clone(), 3.0);
        queue.push(task2.clone(), 2.0);

        assert_eq!(queue.len(), 2);

        let popped_task = queue.pop().unwrap();
        // assert_eq!(popped_task.lock().unwrap().completed, false); 
        assert!(Arc::ptr_eq(&popped_task, &task2)); 

        let popped_task = queue.pop().unwrap();
        assert!(Arc::ptr_eq(&popped_task, &task1)); 
    }

    #[test]
    fn test_peek() {
        let mut queue = PriorityQueue::new();
        let task = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));

        queue.push(task.clone(), 3.0);

        let peeked = queue.peek().unwrap();
        assert!(Arc::ptr_eq(peeked.0, &task));
        assert_eq!(peeked.1, 3.0);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = PriorityQueue::new();
        assert!(queue.is_empty());

        let task = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));

        queue.push(task, 1.0);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_len() {
        let mut queue = PriorityQueue::new();
        assert_eq!(queue.len(), 0);

        let task1 = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));
        let task2 = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));

        queue.push(task1, 1.0);
        queue.push(task2, 2.0);

        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_ordering_with_equal_priorities() {
        let mut queue = PriorityQueue::new();
        let task1 = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));
        let task2 = Arc::new(Mutex::new(VirtualTimerState {
            completed: false,
            waker: None,
        }));

        queue.push(task1.clone(), 1.0);
        queue.push(task2.clone(), 1.0);

        let popped_task1 = queue.pop().unwrap();
        let popped_task2 = queue.pop().unwrap();

        assert!(
            Arc::ptr_eq(&popped_task1, &task1) && Arc::ptr_eq(&popped_task2, &task2)
                || Arc::ptr_eq(&popped_task1, &task2) && Arc::ptr_eq(&popped_task2, &task1)
        );
    }
}

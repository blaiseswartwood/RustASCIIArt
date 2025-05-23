use std::mem;
use std::sync::atomic::{AtomicU32, Ordering};
use atomic_wait::*;
use crate::entity::shared_entity::SharedEntity;

const BUFFER_SIZE: usize = 262144		;

const METADATA_SIZE: usize =
    mem::size_of::<AtomicU32>() + // lock
    mem::size_of::<usize>() +     // size
    mem::size_of::<bool>() * 2;   // modified_flag + done

pub const MAX_ENTITIES: usize =
    (BUFFER_SIZE - METADATA_SIZE) / mem::size_of::<SharedEntity>();

pub struct SharedEntityBuffer {
    pub lock: AtomicU32,
    pub size: usize,
    pub modified_flag: bool,
    pub start_time: u64,
    pub done: bool,
    pub content: [SharedEntity; MAX_ENTITIES],
}

impl Default for SharedEntityBuffer {
    fn default() -> Self {
        SharedEntityBuffer::new()
    }
}

impl SharedEntityBuffer {
    pub fn new() -> SharedEntityBuffer {
        SharedEntityBuffer {
            lock: AtomicU32::new(0),
            size: 0,
            modified_flag: true,
            done: false,
            start_time: 0,
            content: [SharedEntity::new_empty(); MAX_ENTITIES],
        }
    }

    fn lock(&mut self) {
        while self.lock.swap(1, Ordering::Acquire) == 1 {
            wait(&self.lock, 1);
        }
    }

    fn unlock(&mut self) {
        self.lock.store(0, Ordering::Release);
        wake_one(&self.lock);
    }

    pub fn add(&mut self, shared_entity: SharedEntity) {
        self.lock();
        println!("Adding entity. Current size: {}, Max: {}", self.size, MAX_ENTITIES);
        if self.size >= MAX_ENTITIES {
            println!("Cannot add: buffer is full!");
            self.unlock();
            return;
        }
        self.content[self.size] = shared_entity;
        self.size += 1;
        self.modified_flag = true;
        self.unlock();
    }

    pub fn remove(&mut self, index: usize) -> SharedEntity {
        self.lock();
        let mut i = index;
        let res = self.content[i];
        while i < self.size - 1 {
            self.content[i] = self.content[i + 1];
            i += 1;
        }
        self.size -= 1;
        self.unlock();
        res
    }

    pub fn get(&mut self, index: usize) -> SharedEntity {
        self.lock();
        let res = self.content[index];
        self.unlock();
        res
    }

    pub fn current_size(&mut self) -> usize {
        self.lock();
        let res = self.size;
        self.unlock();
        res
    }

    pub fn cleanup_expired(&mut self, current_time: i32) {
        self.lock();
        let mut new_index = 0;
        for i in 0..self.size {
            let se = self.content[i];
            // let ent = se.to_entity();
            // println!("Entity {}: start_time: {}, duration: {}, current_time: {}", i, ent.start_time, ent.duration, current_time);
            if se.start_time + se.duration > current_time {
                if new_index != i {
                    self.content[new_index] = se;
                }
                new_index += 1;
            }
        }
        self.size = new_index;
        self.unlock();
    }

    pub fn mark_done(&mut self) {
        self.lock();
        self.done = true;
        self.unlock();
    }
}

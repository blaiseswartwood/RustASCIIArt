use crate::entity::{Entity, vbo::Vbo, virtual_timer};
use crate::{event_loop_drawing, example_animations};
use crate::entity::shared_entity::SharedEntity;
use super::shared_entity_buffer::SharedEntityBuffer;

use crate::entity::character_map;
use std::sync::atomic::Ordering;
use shared_memory::*;
use std::collections::HashMap;
use crate::fonts;
use crate::tests::run_tests;
use std::time::Instant;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

static mut GLOBAL_SHMEM_HANDLE: Option<Shmem> = None;

const SHMEM_SIZE: usize = 262144;

static mut SMEM_GLOBAL_ENTITY_BUFFER: Option<&mut SharedEntityBuffer> = None;

/// Main entry point — acts as server or client depending on shared memory status.
pub fn smem_init_and_branch() {
    println!("Starting shared memory system...");
    let shmem_flink = "smem_handle";

    let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
    fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
    fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
    let load_font_result = fonts::load_font(fonts_to_use);
    let atlas_map = load_font_result.atlas_map; // atlas_map[character_map.get(c)] => [x, y, w, h]
    let atlas = load_font_result.image;
    let character_map = load_font_result.lookup_table;
    character_map::put(character_map.clone());
    let max_num_characters = character_map.len();

    let shmem = match ShmemConf::new().size(SHMEM_SIZE).flink(shmem_flink).create() {
        Ok(m) => {
            println!("Server: created shared memory!");
            let raw_ptr = m.as_ptr() as *mut SharedEntityBuffer;
            unsafe {
                std::ptr::write(raw_ptr, SharedEntityBuffer::default());
                GLOBAL_SHMEM_HANDLE = Some(m);
                SMEM_GLOBAL_ENTITY_BUFFER = Some(&mut *raw_ptr);
            }
            // server
            event_loop_drawing(atlas, atlas_map, max_num_characters as u32);

            return; 
        }
        Err(ShmemError::LinkExists) => {
            println!("Client: shared memory exists, joining...");
            ShmemConf::new().flink(shmem_flink).open().unwrap()
        }
        Err(e) => {
            eprintln!("Failed to create or open shared memory: {e}");
            return;
        }
    };

    // let is_owner = shmem.is_owner();
    let raw_ptr = shmem.as_ptr() as *mut SharedEntityBuffer;

    unsafe {
        GLOBAL_SHMEM_HANDLE = Some(shmem); // ✅ keep alive
        SMEM_GLOBAL_ENTITY_BUFFER = Some(&mut *raw_ptr);
    }

    // client
    // println!("Client: shared memory opened, starting client loop...");
    client_loop();

}

pub async fn inc_time() {
    let mut previous_instant = Instant::now();
    loop {
        let step_nanos = Instant::now().duration_since(previous_instant).as_nanos();
        if(step_nanos < 1000000) {
            continue;
        }
        let step_millis = (step_nanos as f64 / 1_000_000.0f64).round();
        virtual_timer::step_time(step_millis as u64);
        previous_instant = Instant::now();
    }

}
pub fn time_thread() -> thread::JoinHandle<()> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let handle: thread::JoinHandle<()> = thread::spawn(move || {  
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        // virtual_timer::set_time(0);
        rt.block_on(async {
            let mut set = JoinSet::new();
            set.spawn(async move {
                inc_time().await;
            });
            while let Some(_res) = set.join_next().await {};
        });
    });
    handle
}

pub fn client_loop() {

    println!("Client loop started...");
    println!("Running test");
    if let Some(current_time) = get_time() {
        virtual_timer::set_time(current_time);
        println!("Start time of virtual timer: {}", current_time);
    } else {
        println!("Shared memory not initialized.");
    }
    let mut handles = vec![];
    
    handles.push(time_thread());

    // Add test handle
    // handles.push(run_tests());

    // More complex animations custom built
    let animation_handles = example_animations::run_animations();
    for handle in animation_handles {
        handles.push(handle);
    }

    

    // Wait for all handles to complete
    for handle in handles {
        let _ = handle.join();
    }
    println!("Finished running tests");

}
pub fn add(entity: Entity) {
    unsafe {
        if let Some(ebo) = SMEM_GLOBAL_ENTITY_BUFFER.as_mut() {
            //println!("Adding entity to shared memory.");
            if ebo.size < ebo.content.len() - 1 {
                //println!("Adding entity. Current size: {}, Max: {}", ebo.size, ebo.content.len());
                let current_time = virtual_timer::TIME_VALUE.load(Ordering::Relaxed) as i32;
                //println!("current time {}", current_time);
                //println!("entity start time {}", entity.start_time);
                ebo.content[ebo.size] = SharedEntity::from(entity);
                ebo.size += 1;
                ebo.modified_flag = true;
            } else {
                println!("Shared buffer full. ebo.size: {}, ebo.content.len(): {}", ebo.size, ebo.content.len());
            }
        } else {
            println!("Shared memory not initialized.");
        }
    }
}

pub fn need_update() -> bool {
    //println!("need_update() called.");
    unsafe {
        match SMEM_GLOBAL_ENTITY_BUFFER {
            Some(ref ebo) => ebo.modified_flag,
            None => {
                println!("need_update() called before shared memory was initialized.");
                false
            }
        }
    }
}

pub fn get_time() -> Option<u64> {
    unsafe {
        SMEM_GLOBAL_ENTITY_BUFFER
            .as_ref()
            .map(|ebo| ebo.start_time)
    }
}

pub fn update_time(step_millis: u64) {
    unsafe {
        if let Some(ebo) = SMEM_GLOBAL_ENTITY_BUFFER.as_mut() {
            ebo.start_time += step_millis;
        } else {
            println!("Shared memory not initialized.");
        }
    }
}

pub fn gen_vbo() -> Vbo {
    unsafe {
        if let Some(ebo) = SMEM_GLOBAL_ENTITY_BUFFER.as_mut() {
            let mut vbo = Vbo::new();
            let current_time = virtual_timer::TIME_VALUE.load(Ordering::Relaxed) as i32;
            ebo.cleanup_expired(current_time); 
            // println!("EBO SIZE: {}", ebo.size);
            for i in 0..ebo.size {
                let entity = ebo.content[i].to_entity();
                // println!("current time {}", current_time);
                // println!("entity start time {}", entity.start_time);
                if current_time - entity.start_time < entity.duration {
                    entity.generate_vertices(&mut vbo);
                }
            }

            ebo.modified_flag = false;
            vbo
        } else {
            println!("Shared memory not initialized.");
            Vbo::new()
        }
    }
}

// pub fn gen_vbo() -> Vbo {
//     unsafe {
//         if let Some(ebo) = SMEM_GLOBAL_ENTITY_BUFFER.as_mut() {
//             let mut vbo = Vbo::new();
//             let current_time = virtual_timer::TIME_VALUE.load(Ordering::Relaxed) as i32;

//             let mut retained = 0;
//             for i in 0..ebo.size {
//                 let entity = ebo.content[i].to_entity();
//                 if current_time - entity.start_time < entity.duration {
//                     entity.generate_vertices(&mut vbo);
//                     ebo.content[retained] = ebo.content[i]; // Retain valid entries
//                     retained += 1;
//                 }
//             }

//             ebo.size = retained;
//             ebo.modified_flag = false;
//             vbo
//         } else {
//             println!("Shared memory not initialized.");
//             Vbo::new()
//         }
//     }
// }
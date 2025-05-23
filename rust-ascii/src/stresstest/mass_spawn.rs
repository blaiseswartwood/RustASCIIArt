use crate::entity::{virtual_timer, Entity};
use crate::entity::time_keeper::TimeKeeper;
use crate::animations::translate::Translate;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicI32;
use std::sync::Arc;


pub fn dummy_entity() -> Entity {
    Entity::new([0.0, 0.0], 2000.0, '#', [1.0, 1.0, 0.0])
}

pub fn create_single_entity(position_x: f32, position_y: f32, duration: i32, time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new([position_x, position_y], 500.0, '!', [1.0, 1.0, 0.0]);
    let mut ent2: Entity = Translate::new_entity(ent, duration, [position_x, position_y]);
    ent2.sync_run(time_keeper);
}
                    
/* Expected behavior: Spawn a bunch of entities in a grid, instantly, wait a bit, repeat
    Should not "scroll" on the screen, should just be a bunch of entities blinking on and off */
    pub fn run_spawn_stress_test() -> thread::JoinHandle<()>  {
        let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
            let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
            rt.block_on(async {
                let mut set: JoinSet<()> = JoinSet::new();
    
                // this numbers allow you to see the error
                let number_of_rows = 100;
                let number_of_columns = 300;
                let gap_between_rows = 10.0;
                let gap_between_columns = 9.0;
                let number_of_spawns = 5;
                let time_between_spawns = 800;
                let duration_of_spawn = 500;
    
                // behaving numbers
                // let number_of_rows = 50;
                // let number_of_columns = 50;
                // let gap_between_rows = 30.0;
                // let gap_between_columns = 30.0;
                // let number_of_spawns = 10;
                // let time_between_spawns = 1000;
                // let duration_of_spawn = 500;
    
                let time_keeper = &mut TimeKeeper::new();
                let mut atomic_time = Arc::new(AtomicI32::new(0));
                // let mut time_diff = Arc::new(AtomicI32::new(0));
                for _n in 0..number_of_spawns {                               // This whole things takes about 3.5 seconds to run - Need to see if we can improve this
                    let mut time_keeper_clone = time_keeper.clone();
                    atomic_time = Arc::new(AtomicI32::new(virtual_timer::TIME_VALUE.load(Ordering::Relaxed) as i32));
                    let atomic_time = Arc::clone(&atomic_time);
                    set.spawn(async move {
                        for i in 0..number_of_rows {
                            for j in 0..number_of_columns {
                                // if spawn task is here, we run into async scheduling errors
                                create_single_entity(gap_between_columns * j as f32, gap_between_rows * i as f32, duration_of_spawn, &mut time_keeper_clone);
                            }
                            // println!("i {}", i);
                        }
                        let sys_time = virtual_timer::TIME_VALUE.load(Ordering::Relaxed);
                        atomic_time.fetch_sub(sys_time as i32, Ordering::SeqCst);
                    });
                    let mut dummy = dummy_entity();
                    dummy.wait(time_between_spawns, time_keeper).await;
                }
                println!("i occur");
                let end_time = atomic_time.load(Ordering::SeqCst);
                println!("End time: {}", end_time);
                // IS_ANIMATING.store(false, Ordering::SeqCst); // Ending so doesnt loop forever
                while let Some(_res) = set.join_next().await {};
            });
        });
        handle
    }
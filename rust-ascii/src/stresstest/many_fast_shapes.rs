
use crate::entity::Entity;
use crate::entity::time_keeper::TimeKeeper;
use crate::animations::translate::Translate;
use crate::IS_ANIMATING;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;
use std::sync::atomic::Ordering;

pub fn dummy_entity() -> Entity {
    Entity::new([0.0, 0.0], 2000.0, '#', [1.0, 1.0, 0.0])
}

pub async fn stress_test(i: i64, time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new([0.0, 25.0 * i as f32], 1000.0, '!', [1.0, 1.0, 0.0]);
    let mut ent2: Entity = Translate::new_entity(ent, 5000, [3000.0, 25.0 * i as f32]);
    ent2.run(time_keeper).await;
}                            

/* Expected behavior: Moving columns of entities, moving across the screen
The columns should be straight & the spacing between the columns should be even */
pub fn run_stress_test() -> thread::JoinHandle<()>  {
    let mut dummy = dummy_entity();
    let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            let mut set = JoinSet::new();
            let mut time_keeper = TimeKeeper::new();
            dummy.wait(4000, &mut time_keeper).await;

            // ---------- STRESSS TEST ----------
            println!();
            println!("Starting Stress Test");
            let columns: i64 = 1000; // number of columns to run in parallel

            // Non Behaving Numbers
            // let items_column: i64 = 50; // number of items per columns
            // let column_duration: i64 = 50; // time betwen each column

            // Behaving Numbers
            let items_column: i64 = 50; // number of items per columns
            let column_duration: i64 = 50; // time betwen each column

            let mut dummy = dummy_entity();

            for _j in 0..columns {  
                for i in 0..items_column {
                    // println!("time: {}", context.lock().unwrap().time.schedule_time());
                    let mut time_keeper_clone = time_keeper.clone();
                    set.spawn(async move {
                        stress_test(i, &mut time_keeper_clone).await;
                    });
                }
                dummy.wait(column_duration as u64, &mut time_keeper).await;
            }
            dummy.wait(1000, &mut time_keeper).await;
            
            println!("I AM DONE");
            dummy.wait(1000, &mut time_keeper).await;
            IS_ANIMATING.store(false, Ordering::SeqCst); // Ending so doesnt loop forever

            while let Some(_res) = set.join_next().await {};
        });
    });
    handle
}



use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::animations::translate::Translate;
use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

use crate::example_animations::dummy_entity;


pub async fn run_entity(start_pos: [f32; 2], time_keeper: &mut TimeKeeper, duration: i32, end_pos: [f32; 2], character: char) {
    let ent = Entity::new(start_pos, 1000.0, character, [1.0, 1.0, 0.0]);
    let mut ent2 = Translate::new_entity(ent, duration, end_pos);
    ent2.run_and_wait(time_keeper).await;
}

fn recursive_simple(time_keeper: TimeKeeper, start_pos: [f32; 2], character: char, move_by: f32, duration: i32) -> thread::JoinHandle<()> {
    let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            println!("I recurse here");
            let mut _dummy = dummy_entity();
            let mut set: JoinSet<()> = JoinSet::new();

            println!("start_pos: {:?}", start_pos);
            if(start_pos[0] <= 1920.0) & (start_pos[0] >= 0.0) & (start_pos[1] <= 1080.0) & (start_pos[1] >= 0.0) {
                let mut time_clone = time_keeper.clone();

                set.spawn( async move {
                    let end_pos = [start_pos[0], start_pos[1] - move_by];
                    run_entity(start_pos, &mut time_clone, duration, end_pos, character).await;
                    // return;
                });

                let mut time_clone = time_keeper.clone();

                set.spawn( async move {
                    let end_pos = [start_pos[0], start_pos[1] + move_by];
                    run_entity(start_pos, &mut time_clone, duration, end_pos, character).await;
                    // return;
                });

                let mut time_clone = time_keeper.clone();

                set.spawn( async move {
                    let end_pos = [start_pos[0] - move_by, start_pos[1]];
                    run_entity(start_pos, &mut time_clone, duration, end_pos, character).await;
                    // recursive_simple(animation_clone, time_clone, end_pos, character, move_by, duration);
                    // return;
                });

                let mut time_clone = time_keeper.clone();
                set.spawn( async move {
                    let end_pos = [start_pos[0] + move_by, start_pos[1]];
                    run_entity(start_pos, &mut time_clone, duration, end_pos, character).await;
                    recursive_simple( time_clone, end_pos, character, move_by, duration);
                    // return;
                });
            }

            while let Some(_res) = set.join_next().await {}; //no clue it crashes
        });
    });
    handle
}
// ------------------- PRESENTATION TEST ------------------- //

pub fn run() -> Vec<thread::JoinHandle<()>> {
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    let time_keeper = TimeKeeper::new();
    let mut i = 0;
    let j = 500;
    while i <= 1080 {
        let handle = recursive_simple(time_keeper.clone(), [j as f32, i as f32], 'r', 100.0, 500);
        handles.push(handle);
        i += 100;
    }
    handles
}

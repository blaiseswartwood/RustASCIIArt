pub mod many_fast_shapes;
pub mod mass_spawn;

use std::thread;


pub fn run_animations() -> Vec<thread::JoinHandle<()>>{ 
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    println!("Starting animations");
    handles.push(many_fast_shapes::run_stress_test());
    handles.push(mass_spawn::run_spawn_stress_test());

    handles
}
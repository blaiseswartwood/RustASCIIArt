pub mod hello_world;
pub mod presentationtest;
pub mod ball;
pub mod buffalo;
pub mod dvd;
pub mod elephant;
pub mod recursive;
use crate::entity::Entity;
use std::thread;

pub struct Inputs {
    pub ascii: &'static str,
    pub displacement: f32,
    pub top_left_corner: [f32; 2],
    pub character_spacing: f32,
    pub line_spacing: f32,
    pub approximate_characters_per_line: usize,
}

pub fn dummy_entity() -> Entity {
    Entity::new([0.0, 0.0], 2000.0, '#', [1.0, 1.0, 0.0])
}

pub fn run_animations() -> Vec<thread::JoinHandle<()>>{
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    println!("Starting animations");
    //handles.push(ball::run());
    //handles.push(elephant::run());
    // handles.push(hello_world::run_hello_world_test());
    // handles.push(presentationtest::run());
    //handles.push(buffalo::run());
    handles.push(dvd::run_dvd_animation());
    // let recurse: Vec<thread::JoinHandle<()>> = recursive::run();
    // for handle in recurse {
    //     handles.push(handle);
    // }
    // handles.push(cool_pattern::run());

    handles
}
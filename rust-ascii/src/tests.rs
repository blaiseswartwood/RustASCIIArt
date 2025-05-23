
// TEST MOD.RS
// pub mod entity;
// pub mod translate;
// pub mod colr_rand;
// pub mod char_rand;
// pub mod fading_trail;
// pub mod colr_interpolation;
// pub mod accelerate;

use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::animations::translate::Translate;
use crate::animations::colr_rand::ColrRand;
use crate::animations::char_rand::CharRand;
use crate::animations::fading_trail::FadingTrail;
use crate::animations::colr_interpolation::ColrInterpolate;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;


pub fn dummy_entity() -> Entity {
    Entity::new([0.0, 0.0], 2000.0, '#', [1.0, 1.0, 0.0])
}

pub async fn invisible_entity_test(time_keeper: &mut TimeKeeper) {
    let mut ent: Entity = Entity::new([100.0, 600.0], 0.0, '#', [1.0, 1.0, 0.0]);
    ent.run_and_wait(time_keeper).await;
}

pub async fn still_entity_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new([100.0, 600.0], 2000.0, '!', [1.0, 1.0, 0.0]);
    let mut translate_ent: Entity = Translate::new_entity(ent, 4000, [100.0, 600.0]);
    translate_ent.run_and_wait(time_keeper).await;
}

pub async fn translate_test(time_keeper: &mut TimeKeeper) {
    println!("Translate Test");
    let ent: Entity = Entity::new([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0]);
    let mut ent2: Entity = Translate::new_entity(ent, 2000, [600.0, 600.0]);
    ent2.wait(2000, time_keeper).await;
    ent2.run_and_wait(time_keeper).await;
}

pub async fn different_characters_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0]);
    let mut ent2: Entity = Translate::new_entity(ent, 2000, [100.0, 600.0]);
    ent2.run_and_wait(time_keeper).await;

    let ent3: Entity = Entity::new([100.0, 600.0], 2000.0, '$', [1.0, 1.0, 0.0]);
    let mut ent4: Entity = Translate::new_entity(ent3, 2000, [100.0, 400.0]);
    ent4.run_and_wait(time_keeper).await;

    let ent5: Entity = Entity::new([100.0, 600.0], 2000.0, '!', [1.0, 1.0, 0.0]);
    let mut ent6: Entity = Translate::new_entity(ent5, 2000, [100.0, 200.0]);
    ent6.run_and_wait(time_keeper).await;
}

pub async fn composable_translate_test(time_keeper: &mut TimeKeeper) {
    println!("Composable Translate Test");
    let ent: Entity = Entity::new([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0]);
    let mut ent2: Entity = Translate::new_entity(ent, 2000, [600.0, 600.0]);
    ent2.run_and_wait(time_keeper).await;
    let mut ent3: Entity = Translate::new_entity(ent2, 2000, [100.0, 600.0]);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn random_character_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([400.0, 400.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [400.0, 400.0]);
    let mut ent3: Entity = ColrRand::new_entity(ent2, 1);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn random_color_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([800.0, 400.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [800.0, 400.0]);
    let mut ent3: Entity = CharRand::new_entity(ent2, 1);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn random_color_translate_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([400.0, 100.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [400.0, 800.0]);
    let mut ent3: Entity = ColrRand::new_entity(ent2, 1);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn random_character_translate_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([400.0, 100.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [400.0, 800.0]);
    let mut ent3: Entity = CharRand::new_entity(ent2, 1);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn random_character_and_color_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([400.0, 100.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [400.0, 800.0]);
    let ent3: Entity = CharRand::new_entity(ent2, 1);
    let mut ent4: Entity = ColrRand::new_entity(ent3, 1);
    ent4.run_and_wait(time_keeper).await;
}

pub async fn color_interpolation_test(time_keeper: &mut TimeKeeper) {
    let ent: Entity = Entity::new_f([300.0, 300.0], 2000.0, '#', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [100.0, 600.0]);
    let mut ent3: Entity = ColrInterpolate::new_entity(ent2, [1.0, 1.0, 1.0]);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn fading_trail_test(time_keeper: &mut TimeKeeper) {
    println!("Fading Trail Test");
    let ent: Entity = Entity::new_f([200.0, 200.0], 2000.0, '!', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [800.0, 200.0]);
    let mut ent3: Entity = FadingTrail::new_entity(ent2, 500.0, 100);
    ent3.run_and_wait(time_keeper).await;
}

pub async fn fading_trail_colr_interpolation_test(time_keeper: &mut TimeKeeper) {
    println!("Fading Trail w/ Color Interpolation Test");
    let ent: Entity = Entity::new_f([200.0, 200.0], 2000.0, '!', [1.0, 1.0, 1.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [800.0, 200.0]);
    let ent3: Entity = FadingTrail::new_entity(ent2, 500.0, 400);
    let mut ent4: Entity = ColrInterpolate::new_entity(ent3, [0.0, 0.0, 0.0]);
    ent4.run_and_wait(time_keeper).await;
}

pub async fn fading_trail_char_rand(time_keeper: &mut TimeKeeper) {
    println!("Fading Trail w/ Character Randomization Test");
    let ent: Entity = Entity::new_f([200.0, 200.0], 2000.0, '!', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 6000, [800.0, 200.0]);
    let ent3: Entity = CharRand::new_entity(ent2, 500);
    let mut ent4: Entity = FadingTrail::new_entity(ent3, 1000.0, 800);
    ent4.run_and_wait(time_keeper).await;
}

pub async fn fading_trail_color_rand(time_keeper: &mut TimeKeeper) {
    println!("Fading Trail w/ Color Randomization Test");
    let ent: Entity = Entity::new_f([200.0, 200.0], 2000.0, '!', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 6000, [800.0, 200.0]);
    let ent3: Entity = ColrRand::new_entity(ent2, 1000);
    let mut ent4: Entity = FadingTrail::new_entity(ent3, 2000.0, 800);
    ent4.run_and_wait(time_keeper).await;
}

pub async fn fading_trail_composable_test(time_keeper: &mut TimeKeeper) {
    println!("Composable Fading Trail Test");
    let ent: Entity = Entity::new_f([200.0, 200.0], 2000.0, '!', [1.0, 1.0, 0.0], "libre_bodoni".to_string());
    let ent2: Entity = Translate::new_entity(ent, 2000, [800.0, 200.0]);
    let mut ent3: Entity = FadingTrail::new_entity(ent2, 500.0, 100);
    ent3.run_and_wait(time_keeper).await;
    let ent4: Entity = Translate::new_entity(ent3, 1000, [800.0, 400.0]);
    let mut ent5: Entity = FadingTrail::new_entity(ent4, 500.0, 100);
    ent5.run_and_wait(time_keeper).await;
}

pub fn run_tests() -> thread::JoinHandle<()> {
    let mut dummy = dummy_entity();
    
    let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            let mut set = JoinSet::new();
            println!("Start Visual Testing");
            let mut time_keeper = TimeKeeper::new();
            //println!("Time keepr intialized with time {}", time_keeper.time);
            dummy.wait(2000, &mut time_keeper).await;
            
  
            set.spawn(async move {
                invisible_entity_test(&mut time_keeper).await;
                still_entity_test(&mut time_keeper).await;
                // translate_test(&mut time_keeper).await;
                // different_characters_test(&mut time_keeper).await;
                composable_translate_test(&mut time_keeper).await;
                // random_character_test(&mut time_keeper).await;
                // random_color_test(&mut time_keeper).await;
                // random_color_translate_test(&mut time_keeper).await;
                // random_character_translate_test(&mut time_keeper).await;
                // random_character_and_color_test(&mut time_keeper).await;
                // color_interpolation_test(&mut time_keeper).await;
                fading_trail_test(&mut time_keeper).await;
                fading_trail_colr_interpolation_test(&mut time_keeper).await;
                fading_trail_char_rand(&mut time_keeper).await;
                fading_trail_color_rand(&mut time_keeper).await;
                fading_trail_composable_test(&mut time_keeper).await;
            });

            while let Some(_res) = set.join_next().await {};
        });
    });
    handle
}
use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::composition::translate::Translate;
use crate::composition::colr_interpolation::ColrInterpolate;
use crate::composition::fading_trail::FadingTrail;
use crate::AnimationContext;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

use crate::composition::dummy_entity;

pub fn run_fireball(animation_context: &AnimationContext) -> thread::JoinHandle<()> {
    let context = animation_context.clone();
    let mut dummy = dummy_entity(animation_context);
    
    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            let mut set = JoinSet::new();
            let mut time_keeper = TimeKeeper::new();
            dummy.wait(1000, &mut time_keeper).await;

            // Create the fireball
            let start_pos = [400.0, 300.0];
            let end_pos = [800.0, 300.0];
            let duration = 2000;
            
            // Create the main fireball entity
            let mut ent = Entity::new(start_pos, 50.0, '!', [1.0, 0.5, 0.0], &context);
            ent = FadingTrail::new_entity(ent, 5.0, 1);
            
            // Add glowing effect
            let mut glow_ent = Entity::new(start_pos, 70.0, 'z', [1.0, 0.3, 0.0], &context);
            glow_ent = FadingTrail::new_entity(glow_ent, 3.0, 1);
            
            // Create the movement animation
            let mut fireball = Translate::new_entity(ent, duration, end_pos);
            let mut glow = Translate::new_entity(glow_ent, duration, end_pos);
            
            // Run the fireball movement
            println!("Fireball animation started");
            fireball.run_and_wait(&mut time_keeper).await;
            glow.run_and_wait(&mut time_keeper).await;
            
            // Create explosion particles
            let explosion_chars = ['*', 'z', 'o', '0', '!'];
            let explosion_colors = [
                [1.0, 0.5, 0.0],  // Orange
                [1.0, 0.3, 0.0],  // Dark Orange
                [1.0, 0.0, 0.0],  // Red
                [1.0, 0.7, 0.0],  // Light Orange
                [1.0, 0.0, 0.0],  // Red
            ];
            
            println!("Explosion animation started");
            // Spawn explosion particles
            for i in 0..20 {
                let angle = (i as f32 * 18.0) * std::f32::consts::PI / 180.0;
                let distance = 100.0;
                let target_x = end_pos[0] + angle.cos() * distance;
                let target_y = end_pos[1] + angle.sin() * distance;
                
                let char_idx = i % explosion_chars.len();
                let color_idx = i % explosion_colors.len();
                
                let mut particle = Entity::new(
                    end_pos,
                    30.0,
                    explosion_chars[char_idx],
                    explosion_colors[color_idx],
                    &context
                );
                
                particle = FadingTrail::new_entity(particle, 2.0, 1);
                let mut moving_particle = Translate::new_entity(particle, 500, [target_x, target_y]);
                
                let mut time_keeper_clone = time_keeper.clone();
                set.spawn(async move {
                    moving_particle.run_and_wait(&mut time_keeper_clone).await;
                });
            }
            
            while let Some(_res) = set.join_next().await {};
        });
    });
    handle
}

// ------------------- PRESENTATION TEST ------------------- //
pub fn run(animation_context: &AnimationContext) -> thread::JoinHandle<()> {
    run_fireball(animation_context)
} 
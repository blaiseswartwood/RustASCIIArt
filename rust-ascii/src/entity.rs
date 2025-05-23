pub mod vbo;
pub mod virtual_timer;
pub mod time_keeper;
pub mod character_map;
pub mod shared_entity_buffer;
pub mod shared_entity;
pub mod global_ebo;

use vbo::Vbo;
use vbo::Vertex;
use time_keeper::TimeKeeper;
use crate::animations::fading_trail::FadingTrail;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Clone)]
pub struct Entity {
    pub origin: [f32; 2], // center of the character (4 vertices)
    pub end_pos: [f32; 2], // center of the character (4 vertices)
    pub font_size: f32,
    pub character: char,
    pub color: [f32; 3],
    pub new_color: [f32; 3],
    pub duration: i32,
    pub fade_time: f32,
    pub color_delay_time: i32,
    pub char_delay_time: i32,
    pub texture_id: i32,
    pub fading_frequency: i32,
    pub latency: i32,
    pub random_seed: f32,
    pub start_time: i32,
    pub entity_id: i32,
    pub accel_degree: f32,
}

static ENTITY_ID_GEN_ATOMIC: AtomicI32 = AtomicI32::new(0);
pub fn generate_entity_id() -> i32 {
    ENTITY_ID_GEN_ATOMIC.fetch_add(1, Ordering::SeqCst)
}

impl Entity{
    pub fn new(origin: [f32; 2], font_size: f32, character: char, color: [f32; 3]) -> Entity {
        let cid = character_map::get(character, "default".to_string());
        let mut rng = StdRng::from_entropy(); // Use StdRng, which is thread-safe
        let randoms = rng.gen();
        Entity {
            origin,
            end_pos: origin,
            font_size,
            character,
            color,
            new_color: color,
            duration: 0,
            fade_time: 0.0,
            color_delay_time: -1,
            char_delay_time: -1,
            texture_id: cid,
            fading_frequency: -1,
            latency: 0,
            random_seed: randoms,
            start_time: 0,
            entity_id: generate_entity_id(),
            accel_degree: 0.0,
        }
    }
    // allows the user to specify the font. Didn't want to require it for all entities
    pub fn new_f(origin: [f32; 2], font_size: f32, character: char, color: [f32; 3], font: String) -> Entity {
        let cid = character_map::get(character, font);
        let mut rng = StdRng::from_entropy(); // Use StdRng, which is thread-safe
        let randoms = rng.gen();
        Entity {
            origin,
            end_pos: origin,
            font_size,
            character,
            color,
            new_color: color,
            duration: 0,
            fade_time: 0.0,
            color_delay_time: -1,
            char_delay_time: -1,
            texture_id: cid,
            fading_frequency: -1,
            latency: 0,
            random_seed: randoms,
            start_time: 0,
            entity_id: generate_entity_id(),
            accel_degree: 0.0,
        }
    }

    

    pub async fn wait(&mut self, milliseconds : u64, time_keeper: &mut TimeKeeper) {
        let stime = virtual_timer::TIME_VALUE.load(Ordering::Relaxed); 
        let latency = stime.saturating_sub(time_keeper.time);        
        println!("latency {}", latency);
        println!("milliseconds {}", milliseconds);
        if latency <= milliseconds {
            println!("scheudled wait");
            virtual_timer::VirtualTimerFuture::new(milliseconds - latency).await; 
            println!("wait done");
        }
        time_keeper.step_time(milliseconds);
    }

    pub async fn run(&mut self, time_keeper: &mut TimeKeeper) {
        if self.fading_frequency != -1 {
            println!("wrong run function, call run_and_wait on FadingTrail Entities");
            self.run_and_wait(time_keeper).await;
        }
        self.run_internal(false, time_keeper).await;
    }

    pub async fn run_and_wait(&mut self, time_keeper: &mut TimeKeeper){
        if self.fading_frequency != -1 {
            FadingTrail::run_fading_trail(self, time_keeper).await;
            self.update_entity();
        }
        else  {
            self.run_internal(true, time_keeper).await;
        }
    }

    pub fn sync_run(&mut self, time_keeper: &mut TimeKeeper) { // this function pretty much instant
        self.start_time = time_keeper.schedule_time() as i32;
        global_ebo::add(self.clone());
        self.update_entity();
    }

    pub async fn run_internal(&mut self, should_wait: bool, time_keeper: &mut TimeKeeper) {
        self.start_time = time_keeper.schedule_time() as i32;
        global_ebo::add(self.clone());
        println!("Added client entity to EBO");
        if self.duration != 0 && should_wait {
            println!("waiting for {} ms", self.duration);
            self.wait(self.duration as u64, time_keeper).await;
        }

        self.update_entity();
    }

    fn update_entity(&mut self) {
        self.origin = self.end_pos;
        self.color = self.new_color;
        self.duration = 0;
        self.fade_time = 0.0;
        self.color_delay_time = -1;
        self.char_delay_time = -1;
        self.fading_frequency = -1;
    }

    pub fn generate_vertices(&self, vbo: &mut Vbo) {
        let length = vbo.vertex_buffer.len() as u32;

        for i in 0..4 {
            vbo.vertex_buffer.push(Vertex {
                cmdline: 0,
                start_pos: self.origin,
                end_pos: self.end_pos,
                start_time: self.start_time,
                end_time: self.start_time + self.duration + 1,
                font_size: self.font_size,
                tid: self.texture_id,
                corner: i,
                color: self.color,
                new_color: self.new_color,
                random_seed: self.random_seed,
                fade_time: self.fade_time,
                color_delay_time: self.color_delay_time,
                char_delay_time: self.char_delay_time,
                accel_degree: self.accel_degree,
            });
        }
        vbo.index_buffer.extend_from_slice(&[length, length + 1, length + 2, length + 3, length + 2, length + 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::animations::translate::Translate;
    use crate::animations::colr_rand::ColrRand;
    use crate::animations::char_rand::CharRand;
    use crate::animations::colr_interpolation::ColrInterpolate;
    use crate::animations::fading_trail::FadingTrail;
    use crate::entity::virtual_timer;
    use crate::entity::character_map;
    use crate::fonts;
    use std::collections::HashMap;

    fn load_fonts() {
        let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
        fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
        fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
        let load_font_result = fonts::load_font(fonts_to_use);
        let character_map = load_font_result.lookup_table;
        character_map::put(character_map.clone());
    }

    #[test]
    fn test_bad_font() {  
        load_fonts();      
        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0],"bad_font".to_string());
        assert_eq!(ent1.texture_id, 0);
    }

    // #[tokio::test]
    // async fn test_time_keeper() {
    //     load_fonts();
    //     let mut time_keeper = TimeKeeper::new();
    //     virtual_timer::TIME_VALUE.store(0, Ordering::Relaxed);

    //     let ent1 = Entity::new([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0]);
    //     let mut ent2 = Translate::new_entity(ent1, 1000, [200.0, 700.0]);
    //     assert_eq!(time_keeper.time, 38000);

    //     let handle1 = tokio::spawn(async move {
    //         ent2.run_and_wait(&mut time_keeper).await;
    //         assert_eq!(time_keeper.time, 1000);
    //         ent2.run(&mut time_keeper).await;
    //         assert_eq!(time_keeper.time, 1000);
    //     });

    //     let handle2 = tokio::spawn(async move {
    //         for _i in 0..100 {
    //             virtual_timer::step_time(200);
    //         }
    //     });

    //     let _ = tokio::join!(handle1, handle2);
    // }

    #[tokio::test]
    async fn test_generate_vbo() {
        load_fonts();

        let time_keeper = &mut TimeKeeper::new();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());    
        let mut ent2: Entity = Translate::new_entity(ent1, 1000, [200.0, 700.0]);
        
        ent2.run(time_keeper).await;
        virtual_timer::step_time(3000 + time_keeper::DEFAULT_PREEMPTION);
        
        let vbo = crate::entity::global_ebo::gen_vbo();

        assert_eq!(vbo.vertex_buffer.len(), 0);
        assert_eq!(vbo.index_buffer.len(), 0);
        assert_eq!(ent2.duration, 0);
        assert_eq!(ent2.origin, [200.0, 700.0]);
    }

    #[test]
    fn test_entity_new_translate() {

        load_fonts();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = Translate::new_entity(ent1, 1000, [200.0, 700.0]);

        assert_eq!(ent2.origin, [100.0, 600.0]);
        assert_eq!(ent2.end_pos, [200.0, 700.0]);
        assert_eq!(ent2.duration, 1000);
    }

    #[test]
    fn test_entity_new_colr_rand() {
        
        load_fonts();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = ColrRand::new_entity(ent1, 1000);

        assert_eq!(ent2.color_delay_time, 1000);
    }

    #[test]
    fn test_entity_new_char_rand() {
        
        load_fonts();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = CharRand::new_entity(ent1, 1000);

        assert_eq!(ent2.char_delay_time, 1000);
    }

    #[test]
    fn test_entity_new_colr_interpolation() {

        load_fonts();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = ColrInterpolate::new_entity(ent1, [0.0, 1.0, 1.0]);
        assert_eq!(ent2.new_color, [0.0, 1.0, 1.0]);
    }

    #[test]
    fn test_entity_new_fadingtrail() {
        
        load_fonts();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = FadingTrail::new_entity(ent1, 10.0, 1);

        assert_eq!(ent2.fade_time, 10.0);
        assert_eq!(ent2.fading_frequency, 1);
    }

    #[tokio::test]
    async fn test_entity_fading_run() {

        load_fonts();

        let mut time_keeper = TimeKeeper::new();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());
        let ent2: Entity = Translate::new_entity(ent1, 500, [200.0, 700.0]);
        let mut ent3: Entity = FadingTrail::new_entity(ent2, 10.0, 400);

        assert_eq!(ent3.duration, 500);
        assert_eq!(ent3.origin, [100.0, 600.0]);
        assert_eq!(ent3.fade_time, 10.0);
        assert_eq!(ent3.fading_frequency, 400);

        
        let handle1 = tokio::spawn(async move {
            ent3.run_and_wait(&mut time_keeper).await;
            virtual_timer::step_time(3000 + time_keeper::DEFAULT_PREEMPTION);
            assert_eq!(ent3.origin, [200.0, 700.0]);
            assert_eq!(ent3.duration, 0);
        });

        let handle2 = tokio::spawn(async move {
            for _i in 0..100 {
                virtual_timer::step_time(200);
            }
        });

        let _ = tokio::join!(handle1, handle2);

        assert_eq!(crate::entity::global_ebo::gen_vbo().vertex_buffer.len(), 0);
    }
}
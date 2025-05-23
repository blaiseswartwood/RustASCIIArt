use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use crate::entity::{self, Entity};
use crate::entity::time_keeper::TimeKeeper;

pub struct FadingTrail {
        pub fade_t : i32,
        pub frequency: i32,
}

impl FadingTrail{

    pub fn new_entity(entity: Entity, fade_time: f32, fading_frequency: i32) -> Entity {
        Entity {
            origin: entity.origin, 
            end_pos: entity.end_pos, 
            font_size: entity.font_size, 
            character: entity.character,
            color: entity.color,
            new_color: entity.new_color,
            duration: entity.duration,
            fade_time,
            color_delay_time: entity.color_delay_time,
            char_delay_time: entity.char_delay_time,
            texture_id: entity.texture_id,
            fading_frequency,
            latency: entity.latency,
            random_seed: entity.random_seed,
            start_time: entity.start_time,
            entity_id: entity.entity_id,
            accel_degree: entity.accel_degree,
        }
    }

    pub async fn run_fading_trail(entity: &Entity, time_keeper: &mut TimeKeeper) {
        let num_entities = entity.duration / entity.fading_frequency; // number of entities to create
        let entity_duration = entity.duration / num_entities;
        let mut rng = StdRng::from_entropy(); // Use StdRng, which is thread-safe
        let randoms = rng.gen();

        let mut entity_positions: Vec<[f32; 2]> = Vec::with_capacity(num_entities as usize);
        
        for i in 0..num_entities {
            let current_entity_pos_x = entity.origin[0] + (entity.end_pos[0] - entity.origin[0]) * (i as f32 / num_entities as f32);
            let current_entity_pos_y = entity.origin[1] + (entity.end_pos[1] - entity.origin[1]) * (i as f32 / num_entities as f32);
            entity_positions.push([current_entity_pos_x, current_entity_pos_y]);
        }

        let mut entity_colors: Vec<[f32; 3]> = Vec::with_capacity(num_entities as usize);
        entity_colors.push(entity.color);
        for i in 0..num_entities {
            let current_entity_color_r = entity.color[0] + (entity.new_color[0] - entity.color[0]) * (i as f32 / num_entities as f32);
            let current_entity_color_g = entity.color[1] + (entity.new_color[1] - entity.color[1]) * (i as f32 / num_entities as f32);
            let current_entity_color_b = entity.color[2] + (entity.new_color[2] - entity.color[2]) * (i as f32 / num_entities as f32);
            entity_colors.push([current_entity_color_r, current_entity_color_g, current_entity_color_b]);
        }
        
        for i in 0..num_entities-1 {
            let mut fading_ent = Entity {
                origin: entity_positions[i as usize],
                end_pos: entity_positions[i as usize],
                font_size: entity.font_size,
                character: entity.character,
                color: entity_colors[i as usize],
                new_color: entity_colors[i as usize + 1],
                duration: entity.fade_time as i32,
                fade_time: entity.fade_time,
                color_delay_time: entity.color_delay_time,
                char_delay_time: entity.char_delay_time,
                texture_id: entity.texture_id,
                fading_frequency: -1,
                latency: entity.latency,
                random_seed: randoms,
                start_time: entity.start_time,
                entity_id: entity::generate_entity_id(),
                accel_degree: entity.accel_degree,
            };
            let mut ent = Entity {
                origin: entity_positions[i as usize],
                end_pos: entity_positions[i as usize + 1],
                font_size: entity.font_size,
                character: entity.character,
                color: entity_colors[i as usize],
                new_color: entity_colors[i as usize + 1],
                duration: entity_duration + 2,
                fade_time: 0.0,
                color_delay_time: entity.color_delay_time,
                char_delay_time: entity.char_delay_time,
                texture_id: entity.texture_id,
                fading_frequency: -1,
                latency: entity.latency,
                random_seed: randoms,
                start_time: entity.start_time,
                entity_id: entity::generate_entity_id(),
                accel_degree: entity.accel_degree,
            };
            fading_ent.wait(entity.fading_frequency as u64, time_keeper).await;
            fading_ent.run_internal(false, time_keeper).await;
            ent.run_internal(false, time_keeper).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FadingTrail; 
    use super::super::translate::Translate;
    use crate::entity::Entity;
    use crate::entity::character_map;
    use std::collections::HashMap;
    use crate::fonts; 
    use crate::global_ebo;
    use crate::virtual_timer;
    use super::entity::vbo::Vbo;
    use super::entity::time_keeper;
    use super::entity::time_keeper::TimeKeeper;

    #[test]
    fn test_create_entity() {
        let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
        fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
        fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
        let load_font_result = fonts::load_font(fonts_to_use);
        let character_map = load_font_result.lookup_table;
        character_map::put(character_map.clone());

        let ent: Entity = Entity::new([300.0, 300.0], 2000.0, '#', [1.0, 1.0, 0.0]);
        let ent2: Entity = Translate::new_entity(ent, 8000, [400.0, 400.0]);
        let ent3: Entity = FadingTrail::new_entity(ent2, 10.0, 1);
        assert_eq!(ent3.color_delay_time, -1);
        assert_eq!(ent3.fade_time, 10.0);
        assert_eq!(ent3.fading_frequency, 1);
        assert_eq!(ent3.end_pos, [400.0, 400.0]);
        assert_eq!(ent3.origin, [300.0, 300.0]);
        assert_eq!(ent3.duration, 8000);      
    }

    #[tokio::test]
    async fn test_run_fading_trail() {
        let mut fonts_to_use: HashMap<&str, Vec<u8>> = HashMap::new();
        fonts_to_use.insert("default", google_fonts::lemonada_variable().unwrap());
        fonts_to_use.insert("libre_bodoni", google_fonts::libre_bodoni_variable().unwrap());
        let load_font_result = fonts::load_font(fonts_to_use);
        let character_map = load_font_result.lookup_table;
        character_map::put(character_map.clone());

        let time_keeper = &mut TimeKeeper::new();

        let ent1: Entity = Entity::new_f([100.0, 600.0], 2000.0, '#', [1.0, 1.0, 0.0], "test_font".to_string());    
        let mut ent2: Entity = Translate::new_entity(ent1, 1000, [200.0, 700.0]);
        
        ent2.run(time_keeper).await;
        virtual_timer::step_time(3000 + time_keeper::DEFAULT_PREEMPTION);

        let mut vbo = Vbo::new();
        if global_ebo::need_update() {
            vbo = global_ebo::gen_vbo();
        }

        assert_eq!(vbo.vertex_buffer.len(), 0);
        assert_eq!(vbo.index_buffer.len(), 0);
        assert_eq!(ent2.duration, 0);
        assert_eq!(ent2.origin, [200.0, 700.0]);
    }
}
use crate::entity::Entity;
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicU32;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SharedEntity {
    pub origin: [f32; 2],
    pub end_pos: [f32; 2],
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
    pub accel_degree: f32,
}

impl SharedEntity {
    pub fn new_empty() -> Self {
        SharedEntity {
            origin: [0.0, 0.0],
            end_pos: [0.0, 0.0],
            font_size: 0.0,
            character: ' ',
            color: [0.0, 0.0, 0.0],
            new_color: [0.0, 0.0, 0.0],
            duration: 0,
            fade_time: 0.0,
            color_delay_time: 0,
            char_delay_time: 0,
            texture_id: 0,
            fading_frequency: 0,
            latency: 0,
            random_seed: 0.0,
            start_time: 0,
            accel_degree: 0.0,
        }
    }

    pub fn to_entity(&self) -> Entity {
        Entity {
            origin: self.origin,
            end_pos: self.end_pos,
            font_size: self.font_size,
            character: self.character,
            color: self.color,
            new_color: self.new_color,
            duration: self.duration,
            fade_time: self.fade_time,
            color_delay_time: self.color_delay_time,
            char_delay_time: self.char_delay_time,
            texture_id: self.texture_id,
            fading_frequency: self.fading_frequency,
            latency: self.latency,
            random_seed: self.random_seed,
            start_time: self.start_time,
            entity_id: 0, 
            accel_degree: self.accel_degree,
        }
    }
}

impl From<Entity> for SharedEntity {
    fn from(e: Entity) -> Self {
        SharedEntity {
            origin: e.origin,
            end_pos: e.end_pos,
            font_size: e.font_size,
            character: e.character,
            color: e.color,
            new_color: e.new_color,
            duration: e.duration,
            fade_time: e.fade_time,
            color_delay_time: e.color_delay_time,
            char_delay_time: e.char_delay_time,
            texture_id: e.texture_id,
            fading_frequency: e.fading_frequency,
            latency: e.latency,
            random_seed: e.random_seed,
            start_time: e.start_time,
            accel_degree: e.accel_degree,
        }
    }
}
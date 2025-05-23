use crate::entity::Entity;
pub struct ColrInterpolate {
    pub new_color: [f32; 4],
}

impl ColrInterpolate{

    pub fn new_entity(entity: Entity, new_color: [f32; 3]) -> Entity {
        Entity {
            origin: entity.origin, 
            end_pos: entity.end_pos, 
            font_size: entity.font_size, 
            character: entity.character, 
            color: entity.color, 
            new_color, 
            duration: entity.duration, 
            fade_time: entity.fade_time, 
            color_delay_time: entity.color_delay_time, 
            char_delay_time: entity.char_delay_time,
            texture_id: entity.texture_id, 
            fading_frequency: entity.fading_frequency,
            latency: entity.latency,
            random_seed: entity.random_seed,
            start_time: entity.start_time,
            entity_id: entity.entity_id,
            accel_degree: entity.accel_degree,
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::ColrInterpolate; 
    use super::super::translate::Translate;
    use crate::entity::Entity; 
    use crate::entity::character_map;
    use std::collections::HashMap;
    use crate::fonts;

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
        let ent3: Entity = ColrInterpolate::new_entity(ent2, [1.0, 1.0, 1.0]);
        assert_eq!(ent3.color, [1.0, 1.0, 0.0]);
        assert_eq!(ent3.new_color, [1.0, 1.0, 1.0]);
        assert_eq!(ent3.end_pos, [400.0, 400.0]);
        assert_eq!(ent3.origin, [300.0, 300.0]);
        assert_eq!(ent3.duration, 8000);      
    }
}
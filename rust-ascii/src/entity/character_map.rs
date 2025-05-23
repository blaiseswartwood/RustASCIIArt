// Warning: Do not change/mutate the character_map after it is created. 
// Do not mutate it after it is initialized. Must be initialized before use. Be cautious with this. Do not break the rules.
// This is a static mutable reference to a HashMap that is used to store the character map for different fonts.
use std::collections::HashMap;


static mut CHARACTER_MAP: Option<HashMap<(char, String), usize>> = None;
pub fn put(character_map: HashMap<(char, String), usize>) {
    unsafe {
        CHARACTER_MAP = Some(character_map);
    }
}
pub fn get(character: char, font: String) -> i32 {
    unsafe {
        #[allow(static_mut_refs)]
        match CHARACTER_MAP.as_ref().unwrap().get(&(character, font.clone())) {
            Some(index) => *index as i32,
            None => {
                println!("Error: character {} not found in font {}. Either the font or the character is not in the map. Changed to default character.", character, font);
                0
            }
        }
    }
} 
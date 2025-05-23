use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::animations::translate::Translate;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

use crate::example_animations::Inputs;
use crate::example_animations::dummy_entity;

//only uses let characters = vec![10, 36, 92, 32, 124, 95, 47];
pub fn run_hello_world_test() -> thread::JoinHandle<()> {
    let ascii = r"
$$\   $$\           $$\ $$\                 $$\      $$\                     $$\       $$\ 
$$ |  $$ |          $$ |$$ |                $$ | $\  $$ |                    $$ |      $$ |
$$ |  $$ | $$$$$$\  $$ |$$ | $$$$$$\        $$ |$$$\ $$ | $$$$$$\   $$$$$$\  $$ | $$$$$$$ |
$$$$$$$$ |$$  __$$\ $$ |$$ |$$  __$$\       $$ $$ $$\$$ |$$  __$$\ $$  __$$\ $$ |$$  __$$ |
$$  __$$ |$$$$$$$$ |$$ |$$ |$$ /  $$ |      $$$$  _$$$$ |$$ /  $$ |$$ |  \__|$$ |$$ /  $$ |
$$ |  $$ |$$   ____|$$ |$$ |$$ |  $$ |      $$$  / \$$$ |$$ |  $$ |$$ |      $$ |$$ |  $$ |
$$ |  $$ |\$$$$$$$\ $$ |$$ |\$$$$$$  |      $$  /   \$$ |\$$$$$$  |$$ |      $$ |\$$$$$$$ |
\__|  \__| \_______|\__|\__| \______/       \__/     \__| \______/ \__|      \__| \_______|
";

    let inputs = Inputs {
        ascii,
        displacement: 120.0,
        top_left_corner:  [100.0, 400.0],
        character_spacing: 15.0,
        line_spacing: 15.0,
        approximate_characters_per_line: 94,
    };
    spawn_entities_from_ascii(inputs)
}

fn spawn_entities_from_ascii(inputs: Inputs) -> thread::JoinHandle<()> {
    let mut dummy = dummy_entity();
    
    let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            let mut set = JoinSet::new();
            println!("HELLO WORLD AniMatIon");
            let mut time_keeper = TimeKeeper::new();
            println!("Making time keeper");
            dummy.wait(4000, &mut time_keeper).await;
            println!("HELLO WORLD AniMatIon2");
            let mut character = 0.0;
            let mut line = 0.0;
            let mut colors: Vec<[f32; 3]> = vec![];
            for i in 0..inputs.approximate_characters_per_line {
                let current_entity_color_g = i as f32 / inputs.approximate_characters_per_line as f32;
                colors.push([0.0, current_entity_color_g, 0.5]);
            }
            let mut character_codes: Vec<u32> = vec![];
            println!("Here");
            for c in inputs.ascii.chars() {
                println!("inside loop");
                let mut found = false;
                if !character_codes.contains(&(c as u32)) {
                    character_codes.push(c as u32);
                    found = true;
                }
                if !found {
                    character_codes.push(c as u32);
                    println!("character code {}", c as u32);
                }
                if c == ' ' {
                    character += 1.0;
                    continue;
                }
                if (c as u32 == 10) || (c == '\n') {
                    line += 1.0;
                    character = 0.0;
                    continue;
                }
                println!("SENDING THROUGH ENTITY: c {} - ascii {}", c, c as u32);
                let color = colors[character as usize];
                let position = [inputs.top_left_corner[0] + (character * inputs.character_spacing), inputs.top_left_corner[1] - (line * inputs.line_spacing)];
                let duration = 2000;
                let mut time_keeper_clone = time_keeper.clone();
                set.spawn(async move {

                    let ent: Entity = Entity::new(position, 300.0, c, color);
                    let mut ent2: Entity = Translate::new_entity(ent, duration + (character as i32 * 30), [position[0], position[1] - inputs.displacement]);
                    ent2.run_and_wait(&mut time_keeper_clone).await;
                    let mut ent3: Entity = Translate::new_entity(ent2, duration + (character as i32 * 30), [position[0], position[1] + inputs.displacement]);
                    ent3.run_and_wait(&mut time_keeper_clone).await;
                    println!("sending????");
                    for _i in 0..30 {
                        ent2 = Translate::new_entity(ent3, duration, [position[0], position[1] - inputs.displacement]);
                        ent2.run_and_wait(&mut time_keeper_clone).await;
                        ent3 = Translate::new_entity(ent2, duration, [position[0], position[1] + inputs.displacement]);
                        ent3.run_and_wait(&mut time_keeper_clone).await;
                    }
                });
                character += 1.0;
            }
            while let Some(_res) = set.join_next().await {};
        });
    });
    handle
}
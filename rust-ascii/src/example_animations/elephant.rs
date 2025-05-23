use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::animations::translate::Translate;
use crate::animations::colr_rand::ColrRand;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;

use crate::example_animations::dummy_entity;

struct Inputs {
    ascii: &'static str,
    displacement: f32,
    top_left_corner: [f32; 2],
    character_spacing: f32,
    line_spacing: f32,
    approximate_characters_per_line: usize,
}


pub fn run() -> thread::JoinHandle<()> {
        let ascii = r#"
                    ..:==+++++++++=-..                    ..-=+++++++++=-:..                      
                  .-*#%#*==---------=+#%##############%#####%#+=---------==+#%#*-.                  
               .-*%#=:.          .:-:-=*+-.....------:....:=*+-:::.          .:=#%*-.               
            ..=%#=.   ..:::::......:%@*.  .. ..      ... ..  :#%+.. ...::-:::.  ..=#%=..            
          ..=@#:.  ..:=***=-:=####%*%=.. .+.             :=. ..#*#####+::=***+:..  .:#@=..          
         .-@%..    ...-+##*@@@=....:-.-. -*.             .#- ::.+....:%@@*##*=:..    ..%@-.         
        .*#-.      ::::.::-*@@@%: ....+.-#.               :%-.*.....#@@@#-:..:::-.     .-%#.        
      .-%*.                 -%@@@:  .#:+*.                 :*+-*. .#@@@-.                .*%-.      
     .=@+.                   .@@@= :#=+:                    .-+**..@@@=.                  .+@=.     
    .=@= ..                   =@@.-#::                        .-+#.*@%.                   ..=@=.    
    -@+.-:                   .+@-**.                             .%.%%.                   :-.+@:    
   .%#.*..                   *@-=-.:+:                         :=..#.%%.                   .*.#%.   
   =@:+:                    .@*.:.:=-=*.  ...:--=====--:..   .*-==...:@=                    :+-@-   
   #%.+                     .@+..-#+**-=. .:----==+==----.. .+-**+#-..@+                     +.%*   
  .##.:              ..      *@-. :+##+-..:..:--=====--:..:..-*%#=. .#%.      ..             :.##.  
   =@=...          ..:       .%@*....*=.. .-*-.........-*-. ..=*...:@@=       .-..         ...:@+   
   .-#@%=.         =..        .@@@:.+...- ...+*=-----=++... =..:+.+@@#.        ..-         =%@*-.   
     ..:*%=.     .*:   :      .=@@@-.==.-  .-+--:::::--==.  =.=-.*@@@.      .:   :+.    .-%%-..     
        .-%#:.  .*:    :.      .##@@:.%+: :=:.  .....  ..-- :=%.=@%%-.      .:    -*.  .*@=.        
          .+%#++%:     ::       .%+@%-%#..-..--==---=+=-:.::.#%:%%=#.       ::     -%++%#:          
            ..=@:      -:        *:#@@@= :.=+..::---::..-+.:.+@%@++-        :=      =@-.            
              :@*:..  .%.        .-:@@@. .*.-*-.     .:+=.+. :@@%.*.        .%.  .:=%@.             
              ..-=#%=.-=         .-.#@*. =:+:.         ..+-+ .#@+.+.         ==.+%*-:.              
                  .-#%+.          ..#@-  -*:.:==-----==-..*:  -@+..          .*@*:.                 
                    .@+:..         .%%.  .*:+-.:--==--::==+:  .%%.         .:-*%.                   
                     -*#%%=.      .*@*=-..#*.:=-------==.-%...:*@+.      .=%#*+:.                   
                        .:%%:....:#@@@@%@++==-....::...:+-%.+@@@@@*.....:@%.                        
                          .+%%#%@@@@@@#.:%@#.==:......:+:-@@#.+@@%%@%%%@@=..                        
                           ..:::@@@@@@:  .%@-.--:....:=-.*@+. .%@@@@@:::..                          
                                %@#%%#.  -@@+:.:-====-:.:%@*   =@@@@%.                              
                                +@#-%-  .#@@%--..::::.:-=@@%.  .%@*@*.                              
                                .%@+%.  :@@@@:--......=:+@@@*.  *%*@:                               
                                 -@@*  .*@@%%=.=*=--+*-.#@@@%.  :@@#                                
                                 .+@-  .%@@@@#.-:...:--:@@@%%-  .@@.                                
                                  .%-  :@%@@@%:.:----::-@@@-**. .@=.                                
                                  .%-  =%:@@@@::-::---.+@@*.=#. .@:                                 
                                   #+. +%.+@@@=...::.::#@%. =#. :@.                                 
                                   =%. -%..%@@+.=+=+=::@@=. +#..=#.                                 
                                   .%+ .@-+@@@*:-:.:=.=@@:..%+..@:.                                 
                                    :%=.*%=#@@*....:-.%@@%*=%..%=.                                  
                                     :#*:%+-+%+.:-: .=@@*-:%=+%=.                                   
                                      :@%**=*@=::::::%@%*:**%@%.                                    
                                      :@@#-.=%::=..:*@%%*.=#%@#                                     
                                     .@@@%%%@=-.=..*@@@+.  =@@%.                                    
                                    .*@=:-%@=:-:::#@@@%%%%%#%@@=                                    
                                   .#++***+-:-::-%@#-#@#-:-+@*+%.                                   
                                   :@=-:.::-:.-*%=%*.:%.   .#:=@.                                   
                                   .-+*#%+--=##+. :*#%%*++**%%#=.                                   
                                        .-++-..      ...:::...                                                                            
    "#;
    
    
        let inputs = Inputs {
            ascii,
            displacement: 100.0,
            top_left_corner:  [200.0, 800.0],
            character_spacing: 15.0,
            line_spacing: 25.0,
            approximate_characters_per_line: 100,
        };
        spawn_elephant_from_ascii(inputs)
}
    
fn spawn_elephant_from_ascii(mut inputs: Inputs) -> thread::JoinHandle<()> {
    let mut dummy = dummy_entity();

    let handle: thread::JoinHandle<()> = thread::spawn(move || {
        let rt = Builder::new_current_thread().enable_all().build().expect("Failed to build Tokio runtime");
        rt.block_on(async {
            let mut set = JoinSet::new();
            let mut time_keeper = TimeKeeper::new();

            let num_steps = 50; // Number of frames for full growth
            let final_character_spacing = inputs.character_spacing;
            let final_line_spacing = inputs.line_spacing;
            let mut scale_factor = 0.0;
            println!("Starting elephant animation");
            for _ in 0..num_steps {
                scale_factor += 0.01;

                inputs.character_spacing = final_character_spacing*scale_factor;
                inputs.line_spacing = final_line_spacing*scale_factor;

                render_ascii_elephant(&inputs, &mut set, &mut time_keeper).await;

                dummy.wait(150, &mut time_keeper).await; 
            }
            inputs.displacement = -inputs.displacement;
            for _ in 0..num_steps {
                scale_factor -= 0.01;

                inputs.character_spacing = final_character_spacing*scale_factor;
                inputs.line_spacing = final_line_spacing*scale_factor;

                render_ascii_elephant(&inputs, &mut set, &mut time_keeper).await;

                dummy.wait(150, &mut time_keeper).await; 
            }
        });
    });
    handle
}



async fn render_ascii_elephant(inputs: &Inputs, set: &mut JoinSet<()>, time_keeper: &mut TimeKeeper) {
    let mut character = 0.0;
    let mut line = 0.0;
    let mut colors: Vec<[f32; 3]> = vec![];

    for i in 0..inputs.approximate_characters_per_line {
        let current_entity_color_g = i as f32 / inputs.approximate_characters_per_line as f32;
        colors.push([0.0, current_entity_color_g, 0.5]);
    }

    for c in inputs.ascii.chars() {
        if c == ' ' {
            character += 1.0;
            continue;
        }
        if c == '\n' {
            line += 1.0;
            character = 0.0;
            continue;
        }

        let color = colors[character as usize];
        let position = [
            inputs.top_left_corner[0] + (character * inputs.character_spacing),
            inputs.top_left_corner[1] - (line * inputs.line_spacing),
        ];

        let duration = 150;
        let mut time_keeper_clone = time_keeper.clone();

        // let displacement = inputs.displacement;
        
        set.spawn(async move {
            let ent: Entity = Entity::new(position, 300.0, c, color);
            let ent2: Entity = Translate::new_entity(ent, duration, [position[0], position[1]]);
            let mut ent3: Entity = ColrRand::new_entity(ent2, 1);
            ent3.run_and_wait(&mut time_keeper_clone).await;
        });

        character += 1.0;
    }
}

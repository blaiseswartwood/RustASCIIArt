use crate::entity::time_keeper::TimeKeeper;
use crate::entity::Entity;
use crate::animations::translate::Translate;

use std::thread;
use tokio::runtime::Builder;
use tokio::task::JoinSet;
use rand::Rng;

use crate::example_animations::dummy_entity;

pub struct Inputs { 
    pub window_width: f32,
    pub window_height: f32,
    pub translation_speed: f32,
    pub ascii: &'static str,
    pub top_left_corner: [f32; 2],
    pub character_spacing: f32,
    pub characters_per_line: usize,
    pub line_spacing: f32,
    pub total_lines: usize,
    pub center_x_diff: f32,
    pub center_y_diff: f32,
}

pub fn run_dvd_animation() -> thread::JoinHandle<()> {
    println!("run_dvd_animation");
    let ascii = r"                                                                                       
          .:::::::::::::::::::::::::::::::::::.                 .:::::::::::::::::::::..               
         -%####################################-               =############################*+:        
         +#####################################*             .###################################+     
        :#%%%%%%%%%%%###########################=           -%########%%%%%%%%%%%%%################+   
                           +###########+########%:         *########%:                  :###########*. 
       .#########+           +#########%:########-        #########+ *#########-           %#########- 
       =#########:           .#########% +########.     +#########.  #########%            -#########= 
       %########*.           =#########* :%########   .#########-   =#########=            *########%: 
      -#########-           :#########%   =########: -########*     *#########            -#########=  
      %#########.          +##########     #########*########:     -#########+          :##########=   
     :#########+       .+###########=      .%##############=       =#########.       -*%#########%:    
     *#########%%%%%%############%-         =############%.       :%#########%%%%%##############.      
    :%#########################:             ###########-         -##########################.         
    +####################%=                   ########+           %#####################:              
                                              -#####%:                                                 
                                               *###+                                                   
                                               -%#.                                                    
                           .:--=+++++++++++++++++++++++++++++++++==-:.                                 
            .=+**##%#########################################################%%#***+-                  
     =##%###########:.-%###:.+####*.:%####+....:-%######-....+######+:....:*#############%#*.          
 .##################%: -%#  -#####*  %####=  ###: .#####:  ########  :%##%-  %#################-       
 .%##################%- .  -######*  %####=  ###=  #####:  ..+####*  *####+  *#################+       
    :#%%###############-  -#######*  %####=     .-%#####:    =#####*.      .##############%#+.         
          .:+*###%#############################################################%%##*+-.                
                        .:--=+++++********#############*********++++==-:..                -:++:        
                                                                                             .                                                                                                     
    ";               

    let inputs = Inputs {
        window_height: 1080.0, // y = 1080
        window_width: 1920.0, // x = 1920
        translation_speed: 0.2, // Smaller value = slower speed
        ascii,
        top_left_corner: [0.0, 270.0],
        character_spacing: 8.0,
        characters_per_line: 103,
        line_spacing: 10.0,
        total_lines: 27,
        center_x_diff: 0.0,
        center_y_diff: 0.0,
    };
    spawn_dvd(inputs)
}

pub fn spawn_dvd(mut inputs: Inputs) -> thread::JoinHandle<()> {
    let mut dummy = dummy_entity();
    
    let handle: thread::JoinHandle<()> = thread::spawn(move || {    //tokio runtime
        let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

        runtime.block_on(async {
            let mut set = JoinSet::new();
            let mut time_keeper = TimeKeeper::new();
            let bounce_amount = 12;
            
            println!("DVD ANIMATION");

            let mut top_left_pos = inputs.top_left_corner;
            let mut center_pos = get_center_of_animation(top_left_pos, &inputs);
            inputs.center_x_diff = center_pos[0] - top_left_pos[0];
            inputs.center_y_diff = top_left_pos[1] - center_pos[1];

            let mut center_end_pos = [
                inputs.window_width - inputs.center_x_diff,
                (inputs.window_height - inputs.center_y_diff) / 2.3,
            ];

            for _ in 0..bounce_amount {
                let distance = ((center_pos[0] - center_end_pos[0]).powi(2) + (center_pos[1] - center_end_pos[1]).powi(2)).sqrt();
                
                let duration = (distance / inputs.translation_speed) as i32;
                let top_left_end = get_top_left_of_animation(center_end_pos, &inputs);

                draw_dvd_animation(&mut set, &mut time_keeper, top_left_pos, top_left_end, duration, &inputs).await;
                
                let post_reflection_pos = find_center_reflection_pos(center_pos, center_end_pos, &inputs);
                
                center_pos = center_end_pos;
                top_left_pos = top_left_end;
                center_end_pos = post_reflection_pos;

                
                dummy.wait(duration as u64, &mut time_keeper).await;
            }
            // while let Some(_) = set.join_next().await {}
            while (set.join_next().await).is_some() {}
        });
    });
    handle
}

async fn draw_dvd_animation(set: &mut JoinSet<()>, time_keeper: &mut TimeKeeper, start_pos: [f32; 2], end_pos: [f32; 2], duration: i32, inputs: &Inputs) {

    let ascii = inputs.ascii;
    let character_spacing = inputs.character_spacing;
    let line_spacing = inputs.line_spacing;

    let mut rng = rand::thread_rng();
    let colors = [
        [1.0, 0.0, 0.0], // red
        [0.0, 1.0, 0.0], // green
        [0.0, 0.0, 1.0], // blue
        [1.0, 0.0, 1.0], // magenta
        [1.0, 1.0, 0.0], // yellow
        [0.0, 1.0, 1.0], // cyan
    ];
    let color = colors[rng.gen_range(0..colors.len())];

    let mut character = 0.0;
    let mut line = 0.0;
    let mut character_codes: Vec<u32> = vec![];
    for c in ascii.chars() {
        let mut found = false;
        if !character_codes.contains(&(c as u32)) {
            character_codes.push(c as u32);
            found = true;
        }
        if !found {
            character_codes.push(c as u32);
            // println!("character code {}", c as u32);
        }
        if c == ' ' {
            character += 1.0;
            continue;
        }
        if (c as u32 == 10) || (c == '\n') {
            line += 1.0;
            character = 0.0;
            // println!("line {}", line); //27 lines so this is working fine
            continue;
        }

        let adjusted_start_pos = [start_pos[0] + (character * character_spacing), start_pos[1] - (line * line_spacing)];
        let adjusted_end_pos = [end_pos[0] + (character*character_spacing), end_pos[1] - (line*line_spacing)];

        let mut time_keeper_clone = time_keeper.clone();   
        set.spawn(async move {
            let ent: Entity = Entity::new(adjusted_start_pos, 250.0, c, color);
            let mut ent2: Entity = Translate::new_entity(ent, duration, adjusted_end_pos);
            ent2.run_and_wait(&mut time_keeper_clone).await;

        });
        character += 1.0;
    }
    // while let Some(_) = set.join_next().await {}
}

fn get_center_of_animation(top_left: [f32; 2], inputs: &Inputs) -> [f32; 2] {
    // Calculate the center of the animation area
    let top_left_x = top_left[0];
    let top_left_y = top_left[1];
    let character_spacing = inputs.character_spacing;
    let line_spacing = inputs.line_spacing;
    let characters_per_line = inputs.characters_per_line;
    let total_lines = inputs.total_lines;
    let center_x = top_left_x + (character_spacing * (characters_per_line as f32) / 2.0);
    let center_y = top_left_y - (line_spacing * (total_lines as f32) / 2.0);
    [center_x, center_y]
}

fn get_top_left_of_animation(center: [f32; 2], inputs: &Inputs) -> [f32; 2] {
    // Calculate the top-left corner of the animation area
    let center_x = center[0];
    let center_y = center[1];
    let character_spacing = inputs.character_spacing;
    let line_spacing = inputs.line_spacing;
    let characters_per_line = inputs.characters_per_line;
    let total_lines = inputs.total_lines;
    let top_left_x = center_x - (character_spacing * (characters_per_line as f32) / 2.0);
    let top_left_y = center_y + (line_spacing * (total_lines as f32) / 2.0);
    [top_left_x, top_left_y]
}

fn find_center_reflection_pos(center_pos: [f32; 2], center_end_pos: [f32; 2], inputs: &Inputs) -> [f32; 2] {
    let window_x_min = inputs.center_x_diff;
    let window_x_max = inputs.window_width - inputs.center_x_diff;
    let window_y_min = inputs.center_y_diff;
    let window_y_max = inputs.window_height - inputs.center_y_diff;

    let epsilon = 1e-6;
    let mut rng = rand::thread_rng();
    let angle_variance: f32 = rng.gen_range(-0.01..=0.01); 
    let pi = std::f32::consts::PI;
    let pi_over_4 = pi / 4.0;

    let is_vertical_border = (center_end_pos[0] - window_x_min).abs() < epsilon || 
                            (center_end_pos[0] - window_x_max).abs() < epsilon;
    let is_horizontal_border = (center_end_pos[1] - window_y_min).abs() < epsilon || 
                              (center_end_pos[1] - window_y_max).abs() < epsilon;

    if !is_vertical_border && !is_horizontal_border {
        return center_end_pos;
    }

    let mut azimuth = (center_end_pos[1] - center_pos[1])
        .atan2(center_end_pos[0] - center_pos[0]);
    
    let angle_threshold = 5.0_f32.to_radians();

    let predefined_angles = [0.0, pi / 2.0, pi, 3.0 * pi / 2.0, -pi / 2.0, -pi, -3.0 * pi / 2.0];
    for &target_angle in &predefined_angles {
        if (azimuth - target_angle).abs() < angle_threshold {
            azimuth += pi_over_4 * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            break;
        }
    }

    let mut reflected_angle = match (is_vertical_border, is_horizontal_border) {
        (true, false) => pi - azimuth,
        (false, true) => -azimuth,
        (true, true) => azimuth + pi,
        _ => azimuth,
    };

    reflected_angle += angle_variance;

    let cos_reflected = reflected_angle.cos();
    let sin_reflected = reflected_angle.sin();

    let mut t_min = f32::INFINITY;
    let mut next_intersection = [0.0, 0.0];

    // Check left border
    if cos_reflected < -epsilon {
        let t = (window_x_min - center_end_pos[0]) / cos_reflected;
        let y = center_end_pos[1] + t * sin_reflected;
        if y >= window_y_min && y <= window_y_max && t < t_min {
            t_min = t;
            next_intersection = [window_x_min, y];
        }
    }

    // Check right border
    if cos_reflected > epsilon {
        let t = (window_x_max - center_end_pos[0]) / cos_reflected;
        let y = center_end_pos[1] + t * sin_reflected;
        if y >= window_y_min && y <= window_y_max && t < t_min {
            t_min = t;
            next_intersection = [window_x_max, y];
        }
    }

    // Check bottom border
    if sin_reflected < -epsilon {
        let t = (window_y_min - center_end_pos[1]) / sin_reflected;
        let x = center_end_pos[0] + t * cos_reflected;
        if x >= window_x_min && x <= window_x_max && t < t_min {
            t_min = t;
            next_intersection = [x, window_y_min];
        }
    }

    // Check top border
    if sin_reflected > epsilon {
        let t = (window_y_max - center_end_pos[1]) / sin_reflected;
        let x = center_end_pos[0] + t * cos_reflected;
        if x >= window_x_min && x <= window_x_max && t < t_min {
            // t_min = t;
            next_intersection = [x, window_y_max];
        }
    }

    next_intersection

}

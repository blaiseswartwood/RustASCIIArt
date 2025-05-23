#[macro_use]
extern crate glium;
pub mod fonts;
pub mod stresstest;
pub mod example_animations;
pub mod tests;
pub mod entity;
pub mod animations;

//use async_timer::oneshot::win;
use entity::global_ebo;
use entity::shared_entity_buffer::SharedEntityBuffer;
use entity::shared_entity::SharedEntity;
use entity::vbo::Vbo;
use entity::virtual_timer;
use glium::Surface;
use winit::dpi::LogicalPosition;
use winit::monitor::MonitorHandle;
use winit::window::WindowLevel; //Fullscreen
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use std::thread::sleep;
use std::time::Duration;
use std::time::Instant; //nanoseconds times

static IS_ANIMATING: AtomicBool = AtomicBool::new(true);


fn main() {  
    println!("Starting Application");
    println!("SharedEntity = {}", std::mem::size_of::<SharedEntity>());
    println!("SharedEntityBuffer = {}", std::mem::size_of::<SharedEntityBuffer>());
    system_setup(); 
}

fn system_setup() {
    
    global_ebo::smem_init_and_branch();

    /* ------------------------ ENTITY BUFFER OBJECT ------------------------ */
    // let ebo: (bool, Vec<entity::Entity>) = (false, vec![]);

    /* ------------------------ SETTING UP ASYNC ------------------------ */

    // let animation_context: AnimationContext = Arc::new(Mutex::new(SharedContext {ebo}));
    //let mut handles = vec![];

    // ---- BASIC VISUAL TESTS ----
    // handles.push(tests::run_tests());

    // ---- ANIMATIONS ----
    // let animation_handles: Vec<thread::JoinHandle<()>> = example_animations::run_animations();
    // for handle in animation_handles {
    //     handles.push(handle);
    // }

    // ---- STRESS TESTS ----
    // let stress_test_handles: Vec<thread::JoinHandle<()>> = stresstest::run_animations();
    // for handle in stress_test_handles {
    //     handles.push(handle);
    // }

    // event_loop_drawing(atlas, atlas_map, max_num_characters as u32);

    // for handle in handles {
    //     handle.join().expect(" You were the Chosen One! It was said that you would destroy the Sith, not join them! ");
    // }
}

fn event_loop_drawing(atlas: image::DynamicImage, atlas_map: [[f32; 4]; 512], max_num_characters: u32) {
    /* ------------------------ GLSL DECLARATIONS --reen---------------------- */
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("Rectangle Created");
    
    let vertex_shader_src = include_str!("shaders/vertex_shader.glsl");
    let fragment_shader_src = include_str!("shaders/fragment_shader.glsl");
    
    /* ------------------------ EVENT LOOP WINDOW CREATE ------------------------ */
    let monitor: MonitorHandle = event_loop.primary_monitor().unwrap();
    let monitor_size = monitor.size(); //1920x1080
    let monitor_position = monitor.position(); //0x0
    println!("Monitor Size: {:?}", monitor_size);
    println!("Monitor Position: {:?}", monitor_position);

    let window_builder = winit::window::WindowBuilder::new()
    .with_decorations(false)
    .with_transparent(true)
    // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None))) // Want this but background stops being rendered
        // If all else breaks can disable windows fullscreen optimizations to force it to render
    .with_inner_size(winit::dpi::PhysicalSize::new(monitor_size.width, monitor_size.height-1)) // Sets the window size to the monitor size
    .with_window_level(WindowLevel::AlwaysOnTop) // Displays animations on top of everything else
    .with_title("ASCII Art");

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(window_builder).build(&event_loop);
    window.set_outer_position(LogicalPosition::new(0.0, 0.0));
    if let Err(e) = window.set_cursor_hittest(false) { // Allows the cursor to click through the window
        eprintln!("Failed to set cursor hit test: {:?}", e);
    }

    println!("Scale Factor: {}", window.scale_factor());

    let mut window_size  = [monitor_size.width as f32, monitor_size.height as f32]; // in logical pixels, will be different from the actual window size on a particular device, will probably cause "inital jump" when being initialized
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let background_color = [0.0, 0.0, 0.0, 0.0];
    let blend = glium::Blend {
        color: glium::BlendingFunction::Addition {
            source: glium::LinearBlendingFactor::SourceAlpha,
            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
        },
        alpha: glium::BlendingFunction::Addition {
            source: glium::LinearBlendingFactor::SourceAlpha,
            destination: glium::LinearBlendingFactor::OneMinusSourceAlpha,
        },
        ..Default::default()
    };
    let mut previous_instant = Instant::now();
    
    /* ------------------------ EVENT LOOP FONT ------------------------ */
    let atlas_imagebuffer = atlas.to_rgba8();
    let image_atlas_dimensions = atlas_imagebuffer       .dimensions();
    let atlas_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&atlas_imagebuffer.into_raw(), image_atlas_dimensions);
    let texture_atlas = glium::texture::Texture2d::new(&display, atlas_image).unwrap();    
    let atlas_buffer = glium::uniforms::UniformBuffer::new(
        &display,
        atlas_map,
    ).unwrap();

    println!("Starting event loop");
    /* ------------------------ EVENT LOOP VBO ------------------------ */
    let mut vbo = Vbo::new();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(_window_size) => {
                    window_size = [_window_size.width as f32, _window_size.height as f32];
                },
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(background_color[0], background_color[1], background_color[2], background_color[3]);

                    let uniforms = uniform! {
                        atlas: &texture_atlas,
                        time: virtual_timer::TIME_VALUE.load(Ordering::Relaxed) as i32,
                        windowSize: window_size,
                        atlas_buffer: &atlas_buffer,
                        glyph_count: max_num_characters as i32,
                    };
                    
                    // {
                    //     let ebo = &mut animation_context.lock().unwrap().ebo;
                    //     if ebo.0 {
                    //         vbo = entity::global_ebo::gen_vbo();
                    //     }
                    // }
                    //println!("HERE?");
                    // if global_ebo::need_update() {
                    // println!("Updating EBO");
                    vbo = global_ebo::gen_vbo();
                    // }
                    println!("Ebo size: {}", vbo.vertex_buffer.len());
                    
                    //println!("How far?");
                    let vertex_buffer = glium::VertexBuffer::new(
                        &display, 
                        &vbo.vertex_buffer
                    ).unwrap();
                    // println!("Vertex count: {}", vertex_buffer.len());
                    let index_buffer = glium::IndexBuffer::new(
                        &display,
                        glium::index::PrimitiveType::TrianglesList,
                        &vbo.index_buffer,
                    ).unwrap();

                    let draw_parameters = glium::DrawParameters {
                        blend,
                        ..Default::default()
                    };

                    /* ------------------------ TIME STEP ------------------------ */
                    let step_nanos = Instant::now().duration_since(previous_instant).as_nanos();
                    let step_millis = (step_nanos as f64 / 1_000_000.0f64).round();
                    virtual_timer::step_time(step_millis as u64);
                    global_ebo::update_time(step_millis as u64);
                    previous_instant = Instant::now();

                    /* ------------------------ DRAWING ------------------------ */
                    target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_parameters).unwrap();
                    target.finish().unwrap();
                    // println!("Time: {}", current_time);
                    // let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i32;
                    // let step = (current_time - previous_time) as f32 / 1000.0f32;  
                    // previous_time  = current_time;
                    // if system_start_time + 1000 < current_time {
                    //     if step > maxlag {
                    //         maxlag = step;
                    //         println!("max lag: {}", maxlag);
                    //     }
                    // }
                    // if step > 0.1 {
                    //     println!("lag time: {}", current_time);
                    // }
                    // if step > 0.01 {
                    //     lagcount += 1;
                    //     println!("lagging: {}, lag_time: {}", lagcount, step);
                    // }

                },
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                // Check if the animation is finished
                if IS_ANIMATING.load(Ordering::SeqCst) {
                    window.request_redraw();
                } else {
                    println!("Animation finished. Exiting program.");
                    window_target.exit(); // Exit the event loop
                }
            },
            _ => (),
        };
    });
}


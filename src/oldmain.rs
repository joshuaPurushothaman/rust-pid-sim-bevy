// // extern crate sdl2;

// // use cube_phys::Cube;
// // use sdl2::event::Event;
// // use sdl2::keyboard::Keycode;
// // use std::time::{Duration, Instant};

// // use crate::engine::vec2f::Vec2f;

// // mod cube_phys;
// // pub mod engine;
// // mod pid;

// pub fn main() {
//     // const WIDTH: u32 = 1000;
//     // const HEIGHT: u32 = 1000;

//     let mut x_controller =
//         pid::PIDController::new(10.0, 0.0, 1.0, -f32::INFINITY, f32::INFINITY, f32::INFINITY);
//     let mut y_controller = x_controller.clone();

//     let mut cube = Cube::new(30.0, 1.0);

//     let center = Vec2f {
//         x: (WIDTH / 2) as f32,
//         y: (HEIGHT / 2) as f32,
//     };
//     cube.pose = center;

//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let window = video_subsystem
//         .window("Rustic PID Sim", WIDTH, HEIGHT)
//         .position_centered()
//         .build()
//         .unwrap();

//     let mut canvas = window.into_canvas().build().unwrap();

//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump().unwrap();

//     let mut last_time = Instant::now();

//     'running: loop {
//         canvas.clear();
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 _ => {}
//             }
//         }
//         // The rest of the game loop goes here...
//         let dt = Instant::now() - last_time;
//         last_time = Instant::now();

//         let state = event_pump.mouse_state();

//         let x_force = x_controller.calculate(dt, state.x() as f32, cube.pose.x);
//         let y_force = y_controller.calculate(dt, state.y() as f32, cube.pose.y);

//         let mut net_force = Vec2f {
//             x: x_force,
//             y: y_force,
//         };

//         // Add air resistance
//         net_force -= cube.vel * 0.1;

//         // Static friction
//         if net_force.length() < 1.0 {
//             net_force = Vec2f { x: 0.0, y: 0.0 };
//         }

//         // Max force
//         let max_force = 512.0; //  TODO: parametric? via... CLI args, GUI sliders, TOML config... hmm.
//         if net_force.length() > max_force {
//             net_force.scale_to_length(max_force);
//         }

//         cube.set_force(net_force);
//         cube.update(dt);
//         cube.draw(&mut canvas);

//         // END Game loop
//         canvas.present();

//         // TODO: more sophistication; SDL has a FPSmanager.
//         std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 75));
//     }
// }

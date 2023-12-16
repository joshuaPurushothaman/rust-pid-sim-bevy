// // use std::time::Duration;

// // use sdl2::{pixels::Color, render::Canvas, video::Window};
// use bevy::prelude::*;
// use bevy::math::Vec2;

// #[derive(Component)]
// pub struct Cube {
//     pub side_length: f32,
//     pub mass: f32,
//     pub pose: Vec2,
//     pub vel: Vec2,
//     pub force: Vec2,
//     pub color: Color,
// }

// impl Cube {
//     pub fn new(side_length: f32, mass: f32) -> Cube {
//         Cube {
//             side_length,
//             mass,
//             pose: Vec2 { x: 0.0, y: 0.0 },
//             vel: Vec2 { x: 0.0, y: 0.0 },
//             force: Vec2 { x: 0.0, y: 0.0 },
//             color: Color {
//                 r: 3,
//                 g: 252,
//                 b: 119,
//                 a: 255,
//             },
//         }
//     }

//     pub fn draw(&self, canvas: &mut Canvas<Window>) {
//         // Note: draws cube with pose as its center
//         let original_color = canvas.draw_color();
//         canvas.set_draw_color(self.color);

//         let center = self.pose - (self.side_length / 2.0);

//         canvas
//             .fill_rect(sdl2::rect::Rect::new(
//                 center.x as i32,
//                 center.y as i32,
//                 self.side_length as u32,
//                 self.side_length as u32,
//             ))
//             .unwrap();

//         canvas.set_draw_color(Color {
//             r: 255,
//             g: 0,
//             b: 0,
//             a: 0,
//         });

//         canvas
//             .fill_rect(sdl2::rect::Rect::new(
//                 center.x as i32,
//                 center.y as i32,
//                 5,
//                 5,
//             ))
//             .unwrap();

//         canvas.set_draw_color(original_color);
//     }

//     pub fn update(&mut self, dt: Duration) {
//         let dt_s = dt.as_secs_f32();

//         self.vel += (self.force / self.mass) * dt_s;

//         self.pose += self.vel * dt_s;
//     }

//     pub fn set_force(&mut self, force: Vec2) {
//         self.force = force;
//     }
// }

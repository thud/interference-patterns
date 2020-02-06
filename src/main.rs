extern crate minifb;

use minifb::{Key, Window, WindowOptions, MouseMode, MouseButton, Scale, ScaleMode};
use std::{thread, time};

const WIDTH: usize = 1000;
const HEIGHT: usize = 600;

const SCALE: f32 = 5.0;

const SOURCE_A: (f32, f32) = (0.0, HEIGHT as f32 /3.0);
const SOURCE_B: (f32, f32) = (0.0, 2.0*HEIGHT as f32/3.0);

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut sources: Vec<(f32, f32)> = vec![];
    //sources.push(SOURCE_A);
    //sources.push(SOURCE_B);

    let mut window = Window::new(
        "Interference Patterns (click to place a source)",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch
        }
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut t: f32 = 0.0;
    let mut dt: f32 = 0.1;
    let mut intensity: f32 = 50.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mouse = window.get_mouse_pos(MouseMode::Clamp).unwrap();
        if window.get_mouse_down(MouseButton::Left) {
            //println!("new source! at ({}, {})", mouse.0, mouse.1);
            sources.push(mouse);
            thread::sleep(time::Duration::from_millis(100));
        }

        if window.is_key_down(Key::Down) { dt -= 0.01; }
        if window.is_key_down(Key::Up)   { dt += 0.01; }
        if window.is_key_down(Key::Left) { intensity -= 1.0; }
        if window.is_key_down(Key::Right){ intensity += 1.0; }

        if intensity > 150.0 { intensity = 150.0 }
        if intensity < 0.0   { intensity = 0.0 }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
//                let dist_from_a: f32 = ((x as f32 - SOURCE_A.0 as f32).powf(2.0) + (y as f32 - SOURCE_A.1 as f32).powf(2.0)).sqrt();
//                let dist_from_b: f32 = (((x as f32 - SOURCE_B.0 as f32).powf(2.0) + (y as f32 - SOURCE_B.1 as f32).powf(2.0))).sqrt();

                let mut superpos: f32 = 0.0;

                for s in sources.iter_mut() {
                    let dist: f32 = ((x as f32 - s.0).powf(2.0) + (y as f32 - s.1).powf(2.0)).sqrt();
                    superpos += f32::sin(dist/SCALE - t);
                }

                let mut newIntensity = (superpos.abs() * intensity as f32) as u32;
                if newIntensity > 255 {
                    newIntensity = 255;
                }

                buffer[WIDTH*y + x] = newIntensity; 
                //println!("{}", (f32::sin(dist_from_a*50.0).powf(2.0) * 10.0) as u32);

                //buffer[WIDTH*y + x] = ((f32::sin(dist_from_a/5.0 - t) + f32::sin(dist_from_b/5.0 - t)).powf(2.0) * 50.0).round() as u32; //dist_from_a as u32;

            }
        }
        t += dt;

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

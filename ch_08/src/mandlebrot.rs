use nannou::{
    color::IntoLinSrgba,
    image::{DynamicImage, GenericImage},
    prelude::*,
};
use rayon::prelude::*;

const INITIAL_CYCLE_LIMIT: u32 = 100;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const STARTING_FRAME: u32 = 680;
const CYCLE_GROWTH_RATE: f64 = 2.0;

fn main() {
    let mut model = Model::new();

    for n in STARTING_FRAME..1000 {
        model.scale *= 0.95;
        model.update_pixels(n);
        create_image(&model, n);
    }
}

struct Model {
    pixels: Vec<Vec<u32>>,
    scale: f64,
    offset: (f64, f64),
}

impl Model {
    fn new() -> Self {
        let w: usize = WIDTH as usize;
        let h: usize = HEIGHT as usize;
        Model {
            pixels: vec![vec![0; w]; h],
            scale: 1.0 * 0.95.pow(STARTING_FRAME as f64),
            offset: (-0.743643887037151, 0.131825904205330),
        }
    }

    fn update_pixels(&mut self, frame_number: u32) {
        let h = self.pixels.len();
        let limit = ( INITIAL_CYCLE_LIMIT as f64 * (frame_number as f64).powf(CYCLE_GROWTH_RATE)) as u32;
        self.pixels.par_iter_mut().enumerate().for_each(|(i, row)| {
            let w = row.len();
            row.iter_mut().enumerate().for_each(|(j, pixel)| {
                let a = map_range(
                    i,
                    0,
                    h,
                    -self.scale + self.offset.0,
                    self.scale + self.offset.0,
                );
                let b = map_range(
                    j,
                    0,
                    w,
                    -self.scale + self.offset.1,
                    self.scale + self.offset.1,
                );

                *pixel = mandlebrot(a, b, limit);
            });
        });
    }
}

fn mandlebrot(a: f64, b: f64, limit: u32) -> u32 {
    let mut x = a.clone();
    let mut y = b.clone();
    let mut iteration = 0;
    while x * x + y * y < 4.0 && iteration < limit{
        let xtemp = x * x - y * y + a;
        y = 2.0 * x * y + b;
        x = xtemp;
        iteration += 1;
    }
    iteration
}

fn create_image(model: &Model, frame_number: u32) {
    let mut image: DynamicImage = DynamicImage::new_rgba8(WIDTH, HEIGHT);
    let limit = INITIAL_CYCLE_LIMIT as f64 * (frame_number as f64).powf(CYCLE_GROWTH_RATE);
    for (i, row) in model.pixels.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == limit as u32 {
                continue;
            }

            let color = hsl(
                (((value as f64 / limit * 360.0).powf(1.5) % 360.0) / 360.0) as f32,
                0.5,
                (value as f64 / limit) as f32,
            );
            let color = color.into_lin_srgba();
            let color = rgba(
                (color.red * 255.0) as u8,
                (color.green * 255.0) as u8,
                (color.blue * 255.0) as u8,
                (color.alpha * 255.0) as u8,
            );

            image.put_pixel(
                i as u32,
                j as u32,
                nannou::image::Rgba([color.red, color.green, color.blue, color.alpha]),
            );
        }
    }
    image
        .save(format!("mandelbrot_{}.png", frame_number))
        .unwrap();
}

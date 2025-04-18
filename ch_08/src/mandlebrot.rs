use nannou::{color::IntoLinSrgba, image::{DynamicImage, GenericImage}, prelude::*};
use rayon::prelude::*;

fn main() {
    nannou::app(model)
        .loop_mode(nannou::app::LoopMode::NTimes { number_of_updates: 1000 })
        .update(update)
        .run();
}

const CYCLE_LIMIT: u32 = 10000;
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

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
            scale: 1.0,
            offset: (-0.743643887037151, 0.131825904205330),
        }
    }

    fn update_pixels(&mut self) {
        let h = self.pixels.len();
        self.pixels.par_iter_mut().enumerate().for_each(|(i, row)| {
            let w = row.len();
            row.iter_mut().enumerate().for_each(|(j, pixel)| {
                *pixel = mandlebrot(
                    map_range(
                        i,
                        0,
                        h,
                        -self.scale + self.offset.0,
                        self.scale + self.offset.0,
                    ),
                    map_range(
                        j,
                        0,
                        w,
                        -self.scale + self.offset.1,
                        self.scale + self.offset.1,
                    ),
                );
            });
        });
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        // .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mut model = Model::new();
    model.update_pixels();
    model
}

fn mandlebrot(a: f64, b: f64) -> u32 {
    let mut x = a.clone();
    let mut y = b.clone();
    let mut iteration = 0;
    while x * x + y * y < 4.0 && iteration < CYCLE_LIMIT {
        let xtemp = x * x - y * y + a;
        y = 2.0 * x * y + b;
        x = xtemp;
        iteration += 1;
    }
    iteration
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    // keypress events
    match key {
        Key::I => model.scale *= 0.6,
        Key::O => model.scale *= 1.1,
        Key::W => model.offset.1 += 0.1 * model.scale,
        Key::S => model.offset.1 -= 0.1 * model.scale,
        Key::A => model.offset.0 -= 0.1 * model.scale,
        Key::D => model.offset.0 += 0.1 * model.scale,
        _ => (),
    }
    model.update_pixels();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // update the model
    model.scale *= 0.95;
    model.update_pixels();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let mut image: DynamicImage = DynamicImage::new_rgba8(WIDTH, HEIGHT);
    for (i, row) in model.pixels.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == CYCLE_LIMIT {
                continue;
            }

            let color = hsl(
                (((value as f64 / CYCLE_LIMIT as f64 * 360.0).powf(1.5) % 360.0) / 360.0)
                    as f32,
                0.5,
                (value as f64 / CYCLE_LIMIT as f64) as f32,
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
                nannou::image::Rgba([color.red, color.green, color.blue, color.alpha])
            );
        }
    }
    image.save(format!("mandelbrot_{}.png", frame.nth())).unwrap();

    // let h = HEIGHT as f32;
    // let w = WIDTH as f32;
    // let texture = wgpu::Texture::from_image(
    //     app,
    //     &image,
    // );
    // draw.texture(&texture)
    //     .x_y(0.0, 0.0)
    //     .w_h(w, h);


    // draw.to_frame(app, &frame).unwrap();
}

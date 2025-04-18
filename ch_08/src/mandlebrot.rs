use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

const CYCLE_LIMIT: u32 = 5000;
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;

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
        let w: usize = WIDTH as usize;
        let h: usize = HEIGHT as usize;
        for i in 0..self.pixels.len() {
            for j in 0..self.pixels[i].len() {
                self.pixels[i][j] = mandlebrot(
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
            }
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);
    let h = HEIGHT as f32;
    let w = WIDTH as f32;

    for (i, row) in model.pixels.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == CYCLE_LIMIT {
                continue;
            }

            draw.rect()
                .x_y(
                    map_range(i as f32, 0.0, w, -w / 2.0, w / 2.0),
                    map_range(j as f32, 0.0, h, -h / 2.0, h / 2.0),
                )
                .w_h(1.0, 1.0)
                .color(hsl(
                    (((value as f64 / CYCLE_LIMIT as f64 * 360.0).powf(1.5) % 360.0) / 360.0)
                        as f32,
                    0.5,
                    (value as f64 / CYCLE_LIMIT as f64) as f32,
                ));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

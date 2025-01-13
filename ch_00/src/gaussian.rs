use nannou::prelude::*;
use rand;
use rand_distr::{Distribution, Normal};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn get_random_normal() -> f32 {
    let dist = Normal::new(0.0, 1.0).unwrap();
    let v = dist.sample(&mut rand::thread_rng()) as f32;
    map_range(v, -5.0, 5.0, -200.0, 200.0)
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(400, 400).view(view).build().unwrap();
    let x = get_random_normal();
    let y = get_random_normal();
    Model { x, y }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.x = get_random_normal();
    model.y = get_random_normal();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    draw.ellipse()
        .radius(2.0)
        .x_y(model.x, model.y)
        .rgba(0.5, 0.5, 0.5, 0.2);

    draw.to_frame(app, &frame).unwrap();
}

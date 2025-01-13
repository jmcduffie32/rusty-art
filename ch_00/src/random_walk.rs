use nannou::{noise::{self, NoiseFn}, prelude::*};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;
    let x: f32 = 0.0;
    let y: f32 = 0.0;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    Model { x, y }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let x_rand = random_range(-1, 2);
    let y_rand = random_range(-1, 2);

    let perlin = noise::Perlin::new();
    let value = perlin.get([model.x as f64, model.y as f64, app.time as f64]);


    if random_f32() < 0.1 {
    // if false {
        if app.mouse.position().x > model.x {
            model.x += 1.0;
        } else {
            model.x -= 1.0;
        }

        if app.mouse.position().y > model.y {
            model.y += 1.0;
        } else {
            model.y -= 1.0;
        }
    } else {
        model.x += x_rand as f32 + value as f32;
        model.y += y_rand as f32 + value as f32;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(5.0, 5.0)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

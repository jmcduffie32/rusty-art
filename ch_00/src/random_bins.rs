use nannou::prelude::*;
use std::collections::HashMap;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
  bins: HashMap<i32, i32>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(400,400).view(view).build().unwrap();
    let bins = HashMap::new();
    Model { bins }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let random_int = random_range(0, 10);
    let count = model.bins.entry(random_int).or_insert(0);
    *count += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    // draw a rectangle for each bin
    for (i, count) in model.bins.iter() {
        let height = *count as f32 * 0.1;
        draw.rect()
            .w_h(40.0,height) 
            .x_y(-200.0 + (*i as f32) * 40.0 + 20.0, 0.0 + height / 2.0)
            .rgba(0.5, 0.5, 0.5, 1.0)
            .stroke_weight(2.0)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

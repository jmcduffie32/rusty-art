use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    t: f32,
    points: Vec<Point2>,
    noise: Perlin,
}

fn generate_points(noise: Perlin, t: f32) -> Vec<Point2> {
    let x_min: i32 = -400;
    let x_max: i32 = 400;
    let points: Vec<Vec2> = (x_min..x_max)
        .map(|i: i32| {
            let x: f32 = map_range(i as f32, x_min as f32, x_max as f32, -400.0, 400.0) as f32;
            let scaled_i: f32 = i as f32 * 0.001;
            let y: f32 = noise.get([(t + scaled_i as f32) as f64, 0.0]) as f32;
            let mapped_y: f32 = y * 200.0;
            Point2::new(x, mapped_y)
        })
        .collect();
    points
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;
    let t: f32 = 0.015;
    let noise: Perlin = Perlin::new();
    let points: Vec<Vec2> = generate_points(noise.clone(), t);

    let _window: WindowId = app.new_window().size(w, h).view(view).build().unwrap();
    Model { t, points, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.t += 0.005;
    model.points = generate_points(model.noise.clone(), model.t);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    draw.background().color(BLACK);

    (0..20).for_each(|i| {
        draw.polyline()
            .weight(1.0)
            .points(
            model
                .points
                .clone()
                .into_iter()
                .map(|p| [p.x + random_range(0, 100) as f32, -p.y]),
)
            .color(RED)
            .rotate(map_range(i as f32, 0.0, 20.0, 0.0, 2.0 * PI));
    });

    draw.polyline()
        .weight(1.0)
        .points(
            model
                .points
                .clone()
                .into_iter()
                .map(|p| [p.x + random_range(0, 100) as f32, p.y]),
        )
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}

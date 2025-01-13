use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    position: Point3,
    velocity: Vec3,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;
    let position = pt3(0.0, 0.0, 0.0);
    let velocity = vec3(2.5, 2.0, 1.2);

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    Model { position, velocity }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.position += model.velocity;

    let win_rect = app.window_rect();

    if model.position.x > win_rect.right() || model.position.x < win_rect.left() {
        model.velocity.x = -1.0 * model.velocity.x;
    }

    if model.position.y > win_rect.top() || model.position.y < win_rect.bottom() {
        model.velocity.y = -1.0 * model.velocity.y;
    }

    if model.position.z > 400.0 || model.position.z < -400.0 {
        model.velocity.z = -1.0 * model.velocity.z;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    draw.background().color(WHITE);

    let scale: f32 = (model.position.z + 400.0) / 800.0;
    draw.ellipse()
        .x_y(model.position.x, model.position.y)
        .w_h(50.0 * scale, 50.0 * scale)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

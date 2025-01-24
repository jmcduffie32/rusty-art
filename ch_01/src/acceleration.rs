use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Mover {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    top_speed: f32,
}

impl Mover {
    fn new() -> Self {
        let position = pt2(0.0, 0.0);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let top_speed = 5.0;
        Mover {
            position,
            velocity,
            acceleration,
            top_speed,
        }
    }

    fn update(&mut self) {
        if (self.velocity + self.acceleration).length() <= self.top_speed {
            self.velocity += self.acceleration;
        };
        self.position += self.velocity;
    }

    fn check_edges(&mut self, app: &App) {
        if self.position.x > app.window_rect().right() {
            self.position.x = app.window_rect().left();
        } else if self.position.x < app.window_rect().left() {
            self.position.x = app.window_rect().right();
        }

        if self.position.y > app.window_rect().top() {
            self.position.y = app.window_rect().bottom();
        } else if self.position.y < app.window_rect().bottom() {
            self.position.y = app.window_rect().top();
        }
    }

    fn accelerate(&mut self, jerk: Vec2) {
        self.acceleration += jerk;
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(16.0, 16.0)
            .rgba(0.5, 0.5, 0.5, 1.0)
            .stroke(BLACK);

        draw.arrow()
            .start(self.position)
            .end(self.position + (self.velocity * 100.0))
            .weight(2.0)
            .color(BLACK);

        draw.text(&format!(
            "Current velocity: {x:.3},{y:.3}",
            x = self.velocity.x,
            y = self.velocity.y
        ))
        .xy(pt2(-250.0, 380.0))
        .w(800.0)
        .font_size(20)
        .color(BLACK);

        draw.text(&format!(
            "Current acceleration:  {x:.3},{y:.3}",
            x = self.acceleration.x,
            y = self.acceleration.y
        ))
        .xy(pt2(-230.0, 340.0))
        .w(800.0)
        .font_size(20)
        .color(BLACK);
    }
}

struct Model {
    mover: Mover,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app
        .new_window()
        .size(w, h)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let mover = Mover::new();
    Model { mover }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Up => {
            let jerk = vec2(0.0, 0.001);
            model.mover.accelerate(jerk);
        }
        Key::Down => {
            let jerk = vec2(0.0, -0.001);
            model.mover.accelerate(jerk);
        }
        Key::Right => {
            let jerk = vec2(0.001, 0.0);
            model.mover.accelerate(jerk);
        }
        Key::Left => {
            let jerk = vec2(-0.001, 0.0);
            model.mover.accelerate(jerk);
        }
        _ => {}
    };
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.mover.update();
    model.mover.check_edges(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.mover.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

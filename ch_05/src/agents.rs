use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Vehicle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    max_speed: f32,
    max_acceleration: f32,
}

impl Vehicle {
    fn new() -> Self {
        let position = pt2(0.0, 0.0);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let max_speed = 1.0;
        let max_acceleration = 0.005;

        Vehicle {
            position,
            velocity,
            acceleration,
            max_speed,
            max_acceleration,
        }
    }

    fn update(&mut self, target: Point2) {
        let displacement_vector = target - self.position;
        let d = displacement_vector.length();
        let desired_velocity;
        if d < 100.0 {
            let m = map_range(d, 0.0, 100.0, 0.0, self.max_speed);
            desired_velocity = displacement_vector.normalize() * m;
        } else {
            desired_velocity = displacement_vector.normalize() * self.max_speed;
        }
        self.acceleration =
            (desired_velocity - self.velocity).clamp_length_max(self.max_acceleration);
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = vec2(0.0, 0.0);
    }

    fn display(&self, draw: &Draw) {
        draw.tri()
            .points(
                pt2(10.0, 0.0),
                pt2(10.0, 0.0).rotate(2.5 * PI / 3.0),
                pt2(10.0, 0.0).rotate(-2.5 * PI / 3.0),
            )
            .x_y(self.position.x, self.position.y)
            .rotate(self.velocity.angle())
            .color(BLACK);
    }
}

struct Model {
    vehicle: Vehicle,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let vehicle = Vehicle::new();
    Model { vehicle }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.vehicle.update(app.mouse.position());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.vehicle.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

use itertools::Itertools;
use nannou::prelude::*;

const G: f32 = 1.0e5;

fn main() {
    nannou::app(model).update(update).run();
}

struct Body {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    net_force: Vec2,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32) -> Self {
        Body {
            position,
            velocity,
            acceleration,
            mass,
            net_force: vec2(0.0, 0.0),
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.net_force += force;
    }

    // fn remove_force(&mut self, force: Vec2) {
    //     self.net_force -= force;
    // }

    fn reset_force(&mut self) {
        self.net_force = vec2(0.0, 0.0);
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(2.0, 2.0)
            .color(WHITE);

        // draw.arrow()
        //     .start(self.position)
        //     .end(self.position + (self.velocity))
        //     .weight(2.0)
        //     .color(BLACK);
    }
}

struct Model {
    bodies: Vec<Body>,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let mut bodies = vec![];
    let n = 3;
    for i in 0..n {
        let position: Vec2 = vec2(1.0, 0.0).rotate(i as f32 * TAU / n as f32) * 110.0;
        let velocity: Vec2 = vec2(0.0, 25.0).rotate(i as f32 * TAU / n as f32);
        let acceleration: Vec2 = vec2(0.0, 0.0);
        let mass: f32 = 1.0;
        bodies.push(Body::new(position, velocity, acceleration, mass));
    }
    Model { bodies }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let delta_t: f32 = 1.0 / 60.0;

    // apply gravitational force between each pair of bodies
    for (i, j) in (0..model.bodies.len()).tuple_combinations() {
        let (left, right) = model.bodies.split_at_mut(j);
        let body1: &mut Body = &mut left[i];
        let body2: &mut Body = &mut right[0];
        let displacement: Vec2 = body2.position - body1.position;
        // kludge to avoid division by zero
        if displacement.length() <= 1.0 {
            continue;
        }
        let force: Vec2 =
            G * (body1.mass * body2.mass) / displacement.length().pow(2) * displacement.normalize();

        body1.apply_force(force);
        body2.apply_force(-force);
    }

    for body in model.bodies.iter_mut() {
        body.acceleration = body.net_force / body.mass;
        body.velocity += body.acceleration * delta_t / 2.0;
        body.position += body.velocity * delta_t;
        body.reset_force();
    }

    // apply gravitational force between each pair of bodies
    for (i, j) in (0..model.bodies.len()).tuple_combinations() {
        let (left, right) = model.bodies.split_at_mut(j);
        let body1: &mut Body = &mut left[i];
        let body2: &mut Body = &mut right[0];
        let displacement: Vec2 = body2.position - body1.position;
        // kludge to avoid division by zero
        if displacement.length() <= 1.0 {
            continue;
        }
        let force: Vec2 =
            G * (body1.mass * body2.mass) / displacement.length().pow(2) * displacement.normalize();

        body1.apply_force(force);
        body2.apply_force(-force);
    }

    for body in model.bodies.iter_mut() {
        body.acceleration = body.net_force / body.mass;
        body.velocity += body.acceleration * delta_t / 2.0;
        body.reset_force();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
        draw.background().color(LIGHTBLUE);
    }

    for body in model.bodies.iter() {
        body.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

use itertools::Itertools;
use nannou::prelude::*;

const ELECTRIC_CONSTANT: f32 = 1.0e2;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone)]
struct Body {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    net_force: Vec2,
    alive: bool,
    charge: f32,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32) -> Self {
        Body {
            position,
            velocity,
            acceleration,
            mass,
            net_force: vec2(0.0, 0.0),
            alive: true,
            charge: random_range(-1.0, 1.0)
        }
    }

    fn get_color(&self) -> Rgb {
        if self.charge > 0.0 {
            return rgb(self.charge, 0.0, 1.0 - self.charge);
        } else if self.charge < 0.0 {
            return rgb(1.0 - self.charge, 0.0, self.charge);
        } else {
            return rgb(1.0, 1.0, 1.0);
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        self.net_force += force;
    }

    fn reset_force(&mut self) {
        self.net_force = vec2(0.0, 0.0);
    }

    fn radius(&self) -> f32 {
        self.mass.log2()
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.radius())
            .color(self.get_color());
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
    let n = 500;
    for _i in 0..n {
        let position: Vec2 = vec2(random_range(-400.0, 400.0), random_range(-400.0, 400.0));
        // let velocity: Vec2 = vec2(random_f32() * 100.0 - 50.0, random_f32() * 100.0 - 50.0);
        let velocity: Vec2 = vec2(0.0, 0.0);
        let acceleration: Vec2 = vec2(0.0, 0.0);
        let mass: f32 = random_range(1.0, 5.0);
        bodies.push(Body::new(position, velocity, acceleration, mass));
    }
    Model { bodies }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let delta_t: f32 = 1.0 / 60.0;

    // apply electrical between each pair of bodies
    for (i, j) in (0..model.bodies.len()).tuple_combinations() {
        let (left, right) = model.bodies.split_at_mut(j);
        let body1: &mut Body = &mut left[i];
        let body2: &mut Body = &mut right[0];
        let displacement: Vec2 = body2.position - body1.position;
        // kludge to avoid division by zero
        if displacement.length() <= 1.0 {
            continue;
        }
        let force: Vec2 = ELECTRIC_CONSTANT * (body1.charge * body2.charge) / displacement.length().powi(2)
            * -displacement.normalize();

        body1.apply_force(force);
        body2.apply_force(-force);
    }

    for body in model.bodies.iter_mut() {
        body.acceleration = body.net_force / body.mass;
        body.velocity += body.acceleration * delta_t / 2.0;
        body.position += body.velocity * delta_t;
        body.reset_force();
    }

    // apply force between each pair of bodies
    for (i, j) in (0..model.bodies.len()).tuple_combinations() {
        let (left, right) = model.bodies.split_at_mut(j);
        let body1: &mut Body = &mut left[i];
        let body2: &mut Body = &mut right[0];
        let displacement: Vec2 = body2.position - body1.position;
        // kludge to avoid division by zero
        if displacement.length() <= body1.radius() || displacement.length() <= body2.radius() {
            if body1.mass > body2.mass {
                body1.mass += body2.mass;
                body2.alive = false;
            } else {
                body2.mass += body1.mass;
                body1.alive = false;
            }
            body1.charge += body2.charge;
            body2.charge += body1.charge;
            continue;
        }
        let force: Vec2 =
            ELECTRIC_CONSTANT * (body1.charge * body2.charge) / displacement.length().pow(2) * -displacement.normalize();

        body1.apply_force(force);
        body2.apply_force(-force);
    }

    for body in model.bodies.iter_mut() {
        body.acceleration = body.net_force / body.mass;
        body.velocity += body.acceleration * delta_t / 2.0;
        body.reset_force();
    }

    model.bodies = model
        .bodies
        .iter()
        .filter(|b: &&Body| b.alive)
        .cloned()
        .collect();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if frame.nth() == 0 {
      draw.background().color(BLACK);
    }

    for body in model.bodies.iter() {
        body.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

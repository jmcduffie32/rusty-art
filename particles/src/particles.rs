use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Particle {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    life_span: f32,
}

impl Particle {
    fn new(l: Point2) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() - 1.0);
        let position = l;
        let life_span = 255.0;
        Particle {
            position,
            velocity,
            acceleration,
            life_span,
        }
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.life_span -= 2.0;
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(12.0, 12.0)
            .rgba(
                random_range(0.0, 1.0),
                random_range(0.0, 1.0),
                random_range(0.0, 1.0),
                self.life_span / 255.0,
            )
            .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            .stroke_weight(2.0);
    }

    fn is_dead(&self) -> bool {
        self.life_span < 0.0
    }
}

struct Model {
    particles: Vec<Particle>,
}

fn model(app: &App) -> Model {
    app.new_window().size(640, 360).view(view).build().unwrap();
    let particles: Vec<Particle> = Vec::new();

    Model { particles }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model
        .particles
        .push(Particle::new(app.mouse.position()));

    for i in (0..model.particles.len()).rev() {
        model.particles[i].update();
        if model.particles[i].is_dead() {
            model.particles.remove(i);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for p in model.particles.iter() {
        p.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

use nannou::prelude::*;
use uuid::Uuid;

const SEEK_STRENGTH: f32 = 1.0;
const SEPARATION_STRENGTH: f32 = 1.0;
const ALIGN_STRENGTH: f32 = 0.5;
const COHERE_STRENGTH: f32 = 0.5;

const FLASH_THRESHOLD: i32 = 400;
const MAX_FLASH_TIMER: i32 = 450;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(PartialEq, Clone)]
struct Vehicle {
    id: String,
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    max_speed: f32,
    max_acceleration: f32,
    flash_timer: i32,
}

impl Vehicle {
    fn new() -> Self {
        let position: Vec2 = vec2(random_range(-400.0, 400.0), random_range(-400.0, 400.0));
        let velocity: Vec2 = vec2(0.0, 0.0);
        let acceleration: Vec2 = vec2(0.0, 0.0);

        let max_speed = 1.0;
        let max_acceleration = 0.005;
        let id = Uuid::new_v4().to_string();
        let flash_timer = random_range(0, MAX_FLASH_TIMER);

        Vehicle {
            id,
            position,
            velocity,
            acceleration,
            max_speed,
            max_acceleration,
            flash_timer,
        }
    }

    fn separate(&self, vehicles: &Vec<Vehicle>) -> Vec2 {
        let desired_separation = 25.0;
        let mut steer = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance(vehicle.position);
            if d > 0.0 && d < desired_separation {
                let diff = self.position - vehicle.position;
                steer += diff;
            }
        }

        steer.normalize_or_zero()
    }

    fn seek(&self, target: Point2) -> Vec2 {
        (target - self.position).normalize_or_zero()
    }

    fn align(&self, vehicles: &Vec<Vehicle>) -> Vec2 {
        let neighbor_distance = 50.0;
        let mut sum = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance(vehicle.position);
            if d < neighbor_distance {
                sum += vehicle.velocity;
            }
        }
        return sum.normalize_or_zero();
    }

    fn cohere(&self, vehicles: &Vec<Vehicle>) -> Vec2 {
        let neighbor_distance = 50.0;
        let mut sum = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance(vehicle.position);
            if d < neighbor_distance {
                sum += self.seek(vehicle.position);
            }
        }
        return sum.normalize_or_zero();
    }

    fn handle_flash(&mut self, vehicles: &Vec<Vehicle>) {
        self.flash_timer -= 1;
        if self.flash_timer < 0 {
          for vehicle in vehicles.iter() {
              if vehicle.id == self.id {
                  continue;
              }
              let d = self.position.distance(vehicle.position);
              if d < 20.0 && vehicle.flash_timer > FLASH_THRESHOLD {
                  self.flash_timer = MAX_FLASH_TIMER;
                  break;
              }
          }
        }
        if self.flash_timer <= -MAX_FLASH_TIMER / 2 {
            self.flash_timer = MAX_FLASH_TIMER;
        }
    }

    fn update(&mut self, target: Point2, vehicles: &Vec<Vehicle>) {
        let seek_acceleration: Vec2 = self.seek(target) * SEEK_STRENGTH;
        let separation_acceleration: Vec2 = self.separate(vehicles) * SEPARATION_STRENGTH;
        let align_acceleration: Vec2 = self.align(vehicles) * ALIGN_STRENGTH;
        let cohere_acceleration: Vec2 = self.cohere(vehicles) * COHERE_STRENGTH;
        let total_accel: Vec2 =
            seek_acceleration + separation_acceleration + align_acceleration + cohere_acceleration;
        self.acceleration = total_accel.normalize() * self.max_acceleration;

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position += self.velocity;
        self.acceleration = vec2(0.0, 0.0);

        self.handle_flash(vehicles);

    }

    fn display(&self, draw: &Draw) {
        let color: rgb::Rgb<nannou::color::encoding::Srgb, u8>;
        if self.flash_timer > 5 {
            color = RED;
        } else {
            color = BLACK;
        } 
        draw.tri()
            .points(
                pt2(10.0, 0.0),
                pt2(10.0, 0.0).rotate(2.5 * PI / 3.0),
                pt2(10.0, 0.0).rotate(-2.5 * PI / 3.0),
            )
            .x_y(self.position.x, self.position.y)
            .rotate(self.velocity.angle())
            .color(color);
    }
}

struct Model {
    vehicles: Vec<Vehicle>,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let mut vehicles: Vec<Vehicle> = vec![];
    let n = 100;
    for _i in 0..n {
        vehicles.push(Vehicle::new());
    }
    Model { vehicles }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let target = app.mouse.position();
    let vehicles = model.vehicles.clone();
    for vehicle in model.vehicles.iter_mut() {
        vehicle.update(target, &vehicles);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    for vehicle in model.vehicles.iter() {
        vehicle.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

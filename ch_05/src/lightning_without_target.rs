use nannou::prelude::*;
use uuid::Uuid;

const VEHICLE_COUNT: usize = 150;

const SEPARATION_STRENGTH: f32 = 0.9;
const ALIGN_STRENGTH: f32 = 0.7;
const COHERE_STRENGTH: f32 = 0.3;
const SEEK_STRENGTH: f32 = 1.0;

const FLASH_THRESHOLD: i32 = 200;
const MAX_FLASH_TIMER: i32 = 400;

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
        let desired_separation = 25.0 * 25.0;
        let mut steer = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance_squared(vehicle.position);
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
        let neighbor_distance = 50.0 * 50.0;
        let mut sum = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance_squared(vehicle.position);
            if d < neighbor_distance {
                sum += vehicle.velocity;
            }
        }
        return sum.normalize_or_zero();
    }

    fn cohere(&self, vehicles: &Vec<Vehicle>) -> Vec2 {
        let neighbor_distance = 50.0 * 50.0;
        let mut sum = vec2(0.0, 0.0);
        for vehicle in vehicles.iter() {
            if vehicle.id == self.id {
                continue;
            }
            let d = self.position.distance_squared(vehicle.position);
            if d < neighbor_distance {
                sum += self.seek(vehicle.position);
            }
        }
        return sum.normalize_or_zero();
    }

    fn wrap_position(&mut self) {
        let w = 400.0;
        let h = 400.0;
        if self.position.x > w {
            self.position.x = -w;
        } else if self.position.x < -w {
            self.position.x = w;
        }
        if self.position.y > h {
            self.position.y = -h;
        } else if self.position.y < -h {
            self.position.y = h;
        }
    }

    fn handle_flash(&mut self, vehicles: &Vec<Vehicle>) {
        self.flash_timer -= 1;
        let mut flashing_count = 0;
        if self.flash_timer < 0 {
            for vehicle in vehicles.iter() {
                if vehicle.id == self.id {
                    continue;
                }
                let d = self.position.distance_squared(vehicle.position);
                if d < pow(35.0, 2) && vehicle.flash_timer > FLASH_THRESHOLD {
                    self.flash_timer = random_range(FLASH_THRESHOLD, MAX_FLASH_TIMER);
                    flashing_count += 1;
                    if flashing_count > 0 {
                        break;
                    }
                }
            }
        }
        if self.flash_timer < 0 {
            let n = random_range(0, 100000);
            if n < 5 {
                self.flash_timer = random_range(FLASH_THRESHOLD, MAX_FLASH_TIMER);
            }
        }
    }

    fn update(&mut self, vehicles: &Vec<Vehicle>, target: Point2) {
        let separation_acceleration: Vec2 = self.separate(vehicles) * SEPARATION_STRENGTH;
        let align_acceleration: Vec2 = self.align(vehicles) * ALIGN_STRENGTH;
        let cohere_acceleration: Vec2 = self.cohere(vehicles) * COHERE_STRENGTH;
        let seek_acceleration: Vec2 = self.seek(target) * SEEK_STRENGTH;
        let total_accel: Vec2 = seek_acceleration + separation_acceleration + align_acceleration + cohere_acceleration;
        self.acceleration = total_accel.clamp_length_max(self.max_acceleration);

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length_max(self.max_speed);
        self.position += self.velocity;
        self.wrap_position();
        self.acceleration = vec2(0.0, 0.0);

        self.handle_flash(vehicles);
    }

    fn display(&self, draw: &Draw) {
        let mut intensity = 0;
        if self.flash_timer > FLASH_THRESHOLD {
            intensity = map_range(self.flash_timer, FLASH_THRESHOLD, MAX_FLASH_TIMER, 0, 255);
        }
        // make the gray base a bit more blue based on the intensity
        let color = srgb(
            GRAY.red,
            GRAY.green,
            (GRAY.blue as u32 + intensity as u32).clamp(0, 255) as u8,
        );

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
    target: Point2,
}

fn model(app: &App) -> Model {
    let w: u32 = 800;
    let h: u32 = 800;

    let _window = app.new_window().size(w, h).view(view).build().unwrap();
    let mut vehicles: Vec<Vehicle> = vec![];
    for _i in 0..VEHICLE_COUNT {
        vehicles.push(Vehicle::new());
    }
    let target = vec2(0.0, 0.0);
    Model { vehicles, target }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let vehicles = model.vehicles.clone();
    if random_range(0, 1000) < 5 {
        model.target = vec2(random_range(-300.0, 300.0), random_range(-300.0, 300.0));
    }
    let target = model.target;
    for vehicle in model.vehicles.iter_mut() {
        vehicle.update(&vehicles, target);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(DARKSLATEGRAY);

    for vehicle in model.vehicles.iter() {
        vehicle.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

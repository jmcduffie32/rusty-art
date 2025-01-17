use nannou::prelude::*;
use rapier2d::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
}

const BALL_RADIUS: f32 = 2.0;
const BALL_COEFFICIENT_OF_RESTITUTION: f32 = 1.0;
const LEFT_BLADE_COEFFICIENT_OF_RESTITUTION: f32 = 1.0;
const RIGHT_BLADE_COEFFICIENT_OF_RESTITUTION: f32 = 1.0;
const INITIAL_ALTITUDE: f32 = 30.0;
const LEFT_BLADE_ROTATION: f32 = -PI / 4.0;
const LEFT_BLADE_LENGTH: f32 = 40.0;
const LEFT_BLADE_THICKNESS: f32 = 1.0;
const RIGHT_BLADE_ROTATION: f32 = PI / 4.0;
const RIGHT_BLADE_LENGTH: f32 = 40.0;
const RIGHT_BLADE_THICKNESS: f32 = 1.0;
const SCALE_FACTOR: f32 = 10.0;

fn model(app: &App) -> Model {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let left_wall = ColliderBuilder::cuboid(1.0, 39.0)
        .translation(vector![-40.0, 0.0])
        .restitution(LEFT_BLADE_COEFFICIENT_OF_RESTITUTION)
        .friction(0.0)
        .build();
    collider_set.insert(left_wall);

    let right_wall = ColliderBuilder::cuboid(1.0, 39.0)
        .translation(vector![40.0, 0.0])
        .restitution(LEFT_BLADE_COEFFICIENT_OF_RESTITUTION)
        .friction(0.0)
        .build();
    collider_set.insert(right_wall);

    let ceiling = ColliderBuilder::cuboid(39.0, 1.0)
        .translation(vector![0.0, 40.0])
        .restitution(LEFT_BLADE_COEFFICIENT_OF_RESTITUTION)
        .friction(0.0)
        .build();
    collider_set.insert(ceiling);

    let floor = ColliderBuilder::cuboid(39.0, 1.0)
        .translation(vector![0.0, -40.0])
        .restitution(LEFT_BLADE_COEFFICIENT_OF_RESTITUTION)
        .friction(0.0)
        .build();
    collider_set.insert(floor);

    let fan = RigidBodyBuilder::dynamic()
        .lock_translations()
        .angvel(PI / 2.0)
        .angular_damping(0.0)
        .linear_damping(0.0)
        .build();
    let left_blade_collider =
        ColliderBuilder::cuboid(LEFT_BLADE_LENGTH / 2.0, LEFT_BLADE_THICKNESS / 2.0)
            .rotation(LEFT_BLADE_ROTATION)
            .restitution(LEFT_BLADE_COEFFICIENT_OF_RESTITUTION)
            .friction(0.0)
            .build();

    let right_blade_collider =
        ColliderBuilder::cuboid(RIGHT_BLADE_LENGTH / 2.0, RIGHT_BLADE_THICKNESS / 2.0)
            .rotation(RIGHT_BLADE_ROTATION)
            .restitution(RIGHT_BLADE_COEFFICIENT_OF_RESTITUTION)
            .friction(0.0)
            .build();

    let fan_handle = rigid_body_set.insert(fan);
    collider_set.insert_with_parent(left_blade_collider, fan_handle, &mut rigid_body_set);
    collider_set.insert_with_parent(right_blade_collider, fan_handle, &mut rigid_body_set);

    /* Create the bouncing ball. */
    for _ in 0..10 {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![random_range(-1.0, 1.0), INITIAL_ALTITUDE])
            .build();
        let collider = ColliderBuilder::ball(BALL_RADIUS)
            .restitution(BALL_COEFFICIENT_OF_RESTITUTION)
            .friction(0.0)
            .build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
    }

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let physics_pipeline = PhysicsPipeline::new();
    let island_manager = IslandManager::new();
    let broad_phase = DefaultBroadPhase::new();
    let narrow_phase = NarrowPhase::new();
    let impulse_joint_set = ImpulseJointSet::new();
    let multibody_joint_set = MultibodyJointSet::new();
    let ccd_solver = CCDSolver::new();
    let query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    Model {
        gravity,
        integration_parameters,
        rigid_body_set,
        collider_set,
        physics_pipeline,
        island_manager,
        broad_phase,
        narrow_phase,
        impulse_joint_set,
        multibody_joint_set,
        ccd_solver,
        query_pipeline,
        physics_hooks,
        event_handler,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.physics_pipeline.step(
        &model.gravity,
        &model.integration_parameters,
        &mut model.island_manager,
        &mut model.broad_phase,
        &mut model.narrow_phase,
        &mut model.rigid_body_set,
        &mut model.collider_set,
        &mut model.impulse_joint_set,
        &mut model.multibody_joint_set,
        &mut model.ccd_solver,
        Some(&mut model.query_pipeline),
        &model.physics_hooks,
        &model.event_handler,
    );
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    draw.background().color(WHITE);

    for (_, collider) in model.collider_set.iter() {
        let position = collider.position().translation.vector;
        let shape = collider.shape();
        // change color if there was a collision
        match shape.shape_type() {
            ShapeType::Ball => {
                let ball = shape.as_ball().unwrap();
                draw.ellipse()
                    .x_y(position.x * SCALE_FACTOR, position.y * SCALE_FACTOR)
                    .radius(ball.radius * SCALE_FACTOR)
                    .rgba(0.5, 0.5, 0.5, 1.0)
                    .stroke(BLACK);
            }
            ShapeType::Cuboid => {
                let cuboid = shape.as_cuboid().unwrap();
                draw.rect()
                    .x_y(position.x * SCALE_FACTOR, position.y * SCALE_FACTOR)
                    .w_h(
                        cuboid.half_extents.x * 2.0 * SCALE_FACTOR,
                        cuboid.half_extents.y * 2.0 * SCALE_FACTOR,
                    )
                    .rotate(collider.position().rotation.angle())
                    .color(BLACK);
            }
            _ => {}
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

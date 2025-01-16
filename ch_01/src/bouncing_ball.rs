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
    ball_body_handle: RigidBodyHandle,
}

const BALL_RADIUS: f32 = 20.0;
const BALL_COEFFICIENT_OF_RESTITUTION: f32 = 1.0;
const LEFT_FLOOR_COEFFICIENT_OF_RESTITUTION: f32 = 0.9;
const RIGHT_FLOOR_COEFFICIENT_OF_RESTITUTION: f32 = 0.5;
const INITIAL_ALTITUDE: f32 = 300.0;
const LEFT_WALL_ROTATION: f32 = -PI / 4.0;
const LEFT_WALL_LENGTH: f32 = 600.0;
const RIGHT_WALL_ROTATION: f32 = PI / 4.0;
const RIGHT_WALL_LENGTH: f32 = 600.0;

fn model(app: &App) -> Model {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let left_wall = ColliderBuilder::cuboid(LEFT_WALL_LENGTH / 2.0, 1.0)
        .rotation(LEFT_WALL_ROTATION)
        .restitution(LEFT_FLOOR_COEFFICIENT_OF_RESTITUTION)
        .build();
    collider_set.insert(left_wall);

    let right_wall = ColliderBuilder::cuboid(RIGHT_WALL_LENGTH / 2.0, 1.0)
        .rotation(RIGHT_WALL_ROTATION)
        .restitution(RIGHT_FLOOR_COEFFICIENT_OF_RESTITUTION)
        .build();
    collider_set.insert(right_wall);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, INITIAL_ALTITUDE])
        .build();
    let collider = ColliderBuilder::ball(BALL_RADIUS)
        .restitution(BALL_COEFFICIENT_OF_RESTITUTION)
        .build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

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
        ball_body_handle,
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

    let ball_body = &model.rigid_body_set[model.ball_body_handle];
    println!("Ball altitude: {}", ball_body.translation().y);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();

    draw.background().color(WHITE);

    let position = model.rigid_body_set[model.ball_body_handle].translation();

    draw.ellipse()
        .x_y(position.x, position.y)
        .radius(BALL_RADIUS)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK);

    draw.rect()
        .w_h(LEFT_WALL_LENGTH, 2.0)
        .rotate(LEFT_WALL_ROTATION)
        .color(BLACK);
    draw.rect()
        .w_h(RIGHT_WALL_LENGTH, 2.0)
        .rotate(RIGHT_WALL_ROTATION)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

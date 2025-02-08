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
const SCALE_FACTOR: f32 = 2.0;

fn model(app: &App) -> Model {
    // simulate several pendula
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let mut impulse_joint_set = ImpulseJointSet::new();

    for i in 0..10 {
        let pivot = RigidBodyBuilder::fixed()
            .translation(vector![4.1 * i as f32, 100.0])
            .build();
        let pivot_handle = rigid_body_set.insert(pivot);

        let mut ball = RigidBodyBuilder::dynamic()
            .translation(vector![4.1 * i as f32, 0.0])
            .build();
        if i <= 1 {
            ball.set_linvel(vector![-10.0, 0.0], true);
        }
        let ball_handle = rigid_body_set.insert(ball);
        let ball_collider = ColliderBuilder::ball(BALL_RADIUS).restitution(1.0).build();
        collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

        // connect ball to pivot
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![0.0, 0.0])
            .local_anchor2(point![0.0, 100.0])
            .build();
        impulse_joint_set.insert(pivot_handle, ball_handle, joint, true);
    }

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let physics_pipeline = PhysicsPipeline::new();
    let island_manager = IslandManager::new();
    let broad_phase = DefaultBroadPhase::new();
    let narrow_phase = NarrowPhase::new();
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

    // draw each joint as a line
    for (_, joint) in model.impulse_joint_set.iter() {
        let body1 = model.rigid_body_set.get(joint.body1).unwrap();
        let body2 = model.rigid_body_set.get(joint.body2).unwrap();
        draw.line()
            .start(pt2(
                body1.position().translation.x * SCALE_FACTOR,
                body1.position().translation.y * SCALE_FACTOR,
            ))
            .end(pt2(
                body2.position().translation.x * SCALE_FACTOR,
                body2.position().translation.y * SCALE_FACTOR,
            ))
            .color(BLACK);

        let body_position = body2.position().translation;
        draw.ellipse()
            .x_y(
                body_position.x * SCALE_FACTOR,
                body_position.y * SCALE_FACTOR,
            )
            .radius(2.0 * SCALE_FACTOR)
            .rgba(0.5, 0.5, 0.5, 1.0)
            .stroke(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}

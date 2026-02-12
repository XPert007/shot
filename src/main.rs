use macroquad::prelude::*;
use rapier2d::prelude::*;

const M_TO_PX: f32 = 50.0;

#[macroquad::main("Rapier + Macroquad")]
async fn main() {
    let gravity = vector![0.0, 9.81];
    let integration_parameters = IntegrationParameters::default();

    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let ground_body = RigidBodyBuilder::fixed()
        .translation(vector![
            screen_width() / 2.0 / M_TO_PX,
            (screen_height() - 40.0) / M_TO_PX,
        ])
        .build();

    let ground_handle = rigid_body_set.insert(ground_body);

    let ground_collider = ColliderBuilder::cuboid(screen_width() / M_TO_PX, 10.0 / M_TO_PX).build();

    collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

    let radius_m = 0.4;

    let ball_body = RigidBodyBuilder::dynamic()
        .translation(vector![screen_width() / 2.0 / M_TO_PX, 1.0])
        .build();

    let ball_handle = rigid_body_set.insert(ball_body);

    let ball_collider = ColliderBuilder::ball(radius_m).restitution(0.7).build();
    let ball_body2 =
        RigidBodyBuilder::dynamic().translation(vector![screen_width() / 2.0 / M_TO_PX, 1.0]);
    let ball_handle2 = rigid_body_set.insert(ball_body2);
    let ball_collider2 = ColliderBuilder::ball(radius_m).restitution(0.7).build();
    collider_set.insert_with_parent(ball_collider2, ball_handle2, &mut rigid_body_set);
    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

    loop {
        clear_background(RED);

        {
            let ball = rigid_body_set.get_mut(ball_handle).unwrap();

            if is_key_down(KeyCode::D) {
                ball.set_linvel(vector![5.0, ball.linvel().y], true);
            }
            if is_key_down(KeyCode::A) {
                ball.set_linvel(vector![-5.0, ball.linvel().y], true);
            }
            if is_key_pressed(KeyCode::W) {
                ball.apply_impulse(vector![0.0, -6.0], true);
            }
        }

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        let ball = &rigid_body_set[ball_handle];
        let pos = ball.translation();
        let ball2 = &rigid_body_set[ball_handle2];
        let pos2 = ball2.translation();
        draw_circle(pos.x * M_TO_PX, pos.y * M_TO_PX, radius_m * M_TO_PX, YELLOW);
        draw_circle(
            pos2.x * M_TO_PX,
            pos2.y * M_TO_PX,
            radius_m * M_TO_PX,
            BLACK,
        );
        draw_rectangle(0.0, screen_height() - 40.0, screen_width(), 20.0, BLACK);

        next_frame().await;
    }
}

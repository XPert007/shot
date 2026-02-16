use macroquad::prelude::*;
use rapier2d::prelude::*;

const M_TO_PX: f32 = 50.0;
const G: f32 = 20.0; // tune this

#[macroquad::main("Mutual Gravity (fixed)")]
async fn main() {
    let gravity = vector![0.0, 0.0];
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
        .translation(vector![screen_width() / 2.0 / M_TO_PX - 2.0, 4.0])
        .build();
    let ball_handle = rigid_body_set.insert(ball_body);
    let ball_collider = ColliderBuilder::ball(radius_m)
        .density(10.0) // give it mass via density
        .restitution(0.7)
        .build();
    collider_set.insert_with_parent(ball_collider, ball_handle, &mut rigid_body_set);

    let ball_body2 = RigidBodyBuilder::dynamic()
        .translation(vector![screen_width() / 2.0 / M_TO_PX + 2.0, 4.0])
        .build();
    let ball_handle2 = rigid_body_set.insert(ball_body2);
    let ball_collider2 = ColliderBuilder::ball(radius_m)
        .density(10.0)
        .restitution(0.7)
        .build();
    collider_set.insert_with_parent(ball_collider2, ball_handle2, &mut rigid_body_set);

    rigid_body_set
        .get_mut(ball_handle)
        .unwrap()
        .set_linvel(vector![0.0, 2.0], true);
    rigid_body_set
        .get_mut(ball_handle2)
        .unwrap()
        .set_linvel(vector![0.0, -2.0], true);

    loop {
        clear_background(WHITE);

        {
            let ball = rigid_body_set.get_mut(ball_handle).unwrap();
            if is_key_down(KeyCode::D) {
                ball.set_linvel(vector![5.0, ball.linvel().y], true);
            }
            if is_key_down(KeyCode::A) {
                ball.set_linvel(vector![-5.0, ball.linvel().y], true);
            }
            if is_key_pressed(KeyCode::W) {
                ball.apply_impulse(vector![0.0, -5.0], true);
            }
        }

        let pos1 = rigid_body_set[ball_handle].translation();
        let pos2 = rigid_body_set[ball_handle2].translation();

        let dir = pos2 - pos1; // vector from 1 -> 2
        let dist_sq = dir.norm_squared(); // use norm_squared() (not length_squared)
        let eps = 1e-6_f32;

        if dist_sq > eps {
            let dist = dist_sq.sqrt();
            let dir_unit = dir / dist;

            let mass1 = rigid_body_set[ball_handle].mass() + 20.00;
            let mass2 = rigid_body_set[ball_handle2].mass();

            let force_mag = G * mass1 * mass2 / dist_sq;

            let force_vec = dir_unit * force_mag;

            let dt = get_frame_time(); // seconds this frame
            let impulse = force_vec * dt;

            rigid_body_set
                .get_mut(ball_handle)
                .unwrap()
                .apply_impulse(impulse, true);

            rigid_body_set
                .get_mut(ball_handle2)
                .unwrap()
                .apply_impulse(-impulse, true);
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

        draw_circle(pos.x * M_TO_PX, pos.y * M_TO_PX, radius_m * M_TO_PX, RED);
        draw_circle(pos2.x * M_TO_PX, pos2.y * M_TO_PX, radius_m * M_TO_PX, BLUE);
        draw_rectangle(0.0, screen_height() - 40.0, screen_width(), 20.0, BLACK);
        draw_text("Mutual Gravity (fixed)", 20.0, 30.0, 30.0, BLACK);

        next_frame().await;
    }
}

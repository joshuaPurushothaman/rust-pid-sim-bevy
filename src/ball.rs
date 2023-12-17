use bevy::math::Vec2;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::pid::PIDController;
use crate::MouseWorldCoords;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_ball)
            .add_systems(Update, update_ball)
            .add_systems(Update, update_pid_controllers);
    }
}

fn make_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    debug!("making ball :3");

    commands
        .spawn((
            Ball,
            Velocity(Vec2::ZERO),
            Force(Vec2::ZERO),
            Mass(0.1),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(30.).into()).into(),
                // material: materials.add(ColorMaterial::from(Color::rgb(101., 207., 129.))),
                material: materials.add(ColorMaterial::from(Color::AQUAMARINE)),
                transform: Transform::IDENTITY,
                ..default()
            },
        ))
        .with_children(|parent| {
            let x_controller = PIDController::new(
                10.0,
                0.0,
                0.60,
                -f64::INFINITY,
                f64::INFINITY,
                f64::INFINITY,
            );
            let y_controller = x_controller.clone();

            parent.spawn((x_controller, ControllerType::XController));
            parent.spawn((y_controller, ControllerType::YController));
        });
}

fn update_ball(
    mut query: Query<(&mut Transform, &mut Velocity, &Force, &Mass), With<Ball>>,
    time: Res<Time>,
) {
    for (mut tf, mut vel, force, mass) in &mut query {
        let dt = time.delta_seconds();

        vel.0 += (force.0 / mass.0) * dt;

        tf.translation.x += vel.0.x * dt;
        tf.translation.y += vel.0.y * dt;
    }
}

fn update_pid_controllers(
    mut ball_query: Query<(&Transform, &mut Velocity, &mut Force, &Children), With<Ball>>,
    mut pid_query: Query<(&mut PIDController, &ControllerType)>,
    time: Res<Time>,
    mouse_world_coords: Res<MouseWorldCoords>,
) {
    for (tf, vel, mut force, children) in &mut ball_query {
        for &child in children {
            if let Ok((mut pid, controller_type)) = pid_query.get_mut(child) {
                match controller_type {
                    ControllerType::XController => {
                        let mx = mouse_world_coords.0.x as f64;
                        let tx = tf.translation.x as f64;

                        force.0.x = pid.calculate(time.delta(), mx, tx) as f32;
                    }
                    ControllerType::YController => {
                        let my = mouse_world_coords.0.y as f64;
                        let ty = tf.translation.y as f64;

                        force.0.y = pid.calculate(time.delta(), my, ty) as f32;
                    }
                }

                debug!(
                    "force x {:08.2} y {:08.2} len {:07.1}",
                    force.0.x,
                    force.0.y,
                    force.0.length()
                );
            }
        }

        // Max force
        let max_force = 25000.0; //  TODO: parametric? via... CLI args, GUI sliders, TOML config... hmm.
        if force.0.length() > max_force {
            // FIXME: doesn't work??? lol
            force.0 = force.0.clamp_length(0., max_force);
        }

        // Some resistance?
        force.0 -= vel.0 * 0.23;

        // Static friction
        if force.0.length() < 50.0 {
            force.0 = Vec2::ZERO;
        }
    }
}

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Force(Vec2);

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
enum ControllerType {
    XController,
    YController,
}

#[derive(Component)]
struct Ball;

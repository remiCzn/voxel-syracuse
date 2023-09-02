use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(Startup, (setup_camera, setup_grab_cursor))
            .add_systems(Update, (player_move, player_look, cursor_grab));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    },));
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn setup_grab_cursor(mut prim_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = prim_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Unable to access primary window for `setup_grab_cursor`")
    }
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    prim_window: Query<&mut Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    if let Ok(window) = prim_window.get_single() {
        for mut transform in cameras.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => {}
                    _ => match key {
                        KeyCode::Z => {
                            velocity += forward;
                        }
                        KeyCode::S => {
                            velocity -= forward;
                        }
                        KeyCode::Q => {
                            velocity -= right;
                        }
                        KeyCode::D => {
                            velocity += right;
                        }
                        KeyCode::Space => {
                            velocity += Vec3::Y;
                        }
                        KeyCode::ShiftLeft => {
                            velocity -= Vec3::Y;
                        }
                        _ => {}
                    },
                }
            }

            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * 12.0;
        }
    } else {
        warn!("Unable to access primary window for `player_move`")
    }
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    motion: Res<Events<MouseMotion>>,
    mut state: ResMut<InputState>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let sensitivity = 0.00010;
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.iter(&motion) {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut prim_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = prim_window.get_single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Unable to access primary window for `cursor_grab`")
    }
}

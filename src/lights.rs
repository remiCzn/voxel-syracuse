use std::f32::consts::PI;

use bevy::prelude::{
    default, AmbientLight, Color, Commands, DirectionalLight, DirectionalLightBundle, Plugin, Quat,
    Startup, Transform, Vec3,
};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_lights);
    }
}

fn setup_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::YELLOW,
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 3.),
            ..default()
        },
        ..default()
    });
}

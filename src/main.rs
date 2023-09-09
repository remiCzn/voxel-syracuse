use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use chunk::Chunk;

mod block;
mod camera;
mod chunk;
mod data;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            CameraPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for x in -3..3 {
        for z in -3..3 {
            let custom_texture_handle: Handle<Image> = asset_server.load("spritesheet_tiles.png");
            let mesh = Chunk::new(Vec2 {
                x: x as f32,
                y: z as f32,
            });
            let cube_mesh_handle: Handle<Mesh> = meshes.add(mesh.build());

            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh_handle,
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle),
                        ..default()
                    }),
                    transform: Transform::from_xyz(16.0, 0.0, 0.0),
                    ..Default::default()
                },
                Name::new(format!("Chunk ({},{})", x, z)),
            ));
        }
    }

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.4,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::YELLOW,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 100.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

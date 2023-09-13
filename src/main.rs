use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use chunk::Chunk;
use data::WORLD_SIZE_IN_CHUNKS;
use lights::LightPlugin;

mod block;
mod camera;
mod chunk;
mod data;
mod lights;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            WorldInspectorPlugin::default(),
            CameraPlugin,
            LightPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
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
    for x in -WORLD_SIZE_IN_CHUNKS..WORLD_SIZE_IN_CHUNKS {
        for z in -WORLD_SIZE_IN_CHUNKS..WORLD_SIZE_IN_CHUNKS {
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
}

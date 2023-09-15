use std::ops::Mul;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use chunk::Chunk;
use data::{CHUNK_WIDTH, VIEW_DISTANCE_IN_CHUNKS, WORLD_SIZE_IN_CHUNKS};
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
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup_chunks)
        .add_systems(Update, spawn_chunks)
        .run();
}

fn setup_chunks(mut commands: Commands) {
    for x in 0..WORLD_SIZE_IN_CHUNKS {
        for z in 0..WORLD_SIZE_IN_CHUNKS {
            commands.spawn((
                Chunk::new(Vec2 {
                    x: x as f32,
                    y: z as f32,
                }),
                Name::new(format!("ChunkData ({},{})", x, z)),
            ));
        }
    }
}

fn spawn_chunks(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_q: Query<&mut Chunk>,
    cameras: Query<&Transform, With<Camera3d>>,
) {
    let player_transform = cameras
        .single()
        .translation
        .mul(1.0 / CHUNK_WIDTH as f32)
        .as_ivec3();
    let custom_texture_handle: Handle<Image> = asset_server.load("spritesheet_tiles.png");

    let range_x = player_transform.x - VIEW_DISTANCE_IN_CHUNKS / 2
        ..player_transform.x + VIEW_DISTANCE_IN_CHUNKS / 2;
    let range_z = player_transform.z - VIEW_DISTANCE_IN_CHUNKS / 2
        ..player_transform.z + VIEW_DISTANCE_IN_CHUNKS / 2;

    for mut chunk in chunk_q.iter_mut() {
        let x = chunk.coords.x as i32;
        let z = chunk.coords.y as i32;
        if range_x.contains(&x) && range_z.contains(&z) && !chunk.active {
            let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk.build());

            println!("Let's spawn {x},{z}");
            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh_handle,
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(custom_texture_handle.clone()),
                        ..default()
                    }),
                    ..Default::default()
                },
                Name::new(format!("Chunk ({},{})", x, z)),
            ));
            chunk.active = true;
        }
    }
}

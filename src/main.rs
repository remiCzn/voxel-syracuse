use std::ops::Mul;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use chunk::{Chunk, ChunkDatas};
use data::{CHUNK_WIDTH, VIEW_DISTANCE_IN_CHUNKS};
use lights::LightPlugin;

mod block;
mod camera;
mod chunk;
mod data;
mod lights;
mod noise;

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
        .insert_resource(ChunkDatas::default())
        .insert_resource(Msaa::Off)
        .add_systems(Update, spawn_chunks)
        .run();
}

fn spawn_chunks(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunks: ResMut<ChunkDatas>,
    cameras: Query<&Transform, With<Camera3d>>,
) {
    let player_transform = cameras
        .single()
        .translation
        .mul(1.0 / CHUNK_WIDTH as f32)
        .as_ivec3();
    let custom_texture_handle: Handle<Image> = asset_server.load("spritesheet_tiles.png");

    let range_x =
        player_transform.x - VIEW_DISTANCE_IN_CHUNKS..player_transform.x + VIEW_DISTANCE_IN_CHUNKS;
    let range_z =
        player_transform.z - VIEW_DISTANCE_IN_CHUNKS..player_transform.z + VIEW_DISTANCE_IN_CHUNKS;

    for (x, z) in chunks.active.clone() {
        if !range_x.contains(&x) || !range_z.contains(&z) {
            if let Some(chunk) = chunks.datas.get_mut(&(x, z)) {
                chunk.active = false;
                if let Some(ent) = chunk.entity_id {
                    commands.entity(ent).despawn_recursive();
                }
            }
        }
    }

    chunks.active.clear();

    for x in range_x {
        for z in range_z.clone() {
            chunks.active.push((x, z));
            let chunk = {
                if !chunks.datas.contains_key(&(x, z)) {
                    chunks.datas.insert(
                        (x, z),
                        Chunk::new(Vec2 {
                            x: x as f32,
                            y: z as f32,
                        }),
                    );
                }
                chunks.datas.get_mut(&(x, z)).unwrap()
            };
            if !chunk.active {
                let cube_mesh_handle: Handle<Mesh> = meshes.add(chunk.build());

                let ent = commands
                    .spawn((
                        PbrBundle {
                            mesh: cube_mesh_handle,
                            material: materials.add(StandardMaterial {
                                base_color_texture: Some(custom_texture_handle.clone()),
                                reflectance: 0.0,
                                perceptual_roughness: 1.0,
                                ..default()
                            }),
                            ..Default::default()
                        },
                        Name::new(format!("Chunk ({},{})", x, z)),
                    ))
                    .id();
                chunk.entity_id = Some(ent);
                chunk.active = true;
            }
        }
    }
}

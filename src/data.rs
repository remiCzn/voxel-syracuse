use bevy::prelude::{Vec2, Vec3};

pub const CHUNK_WIDTH: i32 = 5;
pub const CHUNK_HEIGHT: i32 = 15;
pub const TEXTURE_ATLAS_WIDTH: f32 = 9.0;
pub const TEXTURE_ATLAS_HEIGHT: f32 = 10.0;
pub const WORLD_SIZE_IN_CHUNKS: i32 = 40;
pub const VIEW_DISTANCE_IN_CHUNKS: i32 = 8;
pub const WORLD_SIZE_IN_BLOCKS: i32 = WORLD_SIZE_IN_CHUNKS * CHUNK_WIDTH;

pub const VOXEL_VERTS: [Vec3; 8] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(1.0, 0.0, 1.0),
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(0.0, 1.0, 1.0),
];

pub const VOXEL_NORMALS: [[f32; 3]; 6] = [
    [0.0, 0.0, 1.0],
    [0.0, 0.0, -1.0],
    [0.0, 1.0, 0.0],
    [0.0, -1.0, 0.0],
    [-1.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
];

pub const VOXEL_FACE_CHECKS: [Vec3; 6] = [
    Vec3::new(0.0, 0.0, -1.0),
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
    Vec3::new(-1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
];

pub const VOXEL_TRIS: [[usize; 4]; 6] = [
    // 0 1 2 2 1 3
    [0, 3, 1, 2],
    [5, 6, 4, 7],
    [3, 7, 2, 6],
    [1, 5, 0, 4],
    [4, 7, 0, 3],
    [1, 2, 5, 6],
];

pub const VOXEL_UVS: [Vec2; 4] = [
    Vec2::new(1.0 / TEXTURE_ATLAS_WIDTH, 1.0 / TEXTURE_ATLAS_HEIGHT),
    Vec2::new(1.0 / TEXTURE_ATLAS_WIDTH, 0.0),
    Vec2::new(0.0, 1.0 / TEXTURE_ATLAS_HEIGHT),
    Vec2::new(0.0, 0.0),
];

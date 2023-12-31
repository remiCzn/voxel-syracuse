use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashMap,
};

use crate::{block::BLOCK_TYPES, data::*, noise::get_perlin_value};

#[derive(Default, Debug, Resource)]
pub struct ChunkDatas {
    pub datas: HashMap<(i32, i32), Chunk>,
    pub active: Vec<(i32, i32)>,
}

#[derive(Debug)]
pub struct Chunk {
    pub active: bool,
    pub entity_id: Option<Entity>,
    pub coords: Vec2,
    vertices: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    normals: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    index: u32,
    voxel_map: [[[usize; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize],
}

impl Chunk {
    pub fn new(coords: Vec2) -> Self {
        let mut chunk = Self {
            coords,
            active: false,
            entity_id: None,
            vertices: vec![],
            uvs: vec![],
            normals: vec![],
            triangles: vec![],
            index: 0,
            voxel_map: [[[0; CHUNK_WIDTH as usize]; CHUNK_HEIGHT as usize]; CHUNK_WIDTH as usize],
        };
        chunk.populate_voxel_map();
        chunk.create_mesh_data();
        chunk
    }

    pub fn build(&mut self) -> Mesh {
        let mut cube_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        cube_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        cube_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        cube_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        cube_mesh.set_indices(Some(Indices::U32(self.triangles.clone())));
        cube_mesh
    }

    pub fn populate_voxel_map(&mut self) {
        for x in 0..CHUNK_WIDTH as usize {
            for z in 0..CHUNK_WIDTH as usize {
                let r_x = x as f64 + self.coords.x as f64 * CHUNK_WIDTH as f64;
                let r_z = z as f64 + self.coords.y as f64 * CHUNK_WIDTH as f64;
                let perlin_l1 = get_perlin_value(r_x, r_z, 10.0, 0.0, 5.0, 15.0, 0) as usize;
                let perlin_l2 = get_perlin_value(r_x, r_z, 5.0, 0.0, 2.0, 45.0, 0) as usize;
                let perlin = perlin_l1 + perlin_l2;
                for y in 0..CHUNK_HEIGHT as usize {
                    if y < 2 {
                        self.voxel_map[x][y][z] = 1;
                    } else if y == perlin - 1 {
                        self.voxel_map[x][y][z] = 3;
                    } else if y >= 2 && y < perlin - 1 {
                        self.voxel_map[x][y][z] = 2;
                    }
                }
            }
        }
    }

    pub fn create_mesh_data(&mut self) {
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                for z in 0..CHUNK_WIDTH {
                    self.add_voxel_to_chunk(Vec3::new(x as f32, y as f32, z as f32));
                }
            }
        }
    }

    fn check_voxel(&self, pos: Vec3) -> bool {
        let x = pos.x.floor() as i32;
        let y = pos.y.floor() as i32;
        let z = pos.z.floor() as i32;

        if !self.is_voxel_in_chunk(x, y, z) {
            return false;
        }

        BLOCK_TYPES[self.voxel_map[x as usize][y as usize][z as usize]].is_solid
    }

    fn is_voxel_in_chunk(&self, x: i32, y: i32, z: i32) -> bool {
        if !(0..=CHUNK_WIDTH - 1).contains(&x)
            || !(0..=CHUNK_HEIGHT - 1).contains(&y)
            || !(0..=CHUNK_WIDTH - 1).contains(&z)
        {
            return false;
        }
        true
    }

    fn add_voxel_to_chunk(&mut self, pos: Vec3) {
        let chunk_coords = Vec3 {
            x: self.coords.x * CHUNK_WIDTH as f32,
            y: 0.0,
            z: self.coords.y * CHUNK_WIDTH as f32,
        };
        for p in 0..6 {
            if self.check_voxel(pos) && !self.check_voxel(pos + VOXEL_FACE_CHECKS[p]) {
                let block_id = self.voxel_map[pos.x as usize][pos.y as usize][pos.z as usize];
                self.vertices
                    .push((chunk_coords + pos + VOXEL_VERTS[VOXEL_TRIS[p][0]]).to_array());
                self.vertices
                    .push((chunk_coords + pos + VOXEL_VERTS[VOXEL_TRIS[p][1]]).to_array());
                self.vertices
                    .push((chunk_coords + pos + VOXEL_VERTS[VOXEL_TRIS[p][2]]).to_array());
                self.vertices
                    .push((chunk_coords + pos + VOXEL_VERTS[VOXEL_TRIS[p][3]]).to_array());

                let text_id = BLOCK_TYPES[block_id].textures[p];
                if text_id > (TEXTURE_ATLAS_WIDTH * TEXTURE_ATLAS_HEIGHT) as i32 {
                    panic!("The texture id is not defined");
                }
                let text_y = text_id / TEXTURE_ATLAS_WIDTH as i32;
                let text_x = text_id - (text_y * TEXTURE_ATLAS_WIDTH as i32);
                let coords = Vec2::new(
                    (text_x as f32) / TEXTURE_ATLAS_WIDTH,
                    (text_y as f32) / TEXTURE_ATLAS_HEIGHT,
                );

                self.uvs.push((coords + VOXEL_UVS[0]).to_array());
                self.uvs.push((coords + VOXEL_UVS[1]).to_array());
                self.uvs.push((coords + VOXEL_UVS[2]).to_array());
                self.uvs.push((coords + VOXEL_UVS[3]).to_array());
                self.normals.push(VOXEL_NORMALS[p]);
                self.normals.push(VOXEL_NORMALS[p]);
                self.normals.push(VOXEL_NORMALS[p]);
                self.normals.push(VOXEL_NORMALS[p]);
                self.triangles.push(self.index);
                self.triangles.push(self.index + 1);
                self.triangles.push(self.index + 2);
                self.triangles.push(self.index + 2);
                self.triangles.push(self.index + 1);
                self.triangles.push(self.index + 3);
                self.index += 4;
            }
        }
    }
}

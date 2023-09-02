pub struct BlockType {
    pub name: &'static str,
    pub is_solid: bool,
    //Back, front, top, bottom, left, right
    pub textures: [i32; 6],
}

pub const STONE: BlockType = BlockType {
    name: "Stone",
    is_solid: true,
    textures: [39, 39, 39, 39, 39, 39],
};

pub const DIRT: BlockType = BlockType {
    name: "Dirt",
    is_solid: true,
    textures: [52, 52, 52, 52, 52, 52],
};

pub const GRASS: BlockType = BlockType {
    name: "Grass",
    is_solid: true,
    textures: [43, 43, 15, 52, 43, 43],
};

pub const BLOCK_TYPES: [BlockType; 3] = [STONE, DIRT, GRASS];

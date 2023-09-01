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

pub const BLOCK_TYPES: [BlockType; 2] = [STONE, DIRT];

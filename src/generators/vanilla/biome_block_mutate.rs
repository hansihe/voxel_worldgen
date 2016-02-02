use ::rand::{ XorShiftRng, SeedableRng, Rng };
use super::WorldGeneratorState;
use super::util::chunk_data_idx;
use super::constants::*;

const BIOME_BLOCK_MUTATE_SEED: u32 = 77236476;

const BLOCK_AIR: u8 = 0;
const BLOCK_STONE: u8 = 1;
const BLOCK_GRASS: u8 = 2;
const BLOCK_DIRT: u8 = 3;
const BLOCK_BEDROCK: u8 = 7;

pub fn mutate_chunk(world_state: &WorldGeneratorState, blocks: &mut [u8], chunk_pos: &[i32; 2]) {
    for block_x in 0..16 {
        for block_z in 0..16 {
            mutate_chunk_column(world_state, blocks, &[
                                chunk_pos[0]*16+(block_x as i32),
                                chunk_pos[1]*16+(block_z as i32)],
                                &[block_x as u32, block_z as u32]);
        }
    }
}

pub fn mutate_chunk_column(world_state: &WorldGeneratorState, blocks: &mut [u8], 
                           glob_pos: &[i32; 2], pos: &[u32; 2]) {
    let mut rng = XorShiftRng::from_seed([world_state.world_seed, glob_pos[0] as u32, 
                                     glob_pos[1] as u32, BIOME_BLOCK_MUTATE_SEED]);

    let mut topsoil_counter: i8 = -1;

    let block_x = pos[0];
    let block_z = pos[1];

    for block_y in (0..256).rev() {
        let blocks_idx = chunk_data_idx(block_x, block_y, block_z);
        if block_y <= rng.gen_range(0, 5) {
            blocks[blocks_idx] = BLOCK_BEDROCK;
        } else {
            let current_block = blocks[blocks_idx];
            if current_block == BLOCK_AIR {
                topsoil_counter = -1;
            } else if current_block == BLOCK_STONE {
                if topsoil_counter == -1 {
                    // TODO: Randomize
                    topsoil_counter = 3;
                    if block_y >= SEA_LEVEL-1 {
                        blocks[blocks_idx] = BLOCK_GRASS;
                    } else {
                        blocks[blocks_idx] = BLOCK_DIRT;
                    }
                } else if topsoil_counter > 0 {
                    topsoil_counter -= 1;
                    blocks[blocks_idx] = BLOCK_DIRT;
                }
            }
        }
    }
}

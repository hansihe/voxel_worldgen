/// NOTE:
/// Parts of this code is written by decompiling the code of the game Minecraft,
/// making notes while examining the obfuscated code, and later reimplementing
/// it in Rust. No code was directly copied or translated.
/// 
/// Only the algorithm was copied, no code.

use ::rnd::OctavesSeed;
use ::rand::Rng;

pub mod lerp;
pub mod height_field;
pub mod constants;
pub mod util;
pub mod biomes;

pub use self::lerp::lerp_height_field;
pub use self::height_field::gen_height_field;

pub struct WorldGeneratorState {
    depth_noise: OctavesSeed,
    fillin_noise: OctavesSeed,
    fillin_max_noise: OctavesSeed,
    fillin_min_noise: OctavesSeed,
}
impl WorldGeneratorState {
    pub fn new<R: Rng>(rand: &mut R) -> WorldGeneratorState {
        WorldGeneratorState {
            depth_noise: OctavesSeed::new(rand, 16),
            fillin_noise: OctavesSeed::new(rand, 8),
            fillin_max_noise: OctavesSeed::new(rand, 16),
            fillin_min_noise: OctavesSeed::new(rand, 16),
        }
    }
}

pub fn generate_chunk(state: &WorldGeneratorState, chunk_pos: &[i32; 2]) -> Vec<u8> {
    let biomes_gen = biomes::biome_map();
    let biomes = biomes_gen.gen(10, chunk_pos[0]*5, chunk_pos[1]*5, 9, 9);

    let size: [u32; 2] = [5, 5];
    let density_field = gen_height_field(
        state, &biomes[..], &[chunk_pos[0]*4, chunk_pos[1]*4], &size);
    let block_array = lerp_height_field(&density_field, &biomes, chunk_pos);
    block_array
}

use rand::{ XorShiftRng };
pub fn test_generate_chunk(chunk_pos: &[i32; 2]) -> Vec<u8> {
    let mut rng = XorShiftRng::new_unseeded(); 
    let world_gen_state = WorldGeneratorState::new(&mut rng);
    generate_chunk(&world_gen_state, chunk_pos)
}

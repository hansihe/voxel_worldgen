/// NOTE:
/// Parts of this code is written by decompiling the code of the game Minecraft,
/// making notes while examining the obfuscated code, and later reimplementing
/// it in Rust. No code was directly copied or translated.
/// 
/// Only parts of the algorithm was copied/inspired, no code.

use ::rnd::OctavesSeed;
use ::rand::{ Rng, Rand };
use ::nalgebra::{ Vec2, Pnt2 };

pub mod lerp;
pub mod height_field;
pub mod constants;
pub mod util;
pub mod biomes;
pub mod biome_block_mutate;
pub mod caves;

pub use self::lerp::lerp_height_field;
pub use self::height_field::gen_height_field;
use ::nalgebra::Vec3;
use ::gen::unit::GenUnit3;

pub struct WorldGeneratorState {
    world_seed: u32,
    depth_noise: OctavesSeed,
    fillin_noise: OctavesSeed,
    fillin_max_noise: OctavesSeed,
    fillin_min_noise: OctavesSeed,
    topsoil_depth_noise: OctavesSeed,
}
impl WorldGeneratorState {
    pub fn new<R: Rng>(rand: &mut R) -> WorldGeneratorState {
        WorldGeneratorState {
            world_seed: rand.gen(),
            depth_noise: OctavesSeed::new(rand, 16),
            fillin_noise: OctavesSeed::new(rand, 8),
            fillin_max_noise: OctavesSeed::new(rand, 16),
            fillin_min_noise: OctavesSeed::new(rand, 16),
            topsoil_depth_noise: OctavesSeed::new(rand, 2),
        }
    }
}

pub fn generate_chunk(state: &WorldGeneratorState, chunk_pos: Pnt2<i32>) -> GenUnit3<u8> {
    let biomes_gen = biomes::biome_map();
    let biomes = biomes_gen.gen(10, chunk_pos*4, Vec2::new(9, 9));

    let size: Vec2<u32> = Vec2::new(5, 5);
    let density_field = gen_height_field(state, &biomes, chunk_pos * 4, size);
    let mut block_array = lerp_height_field(&density_field, 
                                            Vec3::new(5, 33, 5), Vec3::new(4, 8, 4));
    
    biome_block_mutate::mutate_chunk(state, &mut block_array.data, chunk_pos.as_ref());
    caves::generate(&mut block_array, state.world_seed, 
                    Pnt2::new(chunk_pos[0], chunk_pos[1]));

    block_array
}

use rand::{ XorShiftRng };
pub fn test_generate_chunk(chunk_pos: &[i32; 2]) -> Vec<u8> {
    let mut rng = XorShiftRng::new_unseeded(); 
    let world_gen_state = WorldGeneratorState::new(&mut rng);
    generate_chunk(&world_gen_state, Pnt2::new(chunk_pos[0], chunk_pos[1])).data
}

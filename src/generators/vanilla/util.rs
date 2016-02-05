use ::rnd::OctavesSeed;
use ::num::Float;

pub fn denormalize_clamp<T: Float>(val: T, lower: T, upper: T) -> T {
    if val < ::num::traits::cast(0.0).unwrap() {
        lower
    } else if val > ::num::traits::cast(1.0).unwrap() {
        upper
    } else {
        lower + (upper - lower) * val
    }
}

// Idk if this is optimized by llvm, check it out.
/// This fills in a 5*5 vector with values from a mathemathical function.
/// It is used in biome sampling to give the most weight to the biome
/// closest to the center, and give less and less weight to biomes the
/// further out from the center you get.
pub fn parabolic_field() -> Vec<f64> {
    let mut values = Vec::with_capacity(25);
    for y in -2..3 {
        for x in -2..3 {
            let x_f = x as f64;
            let y_f = y as f64;
            let t: f64 = 10.0 / ((x_f*x_f + y_f*y_f) + 0.2).sqrt();
            values.push(t);
        }
    }
    values
}

pub fn height_field_idx(x: u32, y: u32, z: u32) -> usize {(x + (z*5) + (y*5*5)) as usize}
pub fn chunk_data_idx(x: u32, y: u32, z: u32) -> usize {((x & 15) | ((z & 15) << 4) | (y & 255) << 8) as usize}

/// Function used to generate an array of scaled octaved simplex noise
/// values.
pub fn gen_noise_octaves(seed: &OctavesSeed, pos: &[f64; 3], size: &[u32; 3], scale: &[f64; 3], mult: f64) -> Vec<f64> {
    let scaled = [pos[0] as f64*scale[0], pos[1] as f64*scale[1], pos[2] as f64*scale[2]];
    let mut out = Vec::with_capacity((size[0]*size[1]*size[2]) as usize);
    for x_pos in 0..size[0] {
        for z_pos in 0..size[2] {
            for y_pos in 0..size[1] {
                let val = seed.simplex3(
                    &[(scaled[0]+(x_pos as f64*scale[0]))/8192.0,
                    (scaled[1]+(y_pos as f64*scale[1]))/8192.0,
                    (scaled[2]+(z_pos as f64*scale[2]))/8192.0]) * mult;
                out.push(val);
            }
        }
    }
    out
}



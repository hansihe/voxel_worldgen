use super::WorldGeneratorState;
use super::util::{ gen_noise_octaves, parabolic_field, height_field_idx };
use super::util::{ denormalize_clamp };

use super::constants::*;

/// This is the function that takes a biome index and gives back the
/// minimum height constant in the biome. TODO
fn biome_height_min(id: u8) -> f64 { 
    match id {
        0 => 0.4,
        1 => -0.3,
        _ => panic!(),
    }
}
/// This does the same as biome_height_min, except for height variation.
fn biome_height_variation(id: u8) -> f64 { 
    match id {
        0 => 0.8,
        1 => 0.1,
        _ => panic!(),
    }
}

/// Height map scaling function.
fn scale_height_noise(input: f64) -> f64 {
    let mut val = input;
    if val < 0.0 { val = -val * 0.3 }
    val = val * 3.0 - 2.0;
    if val < 0.0 {
        val /= 2.0;
        if val < -1.0 { val = -1.0 }
        val /= 1.4;
        val /= 2.0;
    } else {
        if val > 1.0 { val = 1.0; }
        val /= 8.0;
    }
    val
}

// -> (start, variation)
/// Takes a biome array, size of the biome array, and a position.
/// The position starts 2 from the edge on both the x and y axis.
/// Samples the height and variation of the biome in the given position.
/// Returns a tuple of height start and variation.
pub fn sample_biome_range(biomes: &[u8], size: &[u32; 2], pos: &[u32; 2]) -> (f64, f64) {
    let parabolic_field = parabolic_field();
    // Get the biome we are currently in. Note that we go out 2 from the edge,
    // as we need the extra data on the edge for averaging the height map
    // earlier.
    let current_biome_idx = (pos[0] + 2) + (pos[1] + 2) * (size[0] + 4);
    let current_biome_id = biomes[current_biome_idx as usize];

    let mut biome_height_start_sum = 0f64;
    let mut biome_height_variation_sum = 0f64;
    let mut biome_height_weight_sum = 0f64;

    // We sample a 5*5 area around the current depth noise sample point and
    // add the values together in a way that makes the current sample count
    // the most, and the other samples count less and less the further you
    // get out. Vanilla uses 10/(sqrt(x^2+y^2)+0.2) as the sample graph.
    for biome_sample_z in -2i32..3 {
        for biome_sample_x in -2i32..3 {
            // Get the id of the biome we are currently sampling.
            let current_biome_sample_idx = ((pos[0] as i32 + biome_sample_x + 2) as u32)
                + ((pos[1] as i32 + biome_sample_z + 2) as u32 * (size[0] + 4));
            let current_biome_sample_id = 
                biomes[current_biome_sample_idx as usize];

            // Get and apply constant weights to the biome min height and variation.
            let biome_height_start = 
                (biome_height_min(current_biome_sample_id) 
                 * BIOME_DEPTH_WEIGHT)
                + BIOME_DEPTH_OFFSET;
            let biome_height_variation = 
                (biome_height_variation(current_biome_sample_id)
                 * BIOME_SCALE_WEIGHT)
                + BIOME_SCALE_OFFSET;

            // TODO: Amplified option?

            // How much the biome affects the total is determined by inverse euclidian
            // distance with a constant offset (the parabolic field) and the biome
            // height start.
            let mut biome_sample_weight =
                parabolic_field[((biome_sample_x + 2)*5 + (biome_sample_z + 2)) as usize]
                / (biome_height_start + 2.0);

                if biome_height_min(current_biome_sample_id) > biome_height_min(current_biome_id) {
                    biome_sample_weight /= 2.0;
                }

                biome_height_start_sum += biome_height_start * biome_sample_weight;
                biome_height_variation_sum += biome_height_variation * biome_sample_weight;
                biome_height_weight_sum += biome_sample_weight;
        }
    }

    // Apply the accumulated weight to the accumuated height start and variance.
    let biome_height_start = biome_height_start_sum / biome_height_weight_sum;
    let biome_height_variation = biome_height_variation_sum / biome_height_weight_sum;

    (biome_height_start, biome_height_variation)
}

/// Generates a height density field. This is usually used for interpolation,
/// as this only samples the noise functions at 5*5*33 intervals.
/// (Starts at the edge and end of the chunk, samples are on the edges of blocks,
/// not in the center as you might expect. The edges are overlapped by 1 between
/// chunks to make chunks smoothly blend.)
pub fn gen_height_field(state: &WorldGeneratorState, biomes: &[u8], pos: &[i32; 2], size: &[u32; 2]) -> Vec<f64> {
    if biomes.len() != ((size[0]+4)*(size[1]+4)) as usize { 
        println!("Biomes array not correct length, expected {:?}, got {:?}", 
                 (size[0]+4)*(size[1]+4), biomes.len());
        panic!() 
    };

    // Output buffer
    let mut height_field = vec![0f64; 5*5*33];

    // Sample the noise data
    let fillin_noise_pos = [pos[0] as f64, 0.0, pos[1] as f64];
    let fillin_noise_size = [5, 33, 5];
    let fillin_noise_thresh_scale = [COORDINATE_SCALE, HEIGHT_SCALE, COORDINATE_SCALE];
    let fillin_noise_value_scale = [
        COORDINATE_SCALE / MAIN_NOISE_SCALE_X,
        HEIGHT_SCALE / MAIN_NOISE_SCALE_Y,
        COORDINATE_SCALE / MAIN_NOISE_SCALE_Z];

    // The fillin values is what causes some of the more drastic local
    // terrain features. This includes features that are impossible to
    // make from a heightmap alone, like overhangs.
    let fillin_value_noise = gen_noise_octaves(
        &state.depth_noise, &fillin_noise_pos, &fillin_noise_size, 
        &fillin_noise_value_scale, 250.0);
    let fillin_max_noise = gen_noise_octaves(
        &state.fillin_max_noise, &fillin_noise_pos, &fillin_noise_size,
        &fillin_noise_thresh_scale, 35000.0);
    let fillin_min_noise = gen_noise_octaves(
        &state.fillin_min_noise, &fillin_noise_pos, &fillin_noise_size,
        &fillin_noise_thresh_scale, 35000.0);

    // The depth noise is what causes the rough height terrain features. 
    // This is later combined with the fillin values.
    let depth_noise = gen_noise_octaves(
        &state.depth_noise, &fillin_noise_pos, &[5, 1, 5],
        &[DEPTH_NOISE_SCALE_X, 1.0, DEPTH_NOISE_SCALE_Z], 1.0);

    // We increment this at the end of every loop iteration to track
    // the current position in the noise buffers.
    let mut noise_buf_2d_idx = 0;
    let mut noise_buf_3d_idx = 0;

    for biome_z in 0..size[1] {
        for biome_x in 0..size[0] {
            // Sample and apply constants to the biome height and variance.
            let (biome_height_start_base, biome_height_variation_base) =
                sample_biome_range(biomes, size, &[biome_z, biome_x]);

            // Apply constants to the height start and variation
            let biome_height_start = (biome_height_start_base * 4.0 - 1.0) / 8.0;
            let biome_height_variation = biome_height_variation_base * 0.9 + 0.1;

            // Calculate and sample depth noise
            let depth_noise_sample = scale_height_noise(depth_noise[noise_buf_2d_idx]);

            let depth_noise_abs = {
                let base = 
                    (biome_height_start + (depth_noise_sample * 0.2))
                    * (DEPTH_BASE_SIZE / 8.0);
                DEPTH_BASE_SIZE + base * 4.0
            };

            for out_buf_height in 0..33 {
                let scaled_heightmap_value = {
                    let mut val = ((((out_buf_height as f64 - depth_noise_abs) 
                                     * HEIGHT_STRETCH) * 128.0) / 256.0) 
                        / biome_height_variation;
                    if val < 0.0 { val *= 4.0 }
                    val
                };

                // The lower and upper threshold for our fillin range
                let fillin_low_threshold = fillin_min_noise[noise_buf_3d_idx] 
                    / LOWER_LIMIT_SCALE;
                let fillin_high_threshold = fillin_max_noise[noise_buf_3d_idx] 
                    / UPPER_LIMIT_SCALE;

                let fillin_value = ((fillin_value_noise[noise_buf_3d_idx] 
                                     / 10.0) + 1.0) / 2.0;
                
                // This will make a density value local to the current location
                // from both the heightmap value and the sampled fillin density
                // values. This is what makes it possible to have overhangs and
                // large cave-ish terrain features.
                let mut clamped_fillin_value = denormalize_clamp(
                    fillin_value, fillin_low_threshold, fillin_high_threshold) 
                    - scaled_heightmap_value;

                // If we are high up, we want to scale down the terrain
                // to avoid clipping the top.
                if out_buf_height > 29 {
                    let factor = (out_buf_height as f64 - 29.0) / 3.0;
                    clamped_fillin_value = clamped_fillin_value 
                        * (1.0 * factor) + (-10.0 * factor);
                }

                height_field[height_field_idx(
                    biome_z as u32, out_buf_height, biome_x as u32)] = 
                    clamped_fillin_value;

                //height_field[height_field_idx(
                //    biome_z as u32, out_buf_height, biome_x as u32)] = 
                //    ((biome_height_variation_base * 5.0) as f64)
                //    - ((out_buf_height as f64) - 20.0);

                noise_buf_3d_idx += 1;
            }
            noise_buf_2d_idx += 1;
        }
    }
    height_field
}

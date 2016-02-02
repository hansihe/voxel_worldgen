use super::util::{ chunk_data_idx, height_field_idx };
use super::constants::*;

/// Takes an array of density values in a chunk, usually by the gen_height_field
/// function. This will linearly interpolate between the values in the density
/// values in the given array, and rasterize into a rough representation of
/// the chunk. (water/lava and stone terrain only)
pub fn lerp_height_field(density_field: &[f64], biomes: &[u8], pos: &[i32; 2]) -> Vec<u8> {

    let mut out = vec![0; 65536];

    for density_field_x in 0..4 {
        for density_field_z in 0..4 {
            for density_field_y in 0..32 {

                // The input density values are only provided for every
                // 8th block in the height of the chunk.
                // We need to divide each of the deltas by 8.
                const RANGE_INV_Y: f64 = 1.0/8.0;

                // As this is 3 dimensional linear interpolation,
                // we only really care about 8 points at any given time.
                // These are the 8 vertexes in a cube.
                // The 4 values below are the values in the 4 bottom
                // vertexes in the cube.
                let mut vertex_1 = density_field[height_field_idx(
                    density_field_x, density_field_y, density_field_z)];
                let mut vertex_2 = density_field[height_field_idx(
                    density_field_x, density_field_y, density_field_z+1)];
                let mut vertex_3 = density_field[height_field_idx(
                    density_field_x+1, density_field_y, density_field_z)];
                let mut vertex_4 = density_field[height_field_idx(
                    density_field_x+1, density_field_y, density_field_z+1)];

                // These are the values if the four upper vertices in our
                // cube, subtracted by the four lower values to find the
                // difference. They are then divided by 8 to find the value
                // we need to add to the vertex values above to linearly
                // go from the values in the bottom to the values in the
                // top in 8 steps.
                let vertex_d1 = (density_field[height_field_idx(
                    density_field_x, density_field_y+1, density_field_z)]
                    - vertex_1) * RANGE_INV_Y;
                let vertex_d2 = (density_field[height_field_idx(
                    density_field_x, density_field_y+1, density_field_z+1)]
                    - vertex_2) * RANGE_INV_Y;
                let vertex_d3 = (density_field[height_field_idx(
                    density_field_x+1, density_field_y+1, density_field_z)]
                    - vertex_3) * RANGE_INV_Y;
                let vertex_d4 = (density_field[height_field_idx(
                    density_field_x+1, density_field_y+1, density_field_z+1)]
                    - vertex_4) * RANGE_INV_Y;
                
                for lerp_y in 0..8 {
                    // The interpolation here is the exact same thing as the
                    // one in the outer loop, except that the sample distance
                    // in the height field is only 4 blocks on both the x and z
                    // axes.
                    // Note that the interpolation in each inner loop is in one
                    // less dimension than in the loop outside of it. This is 
                    // done 3 times to get 3 dinensional interpolation.
                    const RANGE_INV_Z: f64 = 1.0/4.0;

                    let mut lerp_s1 = vertex_1;
                    let mut lerp_s2 = vertex_2;

                    let lerp_ds1 = (vertex_3 - vertex_1) * RANGE_INV_Z;
                    let lerp_ds2 = (vertex_4 - vertex_2) * RANGE_INV_Z;

                    for lerp_z in 0..4 {
                        // See outer loop
                        const RANGE_INV_X: f64 = 1.0/4.0;

                        let mut lerp_f = lerp_s1;
                        let lerp_df = (lerp_s2 - lerp_s1) * RANGE_INV_X;

                        for lerp_x in 0..4 {
                            let x = density_field_x * 4 + lerp_z;
                            let y = density_field_y * 8 + lerp_y;
                            let z = density_field_z * 4 + lerp_x;
                            let idx = chunk_data_idx(x, y, z);

                            // If the value of the current block is above 0,
                            // we set the block to stone. If we are below water
                            // level and the block is air, set it to water.
                            if lerp_f > 0.0 {
                                out[idx] = 1;
                                // Stone
                            } else if y < SEA_LEVEL {
                                out[idx] = 9;
                                // Water
                            }

                            lerp_f += lerp_df;
                        }
                        lerp_s1 += lerp_ds1;
                        lerp_s2 += lerp_ds2;
                    }
                    // Go one step forwards in the interpolation.
                    // This will be done 8 times for every height field
                    // sample cube.
                    vertex_1 += vertex_d1;
                    vertex_2 += vertex_d2;
                    vertex_3 += vertex_d3;
                    vertex_4 += vertex_d4;
                }

            }

        }
    }

    out
}

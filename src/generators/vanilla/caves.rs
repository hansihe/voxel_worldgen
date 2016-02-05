use ::rand::{ Rng, Rand, XorShiftRng, SeedableRng };
use std::f32;
use ::nalgebra::{ cast, norm };
use ::nalgebra::{ Pnt2, Pnt3, Vec2, Vec3 };
use ::nalgebra::{ partial_le, partial_ge };

use ::rnd::RngBuilder;

const CAVE_GEN_RAND_SALT: u64 = 87462348731267;

const RANGE_CHUNKS: u32 = 8;

const BLOCK_AIR: u8 = 0;
const BLOCK_STONE: u8 = 1;
const BLOCK_GRASS: u8 = 2;
const BLOCK_DIRT: u8 = 3;
const BLOCK_FLOWING_WATER: u8 = 8;
const BLOCK_WATER: u8 = 9;
const BLOCK_FLOWING_LAVA: u8 = 10;
const BLOCK_LAVA: u8 = 11;

fn get_chunk_idx(pos_x: i64, pos_y: i64, pos_z: i64) -> usize {
    (pos_x + (pos_z*16) + (pos_y*16*16)) as usize
}

pub fn generate(chunk_data: &mut [u8], world_seed: u32, chunk_pos: Pnt2<i32>) {
    let range = RANGE_CHUNKS as i32;

    for current_x in (chunk_pos[0]-range)..(chunk_pos[0]+range+1) {
        for current_y in (chunk_pos[1]-range)..(chunk_pos[1]+range+1) {
            let mut chunk_rng = RngBuilder::init()
                .mix(world_seed as u64)
                .mix(CAVE_GEN_RAND_SALT)
                .mix(current_x as u64)
                .mix(current_y as u64).build();

            single_generate(chunk_data, &mut chunk_rng, 
                            Pnt2::new(current_x, current_y), chunk_pos);
        }
    }
}

pub fn single_generate<R>(chunk_data: &mut [u8], rand: &mut R, current: Pnt2<i32>, 
                          target: Pnt2<i32>) where R: Rng {
    let mut source_num = {
        let t1 = rand.gen_range(0, 15);
        let t2 = rand.gen_range(0, t1 + 1);
        rand.gen_range(0, t2 + 1)
    };
    if !rand.gen_weighted_bool(7) {
        source_num = 0;
    }
    //println!("Current: {:?} {:?}", current, rand.next_u32());

    for current_source in 0..source_num {
        let pos = Pnt3::new(
            current[0]*16 + rand.gen_range(0, 16),
            {
                let t1 = rand.gen_range(0, 120);
                rand.gen_range(0, 8 + t1)
            },
            current[1]*16 + rand.gen_range(0, 16));

        let mut num_caves = 1;
        if rand.gen_weighted_bool(4) {
            carve_dead_cave(chunk_data, rand, target, cast(pos));
            num_caves += rand.gen_range(0, 4);
        }

        for current_cave in 0..num_caves {
            let angle = [
                (rand.next_f32() - 0.5) * 2.0 / 8.0,
                rand.next_f32() * f32::consts::PI * 2.0];

            let mut width_factor = rand.next_f32() * 2.0 + rand.next_f32();
            if rand.gen_weighted_bool(10) {
                width_factor *= rand.next_f32() * rand.next_f32() * 3.0 + 1.0;
            }

            carve_cave(chunk_data, rand, target, cast(pos), &angle, 0, 0, 
                       width_factor, 1.0);
        }
    }
}

pub fn carve_dead_cave<R>(chunk_data: &mut [u8], rand: &mut R, chunk_pos: Pnt2<i32>, 
                          cave_pos: Pnt3<f64>) where R: Rng {
    let width_rand = 1.0 + rand.next_f32() * 6.0;
    carve_cave(chunk_data, rand, chunk_pos, cave_pos, &[0.0, 0.0], -1, -1, 
               width_rand, 0.5)
}

pub fn carve_cave<R>(chunk_data: &mut [u8], rand: &mut R, chunk_pos: Pnt2<i32>,
                     cave_pos_start: Pnt3<f64>, cave_angle_start: &[f32; 2],
                     mut cave_len_progress: i32, mut cave_len_total: i32,
                     horizontal_stretch: f32, vertical_stretch: f32) where R: Rng {
    let chunk_center: Pnt2<i64> = cast(chunk_pos * 16 + 8);
    let chunk_center_float: Pnt2<f64> = cast(chunk_center);
    let range_blocks = (RANGE_CHUNKS*16 - 16);

    let mut pos = cave_pos_start;
    let mut ang = [cave_angle_start[0], cave_angle_start[1]];
    let mut ang_delta = [0.0_f32, 0.0];

    if cave_len_total <= 0 {
        cave_len_total = (range_blocks - rand.gen_range(0, range_blocks / 4)) as i32;
    }

    let mut unforkable_cave = false;
    if cave_len_progress == -1 {
        cave_len_progress = cave_len_total / 2;
        unforkable_cave = true;
    }

    let next_fork_distance = rand.gen_range(0, cave_len_total / 2) + cave_len_total/4;
    let cave_piece_more_vertical = rand.gen_weighted_bool(6);

    for cave_len_progress in cave_len_progress..cave_len_total {
        let horizontal_scale_factor: f32 = 1.5 + 
            ((cave_len_progress as f32 / cave_len_total as f32 * f32::consts::PI).sin()
             * horizontal_stretch);
        let vertical_scale_factor: f32 = horizontal_scale_factor * vertical_stretch;

        {
            let pitch_value_x = ang_delta[0].cos();
            let pitch_value_y = ang_delta[0].sin();
            pos[0] += (ang[1].cos() * pitch_value_x) as f64;
            pos[1] += pitch_value_y as f64;
            pos[2] += (ang[1].sin() * pitch_value_x) as f64;
        }

        ang[0] *= if cave_piece_more_vertical { 0.92 } else { 0.70 };
        ang[0] += ang_delta[0] * 0.1;
        ang[1] += ang_delta[1] * 0.1;

        ang_delta[0] *= 0.90;
        ang_delta[0] += (rand.next_f32() - rand.next_f32()) * rand.next_f32() * 2.0;
        ang_delta[1] *= 0.75;
        ang_delta[1] += (rand.next_f32() - rand.next_f32()) * rand.next_f32() * 4.0;

        let pos2d = Pnt2::new(pos[0], pos[2]);

        if !unforkable_cave && cave_len_progress == next_fork_distance && horizontal_stretch > 1.0 && cave_len_total > 0 {
            let mut rng1: XorShiftRng = rand.gen();
            carve_cave(chunk_data, &mut rng1, chunk_pos, pos, 
                       &[ang[0]/3.0, ang[1]-(f32::consts::PI/2.0)],
                       cave_len_progress, cave_len_total,
                       rand.next_f32() * 0.5 + 0.5, 1.0);
            let mut rng2: XorShiftRng = rand.gen();
            carve_cave(chunk_data, &mut rng2, chunk_pos, pos, 
                       &[ang[0]/3.0, ang[1]+(f32::consts::PI/2.0)],
                       cave_len_progress, cave_len_total,
                       rand.next_f32() * 0.5 + 0.5, 1.0);
            return;
        }

        if unforkable_cave || !rand.gen_weighted_bool(4) {
            let center_dist: Vec2<f64> = pos2d - chunk_center_float;
            let cave_len_left = cave_len_total - cave_len_progress;
            let max_carve_size = horizontal_stretch + 2.0 + 16.0;

            if center_dist[0].powi(2) + center_dist[1].powi(2) - (cave_len_left as f64).powi(2) > (max_carve_size as f64).powi(2) {
                return;
            }

            let lower_carve_bound = chunk_center_float 
                - 16.0 - (horizontal_scale_factor as f64 * 2.0);
            let upper_carve_bound = chunk_center_float 
                + 16.0 + (horizontal_scale_factor as f64 * 2.0);
            if partial_ge(&pos2d, &lower_carve_bound) && partial_le(&pos2d, &upper_carve_bound) {
                let mut carve_box_start = Pnt3::new(
                    (pos.x - horizontal_scale_factor as f64).floor() as i64 
                        - (chunk_pos[0] as i64 * 16) - 1,
                    (pos.y - vertical_scale_factor as f64).floor() as i64 - 1,
                    (pos.z - horizontal_scale_factor as f64).floor() as i64
                        - (chunk_pos[1] as i64 * 16) - 1);
                let mut carve_box_end = Pnt3::new(
                    (pos.x + horizontal_scale_factor as f64).floor() as i64 
                        - (chunk_pos[0] as i64 * 16) + 1,
                    (pos.y + vertical_scale_factor as f64).floor() as i64 + 1,
                    (pos.z + horizontal_scale_factor as f64).floor() as i64
                        - (chunk_pos[1] as i64 * 16) + 1);

                if carve_box_start[0] < 0 { carve_box_start[0] = 0 }
                if carve_box_start[1] < 1 { carve_box_start[1] = 1 }
                if carve_box_start[2] < 0 { carve_box_start[2] = 0 }
                if carve_box_end[0] > 16 { carve_box_end[0] = 16 }
                if carve_box_end[1] > 248 { carve_box_end[1] = 248 }
                if carve_box_end[2] > 16 { carve_box_end[2] = 16 }

                let hit_water = scan_water(chunk_data, carve_box_start, carve_box_end);
                if !hit_water {
                    carve_cave_step(chunk_data, chunk_pos,  pos, 
                                    carve_box_start, carve_box_end,
                                    horizontal_scale_factor, vertical_scale_factor);
                    if unforkable_cave {
                        return;
                    }
                }
            }
        }
    }
}

fn carve_cave_step(chunk_data: &mut [u8], 
                   chunk_pos: Pnt2<i32>, pos: Pnt3<f64>,
                   carve_box_start: Pnt3<i64>, carve_box_end: Pnt3<i64>, 
                   horizontal_scale_factor: f32, vertical_scale_factor: f32) {

    for curr_x in carve_box_start[0]..carve_box_end[0] {
        let x_norm_pos_dist = ((curr_x + chunk_pos[0] as i64 * 16) as f64
                               + 0.5 - pos[0]) / horizontal_scale_factor as f64;

        for curr_z in carve_box_start[2]..carve_box_end[2] {
            let z_norm_pos_dist = ((curr_z + chunk_pos[1] as i64 * 16) as f64
                                   + 0.5 - pos[2]) / horizontal_scale_factor as f64;

            if x_norm_pos_dist.powi(2) + z_norm_pos_dist.powi(2) < 1.0 {
                let mut hit_grass = false;

                for curr_y in (carve_box_start[1]..carve_box_end[1]).rev() {
                    let y_norm_pos_dist =((curr_y - 1) as f64 
                                          + 0.5 - pos[1]) / vertical_scale_factor as f64;

                    if y_norm_pos_dist > -0.7 && x_norm_pos_dist.powi(2) + z_norm_pos_dist.powi(2) + y_norm_pos_dist.powi(2) < 1.0 {
                        let curr_idx = get_chunk_idx(curr_x, curr_y, curr_z);
                        let over_idx = get_chunk_idx(curr_x, curr_y+1, curr_z);
                        let under_idx = get_chunk_idx(curr_x, curr_y-1, curr_z);
                        let block_curr = chunk_data[curr_idx];
                        let block_over = chunk_data[over_idx];

                        // TODO: Mycelium
                        if block_curr == BLOCK_GRASS {
                            hit_grass = true;
                        }
                        if block_is_carvable(block_curr, block_over) {
                            if curr_y < 10 {
                                chunk_data[curr_idx as usize] = BLOCK_LAVA;
                            } else {
                                chunk_data[curr_idx as usize] = BLOCK_AIR;

                                // TODO: If above is sand, set sandstone (remember variation)
                                // TODO: check that curr_idx-1 doesn't underflow into
                                // next column
                                if hit_grass && chunk_data[under_idx] == BLOCK_DIRT {
                                    // TODO: Set correct variant (mycelium)
                                    chunk_data[under_idx] = BLOCK_GRASS;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// should be true if curr is one of:
// stone, dirt, grass (normal, mycelium), hardened clay (stained or not), 
// sandstone (normal, red), thin snow, sand, gravel
// and if above is not water
fn block_is_carvable(curr: u8, above: u8) -> bool {
    above != BLOCK_WATER && above != BLOCK_FLOWING_WATER
}

fn scan_water(chunk_data: &mut [u8], carve_box_start: Pnt3<i64>, 
              carve_box_end: Pnt3<i64>) -> bool {
    for curr_x in carve_box_start[0]..carve_box_end[0] {
        for curr_z in carve_box_start[2]..carve_box_end[2] {
            for curr_y in carve_box_start[1]..carve_box_end[1] {
                if curr_y >= 0 && curr_y < 256 {
                    let idx = get_chunk_idx(curr_x, curr_y, curr_z);
                    let block = chunk_data[idx as usize];

                    if block == BLOCK_WATER || block == BLOCK_FLOWING_WATER {
                        return true;
                    }
                    // TODO: Check only edge of area?
                }
            }
        }
    }
    false
}

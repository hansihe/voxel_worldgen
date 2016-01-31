use ::noise::Seed;
use ::noise::{open_simplex2, open_simplex3};

// https://gist.github.com/hansihe/55c7c8c5ffe03ba4c9e2

//const FACTORS: [f32; 9] = [4.40762211e-01, -1.14970926e+00, 4.59721677e-01, 2.13293359e+01,
//  -5.17718948e+01, 4.00625815e+01, -8.99295761e+00, 6.40966963e-01,
//  -9.41897942e-03];
//const FACTORS: [f32; 11] = [
//    1.50453101e-02, -3.51332521e+00, -3.22604657e-02, 7.46428714e+00,
//    1.44947761e-02, -3.99374423e+00, 1.18800040e-02, -1.34473632e+00,
//    -1.18283342e-02, 2.37064079e+00, 2.71704099e-03];
//pub fn transform_simplex(val: f32) -> f32 {
//    let mut acc = 0f32;
//    acc += FACTORS[0] * val.powi(10);
//    acc += FACTORS[1] * val.powi(9);
//    acc += FACTORS[2] * val.powi(8);
//    acc += FACTORS[3] * val.powi(7);
//    acc += FACTORS[4] * val.powi(6);
//    acc += FACTORS[5] * val.powi(5);
//    acc += FACTORS[6] * val.powi(4);
//    acc += FACTORS[7] * val.powi(3);
//    acc += FACTORS[8] * val.powi(2);
//    acc += FACTORS[9] * val;
//    acc += FACTORS[10];
//    acc.min(1.0).max(-1.0)
//}
use ::simplex_normalized::normalize_simplex;

const START_1: f32 = 0.19273772f32;
const START_2: f32 = 0.23512737f32;
const START_3: f32 = 0.12872678f32;

pub fn divide(bins: i32, samples: i32) -> Vec<i32> {
    let mut sample_list: Vec<i32> = vec![0; bins as usize];
    let mut sample_transformed: Vec<i32> = vec![0; bins as usize];
    let mut sample_transformed2: Vec<i32> = vec![0; bins as usize];
    let seed = Seed::new(1);

    let mut val1 = START_1;
    let mut val2 = START_2;
    let mut val3 = START_3;

    //let mult = bins as f32 / 2.0;
    let mult2 = (bins-1) as f32;

    for _ in 0..samples {
        let sample = open_simplex3(&seed, &[val1, val2, val3]);

        let sample_range = ((sample + 1.0) / 2.0 ) * mult2;
        let sample_range_transformed = ((normalize_simplex(sample) + 1.0) / 2.0) * mult2;

        sample_list[sample_range.floor() as usize] += 1;
        //let p = (transform_simplex(sample_range) * mult2).floor() as usize;
        sample_transformed[sample_range_transformed.floor() as usize] += 1;

        //let y = (transform_simplex_2(sample_range) * mult2).floor() as usize;
        //sample_transformed2[y] += 1;
        //println!("{:?}", samplet);
        //let s2 = transform_simplex(sample_range);
        //let res = ((sample + 1.0) * mult) as i32;
        //let res = (s2 * mult2) as i32;
        //sample_list[res as usize] += 1;

        val1 += 0.2836529;
        if val1 > 2000.0 {
            val1 -= START_1 * 10000.0;
            val2 += 0.3864783;
        }
        if val2 > 2000.0 {
            val2 -= START_2 * 10000.0;
            val3 += 0.2727368;
        }
    }
    
    println!("Orig: {:?}\nTransformed: {:?}", sample_list, sample_transformed);
    sample_list
}

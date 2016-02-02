// Factors derived by sampling the output of the noise function, divide it into 1000
// bins from -1 to 1, taking the cumulative sum, and approximating a polynomial function
// from the resulting curve.
// https://gist.github.com/hansihe/55c7c8c5ffe03ba4c9e2

//const FACTORS_O: [f32; 11] = [
//    1.50453101e-02, -3.51332521e+00, -3.22604657e-02, 7.46428714e+00,
//    1.44947761e-02, -3.99374423e+00, 1.18800040e-02, -1.34473632e+00,
//    -1.18283342e-02, 2.37064079e+00, 2.71704099e-03];
pub fn normalize_simplex(val: f32) -> f32 {
    let mut acc = 0f32;
    acc += FACTORS[0] * val.powi(10);
    acc += FACTORS[1] * val.powi(9);
    acc += FACTORS[2] * val.powi(8);
    acc += FACTORS[3] * val.powi(7);
    acc += FACTORS[4] * val.powi(6);
    acc += FACTORS[5] * val.powi(5);
    acc += FACTORS[6] * val.powi(4);
    acc += FACTORS[7] * val.powi(3);
    acc += FACTORS[8] * val.powi(2);
    acc += FACTORS[9] * val;
    acc += FACTORS[10];
    acc.min(1.0).max(-1.0)
}

const FACTORS: [f32; 11] = [
    6.52302987e-01, -1.72903489e+01, -1.12122237e+00, 2.77098705e+01,
    6.83447908e-01, -1.38410486e+01, -1.54970218e-01, 4.01738791e-01,
    -3.72274565e-03, 2.29025865e+00, 4.93543117e-03];

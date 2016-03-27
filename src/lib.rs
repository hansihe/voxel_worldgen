//extern crate image;
extern crate noise;
extern crate rand;
//extern crate time;
extern crate num;
extern crate nalgebra;

pub mod simplex_normalized;
pub mod layer;
pub mod generators;
pub mod rnd;
pub mod gen;

pub use generators::vanilla::{ WorldGeneratorState, lerp_height_field, test_generate_chunk };
pub use rand::{XorShiftRng, StdRng};

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

use ::layer::GenL;
use ::layer::{ GenIsland, GenZoom, ZoomType, GenReduceOcean };
use ::layer::{ GenSimplex, SimplexNoiseType };
use ::layer::{ GenSimpleFnTransformer, GenSimpleFnMixer };

use ::num::Float;
use ::nalgebra::Pnt2;

pub fn land_mask() -> GenL<bool> {
    let mut src: GenL<bool> = GenIsland::new(1, 4);
    src = GenZoom::new(2000, ZoomType::FUZZY, src);
    src = GenZoom::new(2001, ZoomType::MAJORITY, src);
    src = GenReduceOcean::new(2, 2, src);
    src = GenZoom::new(2002, ZoomType::MAJORITY, src);
    src = GenZoom::new(2003, ZoomType::MAJORITY, src);
    src
}


pub fn biome_map() -> GenL<u8> {

    fn make_biome_index(pos: Pnt2<i32>, temp: f32, wet: f32) -> (u8, u8) {
        let temp_idx = (((temp + 1.0) / 2.0) * 6.0).floor() as u8;
        let wet_idx = (((wet + 1.0) / 2.0) * 4.0).floor() as u8;
        //(wet_idx * 8) + temp_idx
        /*if temp_idx == 0 && wet_idx == 0 {
            (10, 10)
        } else {*/
        (temp_idx, wet_idx)
    }

    fn make_biome_val(pos: Pnt2<i32>, data: (u8, u8)) -> u8 {
        let (temp, wet) = data;
        if temp > 3 {
            0
        } else {
            1
        }
    }

    let mut temp_src = GenSimplex::new(1, 16.0, SimplexNoiseType::Normalized);
    let mut wet_src = GenSimplex::new(2, 16.0, SimplexNoiseType::Normalized);
    let mut biome_idx = GenSimpleFnMixer::new(make_biome_index, temp_src, wet_src);
    GenSimpleFnTransformer::new(make_biome_val, biome_idx)
}

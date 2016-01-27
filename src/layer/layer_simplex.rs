use super::{ GenLayer, LayerLCG };
use ::simplex_normalized::normalize_simplex;
use ::noise::{ Seed, open_simplex2 };
use std::rc::Rc;

#[derive(Clone)]
pub enum SimplexNoiseType {
    Original,
    Normalized,
}

#[derive(Clone)]
pub struct GenSimplex {
    seed: i64,
    noise_type: SimplexNoiseType,
    scale: f32,
}
impl GenSimplex {
    pub fn new(seed: i64, scale: f32, noise_type: SimplexNoiseType) -> Rc<GenSimplex> {
        Rc::new(GenSimplex {
            seed: seed,
            noise_type: noise_type,
            scale: scale,
        })
    }
}
impl GenLayer<f32> for GenSimplex {
    fn gen(&self, seed: i64, area_x: i32, area_y: i32, area_width: i32, area_height: i32) -> Vec<f32> {
        let mut lcg = LayerLCG::new(self.seed, seed);
        let mut buf = Vec::with_capacity((area_width * area_height) as usize);
        
        let seed = Seed::new(lcg.world_seed() as u32);
        for y in 0..area_height {
            for x in 0..area_width {
                let sx = (area_x + x) as f32 / self.scale;
                let sy = (area_y + y) as f32 / self.scale;
                let noise = open_simplex2(&seed, &[sx, sy]);
                buf.push(match self.noise_type {
                    SimplexNoiseType::Original => noise,
                    SimplexNoiseType::Normalized => normalize_simplex(noise),
                });
            }
        }

        buf
    }
}

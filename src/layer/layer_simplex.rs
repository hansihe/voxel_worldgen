use super::{ GenLayer, LayerLCG };
use ::simplex_normalized::normalize_simplex;
use ::noise::{ Seed, open_simplex2 };
use std::rc::Rc;
use ::nalgebra::{ Vec2, Pnt2 };
use ::gen::unit::GenUnit2;

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
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> GenUnit2<f32> {
        let lcg = LayerLCG::new(self.seed, seed);
        let mut buf = Vec::with_capacity((size.x * size.y) as usize);
        
        let seed = Seed::new(lcg.world_seed() as u32);
        for y in 0..size.y {
            for x in 0..size.x {
                let sx = (pos.x + x as i32) as f32 / self.scale;
                let sy = (pos.y + y as i32) as f32 / self.scale;
                let noise = open_simplex2(&seed, &[sx, sy]);
                buf.push(match self.noise_type {
                    SimplexNoiseType::Original => noise,
                    SimplexNoiseType::Normalized => normalize_simplex(noise),
                });
            }
        }

        GenUnit2::new2_from_vec(size, buf)
    }
}

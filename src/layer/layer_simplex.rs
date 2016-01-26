use super::{ GenLayer, LayerLCG };
use ::simplex_normalized::normalize_simplex;
use ::noise::{ Seed, open_simplex2 };

pub enum SimplexNoiseType {
    Original,
    Normalized,
}

pub struct GenSimplex {
    lcg: LayerLCG,
    noise_type: SimplexNoiseType,
    scale: f32,
}
impl GenSimplex {
    pub fn new(seed: i64, scale: f32, noise_type: SimplexNoiseType) -> Box<GenSimplex> {
        Box::new(GenSimplex {
            lcg: LayerLCG::new(seed),
            noise_type: noise_type,
            scale: scale,
        })
    }
}
impl GenLayer<f32> for GenSimplex {
    fn seed_world(&mut self, seed: i64) {
        self.lcg.seed_world(seed);
    }
    fn gen(&mut self, area_x: i32, area_y: i32, area_width: i32, area_height: i32) -> Vec<f32> {
        let mut buf = Vec::with_capacity((area_width * area_height) as usize);
        
        let seed = Seed::new(self.lcg.world_seed() as u32);
        for y in 0..area_height {
            for x in 0..area_width {
                let sx = self.scale * (area_x + x) as f32;
                let sy = self.scale * (area_y + y) as f32;
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

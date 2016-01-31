use ::rand::{ Rng, Rand };
use ::noise::{ Seed, open_simplex3, open_simplex2 };

pub struct OctavesSeed {
    octaves: u32,
    seeds: Box<Vec<Seed>>,
}
impl OctavesSeed {
    pub fn new<R: Rng>(rand: &mut R, octaves: u32) -> OctavesSeed {
        let mut seeds: Vec<Seed> = Vec::with_capacity(octaves as usize);
        for num in 0..octaves {
            seeds.push(rand.gen());
        }
        OctavesSeed {
            octaves: octaves,
            seeds: Box::new(seeds),
        }
    }
    pub fn simplex3(&self, pos: &[f64; 3]) -> f64 {
        let mut acc = 0f64;
        let mut octave_scale = 1f64;

        for octave_seed in self.seeds.iter() {
            let pos_offset = [pos[0]*octave_scale, pos[1]*octave_scale, 
                pos[2]*octave_scale];
            acc += open_simplex3(octave_seed, &pos_offset) / octave_scale;
            octave_scale *= 2.0;
        }
        acc.max(-1.0).min(1.0)
    }
    pub fn simplex2(&self, pos: &[f64; 2]) -> f64 {
        let mut acc = 0f64;
        let mut octave_scale = 1f64;

        for octave_seed in self.seeds.iter() {
            let pos_offset = [pos[0]*octave_scale, pos[1]*octave_scale];
            acc += open_simplex2(octave_seed, &pos_offset) / octave_scale;
            octave_scale *= 2.0;
        }
        acc.max(-1.0).min(1.0)
    }
}

pub fn simplex3_octaves(seed: &OctavesSeed, pos: &[f64; 3]) -> f64 {
    seed.simplex3(pos)
    /*let mut acc = 0f64;
    let mut octave_scale = 1f64;

    for octave_seed in seed.seeds.iter() {
        let pos_offset = [pos[0]*octave_scale, pos[1]*octave_scale, 
            pos[2]*octave_scale];
        acc += open_simplex3(octave_seed, &pos_offset) / octave_scale;
        octave_scale *= 2.0;
    }
    acc*/
}



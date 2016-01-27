const LCG_MULT: i64 = 6364136223846793005;
const LCG_ADD: i64 = 1442695040888963407;
#[inline(always)]
fn twist(seed: i64) -> i64 {
    seed.wrapping_mul(seed.wrapping_mul(LCG_MULT).wrapping_add(LCG_ADD))
}

#[derive(Clone)]
pub struct LayerLCG {
    base_seed: i64,
    world_seed: i64,
    pos_seed: i64,
}
impl LayerLCG {
    pub fn new(base_seed: i64, world_seed: i64) -> LayerLCG {
        let mut lcg = LayerLCG {
            base_seed: base_seed,
            world_seed: base_seed,
            pos_seed: 0,
        };
        lcg.seed_world(world_seed);
        lcg
    }
    /// Seed the LCG from the base seed and the world seed
    pub fn seed_world(&mut self, world_seed: i64) {
        self.world_seed = twist(world_seed);
        self.world_seed = self.world_seed.wrapping_add(self.base_seed);
        self.world_seed = twist(self.world_seed);
        self.world_seed = self.world_seed.wrapping_add(self.base_seed);
        self.world_seed = twist(self.world_seed);
        self.world_seed = self.world_seed.wrapping_add(self.base_seed);
    }
    pub fn world_seed(&self) -> i64 {
        self.world_seed
    }
    /// Seed the LCG from the current world seed and the position
    pub fn seed_pos(&mut self, x: i64, z: i64) {
        self.pos_seed = twist(self.world_seed);
        self.pos_seed = self.pos_seed.wrapping_add(x);
        self.pos_seed = twist(self.pos_seed);
        self.pos_seed = self.pos_seed.wrapping_add(z);
        self.pos_seed = twist(self.pos_seed);
        self.pos_seed = self.pos_seed.wrapping_add(x);
        self.pos_seed = twist(self.pos_seed);
        self.pos_seed = self.pos_seed.wrapping_add(z);
    }
    /// Next random number n from 0<=n<max
    pub fn next_int(&mut self, max: i32) -> i32 {
        let mut number = (self.pos_seed >> 24).wrapping_rem(max as i64) as i32;
        if number < 0 { number += max; }
        
        self.pos_seed = twist(self.pos_seed);
        self.pos_seed = self.pos_seed.wrapping_add(self.world_seed);

        number
    }

    pub fn random_from_2<T>(&mut self, i1: T, i2: T) -> T {
        match self.next_int(2) {
            0 => i1,
            1 => i2,
            _ => unreachable!(),
        }
    }
    pub fn random_from_4<T>(&mut self, i1: T, i2: T, i3: T, i4: T) -> T {
        match self.next_int(4) {
            0 => i1,
            1 => i2,
            2 => i3,
            3 => i4,
            _ => unreachable!(),
        }
    }
}

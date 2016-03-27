pub mod simplex_octaves;
pub use self::simplex_octaves::{ OctavesSeed, simplex3_octaves };

use ::rand::{ XorShiftRng, SeedableRng };

const LCG_MULT: u64 = 6364136223846793005;
const LCG_ADD: u64 = 1442695040888963407;
const INIT_VAL: u64 = 781637684348;
#[inline(always)]
fn twist(seed: u64) -> u64 {
    seed.wrapping_mul(seed.wrapping_mul(LCG_MULT).wrapping_add(LCG_ADD))
}

pub struct RngBuilder {
    state: u64,
}
impl RngBuilder {
    pub fn init() -> RngBuilder {
        RngBuilder { state: INIT_VAL }
    }
    pub fn mix(mut self, mix: u64) -> RngBuilder {
        let mut state = self.state;

        state = twist(state);
        state = state.wrapping_add(mix);
        state = twist(state);
        state = state.wrapping_add(mix);

        self.state = state;
        self
    }
    fn next_u32(&mut self) -> u32 {
        let num = (self.state >> 24) as u32;
        self.state = twist(self.state);
        self.state = twist(self.state);
        num
    }
    pub fn build(mut self) -> XorShiftRng {
        let s1 = self.next_u32();
        let s2 = self.next_u32();
        let s3 = self.next_u32();
        let s4 = self.next_u32();

        SeedableRng::from_seed([s1, s2, s3, s4])
    }
}

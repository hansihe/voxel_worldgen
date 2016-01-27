use std::rc::Rc;
use super::{ GenLayer, LayerLCG };

#[derive(Clone)]
pub struct GenIsland {
    seed: i64,
    island_chance: i32,
}
impl GenIsland {
    pub fn new(seed: i64, island_chance: i32) -> Rc<GenIsland> {
        Rc::new(GenIsland {
            seed: seed,
            island_chance: island_chance,
        })
    }
}
impl GenLayer<bool> for GenIsland {
    fn gen(&self, seed: i64, area_x: i32, area_y: i32, area_width: i32, area_height: i32
           ) -> Vec<bool> {
        let mut lcg = LayerLCG::new(self.seed, seed);
        let mut buf = Vec::with_capacity((area_width * area_height) as usize);

        for y in 0..area_height {
            for x in 0..area_width {
                lcg.seed_pos((area_x + x) as i64, (area_y + y) as i64);
                let has_island = lcg.next_int(self.island_chance) == 0;
                buf.push(has_island)
            }
        }

        // Make sure the center is always an island.
        // Keep outside of loop to minimize pipeline stalls
        if area_x > -area_width && area_x <= 0 && area_y >= -area_height && area_y < 0 {
            buf[(-area_x + -area_y * area_width) as usize] = true;
        }

        buf
    }
}

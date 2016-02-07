use std::rc::Rc;
use super::{ GenLayer, LayerLCG };
use ::nalgebra::{ Vec2, Pnt2 };
use ::gen::unit::GenUnit2;

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
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> GenUnit2<bool> {
        let mut lcg = LayerLCG::new(self.seed, seed);
        let mut buf = Vec::with_capacity((size.x*size.y) as usize);

        for y in 0..size.y {
            for x in 0..size.x {
                lcg.seed_pos((pos.x+x as i32) as i64, (pos.y+y as i32) as i64);
                let has_island = lcg.next_int(self.island_chance) == 0;
                buf.push(has_island)
            }
        }

        // Make sure the center is always an island.
        // Keep outside of loop to minimize pipeline stalls
        if pos.x > -(size.x as i32) && pos.x <= 0 && pos.y >= -(size.y as i32) && pos.y < 0 {
            buf[(-pos.x + -pos.y * size.x as i32) as usize] = true;
        }

        GenUnit2::new2_from_vec(size, buf)
    }
}

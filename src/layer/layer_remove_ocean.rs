use super::{ GenLayer, LayerLCG };
use std::rc::Rc;
use ::nalgebra::{ Vec2, Pnt2 };
use ::gen::unit::GenUnit2;

#[derive(Clone)]
pub struct GenReduceOcean {
    seed: i64,
    source: Rc<GenLayer<bool>>,
    factor: i32,
}
impl GenReduceOcean {
    pub fn new(seed: i64, factor: i32, source: Rc<GenLayer<bool>>) -> Rc<GenReduceOcean> {
        Rc::new(GenReduceOcean {
            source: source,
            seed: seed,
            factor: factor,
        })
    }
}
impl GenLayer<bool> for GenReduceOcean {
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> GenUnit2<bool> {
        let mut lcg = LayerLCG::new(self.seed, seed);
        let source = self.source.gen(seed, pos-Vec2::new(1, 1), size+Vec2::new(2, 2));
        let mut sink = Vec::with_capacity((size.x * size.y) as usize);

        for y in 0..size.y {
            for x in 0..size.x {
                let v1 = source[(x+1 + (y+1-1) * (size.x + 2)) as usize];
                let v2 = source[(x+1+1 + (y+1) * (size.x + 2)) as usize];
                let v3 = source[(x+1-1 + (y+1) * (size.x + 2)) as usize];
                let v4 = source[(x+1 + (y+1+1) * (size.x + 2)) as usize];
                let v5 = source[(x+1 + (y+1) * (size.x + 2)) as usize];

                lcg.seed_pos((pos.x + x as i32) as i64, (pos.y + y as i32) as i64);
                sink.push(if !v1 && !v2 && !v3 && !v4 && !v5
                          && lcg.next_int(self.factor) == 0 { true } else { v5 })
            }
        }

        GenUnit2::new2_from_vec(size, sink)
    }
}

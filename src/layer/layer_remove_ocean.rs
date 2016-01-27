use super::{ GenLayer, LayerLCG };
use std::rc::Rc;

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
    fn gen(&self, seed: i64, area_x: i32, area_y: i32, area_width: i32, area_height: i32
           ) -> Vec<bool> {
        let mut lcg = LayerLCG::new(self.seed, seed);
        let source = self.source.gen(seed, area_x-1, area_y-1, area_width+2, area_height+2);
        let mut sink = Vec::with_capacity((area_width * area_height) as usize);

        for y in 0..area_height {
            for x in 0..area_width {
                let v1 = source[(x+1 + (y+1-1) * (area_width + 2)) as usize];
                let v2 = source[(x+1+1 + (y+1) * (area_width + 2)) as usize];
                let v3 = source[(x+1-1 + (y+1) * (area_width + 2)) as usize];
                let v4 = source[(x+1 + (y+1+1) * (area_width + 2)) as usize];
                let v5 = source[(x+1 + (y+1) * (area_width + 2)) as usize];

                lcg.seed_pos((area_x + x) as i64, (area_y + y) as i64);
                sink.push(if !v1 && !v2 && !v3 && !v4 && !v5
                          && lcg.next_int(self.factor) == 0 { true } else { v5 })
            }
        }

        sink
    }
}

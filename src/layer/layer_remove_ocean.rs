use super::{ GenLayer, LayerLCG };

pub struct GenReduceOcean {
    lcg: LayerLCG,
    source: Box<GenLayer<bool>>,
    factor: i32,
}
impl GenReduceOcean {
    pub fn new(seed: i64, factor: i32, source: Box<GenLayer<bool>>) -> Box<GenReduceOcean> {
        Box::new(GenReduceOcean {
            source: source,
            lcg: LayerLCG::new(seed),
            factor: factor,
        })
    }
}
impl GenLayer<bool> for GenReduceOcean {
    fn seed_world(&mut self, seed: i64) {
        self.lcg.seed_world(seed);
        self.source.seed_world(seed);
    }
    fn gen(&mut self, area_x: i32, area_y: i32, area_width: i32, area_height: i32
           ) -> Vec<bool> {
        let source = self.source.gen(area_x-1, area_y-1, area_width+2, area_height+2);
        let mut sink = Vec::with_capacity((area_width * area_height) as usize);

        for y in 0..area_height {
            for x in 0..area_width {
                let v1 = source[(x+1 + (y+1-1) * (area_width + 2)) as usize];
                let v2 = source[(x+1+1 + (y+1) * (area_width + 2)) as usize];
                let v3 = source[(x+1-1 + (y+1) * (area_width + 2)) as usize];
                let v4 = source[(x+1 + (y+1+1) * (area_width + 2)) as usize];
                let v5 = source[(x+1 + (y+1) * (area_width + 2)) as usize];

                self.lcg.seed_pos((area_x + x) as i64, (area_y + y) as i64);
                sink.push(if !v1 && !v2 && !v3 && !v4 && !v5
                          && self.lcg.next_int(self.factor) == 0 { true } else { v5 })
            }
        }

        sink
    }
}

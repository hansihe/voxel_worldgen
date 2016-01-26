use super::{ GenLayer, LayerLCG };

/*pub enum BlendMode {
    Normal,
    // Randomly selects pixels from each layer.
    // `factor`: between 0 (only layer 1) and 1 (only layer 2)
    //Dissolve(u32),

    /// Logical AND
    And,
    /// Logical OR
    Or,
}

pub struct GenBlend {
    mode: BlendMode,
    source_a: Box<GenLayer>,
    source_b: Box<GenLayer>,
}
impl GenBlend {
    pub fn new(mode: BlendMode, source_a: Box<GenLayer>, source_b: Box<GenLayer>
           ) -> Box<GenBlend> {
        Box::new(GenBlend {
            mode: mode,
            source_a: source_a,
            source_b: source_b,
        })
    }
}
impl GenLayer for GenBlend {
    fn seed_world(&mut self, seed: i64) {
        self.source_a.seed_world(seed);
        self.source_b.seed_world(seed);
    }
    fn gen(&mut self, x: i32, y: i32, w: i32, h: i32) -> Vec<i32> {
        match self.mode {
            BlendMode::Normal => self.source_a.gen(x, y, w, h),
            BlendMode::And => {
                let a = self.source_a.gen(x, y, w, h);
                let b = self.source_b.gen(x, y, w, h);
                a.iter().zip(b.iter()).map(|(av, bv)| {
                    if *av == 1 && *bv == 1 { 1 } else { 0 }
                }).collect()
            },
            BlendMode::Or => {
                let a = self.source_a.gen(x, y, w, h);
                let b = self.source_b.gen(x, y, w, h);
                a.iter().zip(b.iter()).map(|(av, bv)| {
                    if *av == 1 || *bv == 1 { 1 } else { 0 }
                }).collect()
            },
            //_ => unimplemented!(),
        }
    }
}*/
// TODO: Split this into different variants

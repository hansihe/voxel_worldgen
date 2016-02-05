use std::rc::Rc;
use super::{ GenLayer, LayerLCG };

const WEIGHT_FIELD: [f64; 25] = [
    3.0, 5.0, 6.0, 5.0, 3.0,
    5.0, 6.0, 9.0, 6.0, 5.0,
    6.0, 9.0, 20.0, 9.0, 6.0,
    5.0, 6.0, 9.0, 6.0, 5.0,
    3.0, 5.0, 6.0, 5.0, 3.0];

#[derive(Clone)]
pub struct GenBiomesHeightmap {
    parent: Rc<GenLayer<u32>>,
}
impl GenBiomesHeightmap {
    pub fn new(parent: Rc<GenLayer<u32>>) -> Rc<GenBiomesHeightmap> {
        GenBiomesHeightmap {
            parent: parent,
        }
    }
}
impl GenLayer<f64> for GenBiomesHeightmap {
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> Vec<f64> {
        let biomes = self.parent.gen(seed, area_x-2, area_y-2, area_width+4, area_width+4);
    }
}

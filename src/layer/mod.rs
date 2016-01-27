use std::rc::Rc;

mod lcg;
pub use self::lcg::LayerLCG;

mod layer_island;
pub use self::layer_island::GenIsland;
mod layer_zoom;
pub use self::layer_zoom::{ GenZoom, ZoomType };
//mod layer_blend;
//pub use self::layer_blend::{ GenBlend, BlendMode };
mod layer_remove_ocean;
pub use self::layer_remove_ocean::GenReduceOcean;
mod layer_simplex;
pub use self::layer_simplex::{ GenSimplex, SimplexNoiseType };

mod layer_simple_fn;
pub use self::layer_simple_fn::{ GenSimpleFn, GenSimpleFnTransformer, GenSimpleFnMixer };

pub trait GenLayer<O: Sized> {
    fn gen(&self, world_seed: i64, x: i32, z: i32, width: i32, height: i32) -> Vec<O>;
}

pub type GenL<O> = Rc<GenLayer<O>>;

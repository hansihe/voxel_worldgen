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
pub use self::layer_simple_fn::GenSimpleFn;

pub trait GenLayer<O: Sized> {
    fn seed_world(&mut self, seed: i64);
    fn gen(&mut self, x: i32, z: i32, width: i32, height: i32) -> Vec<O>;
}

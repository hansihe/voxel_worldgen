use super::{ GenLayer };

pub struct GenSimpleFn<O> {
    fun: fn(x: i32, y: i32) -> O,
}
impl <O> GenSimpleFn<O> {
    pub fn new<O1>(fun: fn(x: i32, y: i32) -> O) -> Box<GenSimpleFn<O>> {
        Box::new(GenSimpleFn {
            fun: fun
        })
    }
}
impl <O> GenLayer<O> for GenSimpleFn<O> {
    fn seed_world(&mut self, seed: i64) {}
    fn gen(&mut self, x: i32, y: i32, area_x: i32, area_y: i32) -> Vec<O> {
        let mut sink_buf = Vec::with_capacity((area_x * area_y) as usize);
        let fun = self.fun;
        for sample_y in 0..area_y {
            for sample_x in 0..area_x {
                sink_buf.push(fun(x, y));
            }
        }
        sink_buf
    }
}

use super::{ GenLayer };
use std::rc::Rc;

pub struct GenSimpleFn<O> {
    fun: fn(i32, i32) -> O,
}
impl <O> GenSimpleFn<O> {
    pub fn new(fun: fn(i32, i32) -> O) -> Rc<GenSimpleFn<O>> {
        Rc::new(GenSimpleFn {
            fun: fun
        })
    }
}
impl <O> GenLayer<O> for GenSimpleFn<O> {
    fn gen(&self, _seed: i64, x: i32, y: i32, area_x: i32, area_y: i32) -> Vec<O> {
        let mut sink_buf = Vec::with_capacity((area_x * area_y) as usize);
        let fun = self.fun;
        for sample_y in 0..area_y {
            for sample_x in 0..area_x {
                sink_buf.push(fun(x+sample_x, y+sample_y));
            }
        }
        sink_buf
    }
}

pub struct GenSimpleFnTransformer<I, O> {
    fun: fn(i32, i32, I) -> O,
    source: Rc<GenLayer<I>>,
}
impl <I, O> GenSimpleFnTransformer<I, O> {
    pub fn new(fun: fn(i32, i32, I) -> O, source: Rc<GenLayer<I>>) -> Rc<GenSimpleFnTransformer<I, O>> {
        Rc::new(GenSimpleFnTransformer {
            fun: fun,
            source: source,
        })
    }
}
// Could be made faster by specializing. Will not do until it poses a problem.
impl <I: Copy, O> GenLayer<O> for GenSimpleFnTransformer<I, O> {
    fn gen(&self, seed: i64, x: i32, y: i32, area_x: i32, area_y: i32) -> Vec<O> {
        let buf = self.source.gen(seed, x, y, area_x, area_y);
        let mut out_buf = Vec::with_capacity(buf.len());
        let fun = self.fun;
        for sample_y in 0..area_y {
            for sample_x in 0..area_x {
                let idx = ((sample_y * area_x) + sample_x) as usize;
                out_buf.push(fun(sample_x, sample_y, buf[idx]));
            }
        }
        out_buf
    }
}


pub struct GenSimpleFnMixer<I1, I2, O> {
    fun: fn(i32, i32, I1, I2) -> O,
    source1: Rc<GenLayer<I1>>,
    source2: Rc<GenLayer<I2>>,
}
impl <I1, I2, O> GenSimpleFnMixer<I1, I2, O> {
    pub fn new(fun: fn(i32, i32, I1, I2) -> O, 
               source1: Rc<GenLayer<I1>>, source2: Rc<GenLayer<I2>>
               ) -> Rc<GenSimpleFnMixer<I1, I2, O>> {
        Rc::new(GenSimpleFnMixer {
            fun: fun,
            source1: source1,
            source2: source2,
        })
    }
}
impl <I1: Copy, I2: Copy, O> GenLayer<O> for GenSimpleFnMixer<I1, I2, O> {
    fn gen(&self, seed: i64, x: i32, y: i32, area_x: i32, area_y: i32) -> Vec<O> {
        let src1_buf = self.source1.gen(seed, x, y, area_x, area_y);
        let src2_buf = self.source2.gen(seed, x, y, area_x, area_y);
        let mut out_buf = Vec::with_capacity(src1_buf.len());
        let fun = self.fun;
        for sample_y in 0..area_y {
            for sample_x in 0..area_x {
                let idx = ((sample_y * area_x) + sample_x) as usize;
                out_buf.push(fun(sample_x, sample_y, src1_buf[idx], src2_buf[idx]));
            }
        }
        out_buf
    }
}

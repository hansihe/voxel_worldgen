use super::{ GenLayer };
use std::rc::Rc;
use ::nalgebra::{ Vec2, Pnt2 };

pub struct GenSimpleFn<O> {
    fun: fn(Pnt2<i32>) -> O,
}
impl <O> GenSimpleFn<O> {
    pub fn new(fun: fn(Pnt2<i32>) -> O) -> Rc<GenSimpleFn<O>> {
        Rc::new(GenSimpleFn {
            fun: fun
        })
    }
}
impl <O> GenLayer<O> for GenSimpleFn<O> {
    fn gen(&self, _seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> Vec<O> {
        let mut sink_buf = Vec::with_capacity((size.x*size.y) as usize);
        let fun = self.fun;
        for sample_y in 0..size.y {
            for sample_x in 0..size.x {
                sink_buf.push(fun(pos+Vec2::new(sample_x as i32, sample_y as i32)));
            }
        }
        sink_buf
    }
}

pub struct GenSimpleFnTransformer<I, O> {
    fun: fn(Pnt2<i32>, I) -> O,
    source: Rc<GenLayer<I>>,
}
impl <I, O> GenSimpleFnTransformer<I, O> {
    pub fn new(fun: fn(Pnt2<i32>, I) -> O, source: Rc<GenLayer<I>>) -> Rc<GenSimpleFnTransformer<I, O>> {
        Rc::new(GenSimpleFnTransformer {
            fun: fun,
            source: source,
        })
    }
}
// Could be made faster by specializing. Will not do until it poses a problem.
impl <I: Copy, O> GenLayer<O> for GenSimpleFnTransformer<I, O> {
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> Vec<O> {
        let buf = self.source.gen(seed, pos, size);
        let mut out_buf = Vec::with_capacity(buf.len());
        let fun = self.fun;
        for sample_y in 0..size.y {
            for sample_x in 0..size.x {
                let idx = ((sample_y * size.x) + sample_x) as usize;
                out_buf.push(fun(pos+Vec2::new(sample_x as i32, sample_y as i32), buf[idx]));
            }
        }
        out_buf
    }
}


pub struct GenSimpleFnMixer<I1, I2, O> {
    fun: fn(Pnt2<i32>, I1, I2) -> O,
    source1: Rc<GenLayer<I1>>,
    source2: Rc<GenLayer<I2>>,
}
impl <I1, I2, O> GenSimpleFnMixer<I1, I2, O> {
    pub fn new(fun: fn(Pnt2<i32>, I1, I2) -> O, 
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
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> Vec<O> {
        let src1_buf = self.source1.gen(seed, pos, size);
        let src2_buf = self.source2.gen(seed, pos, size);
        let mut out_buf = Vec::with_capacity(src1_buf.len());
        let fun = self.fun;
        for sample_y in 0..size.y {
            for sample_x in 0..size.x {
                let idx = ((sample_y * size.x) + sample_x) as usize;
                out_buf.push(fun(pos+Vec2::new(sample_x as i32, sample_y as i32), src1_buf[idx], src2_buf[idx]));
            }
        }
        out_buf
    }
}

use super::{ GenLayer, LayerLCG };
use std::rc::Rc;
use ::nalgebra::{ Vec2, Pnt2 };

fn frequent_or_random<T>(rand: &mut LayerLCG, i1: T, i2: T, i3: T, i4: T) -> T where T: PartialEq {
    if i2 == i3 && i3 == i4 { i2 }
    else if i1 == i2 && i2 == i3 { i1 }
    else if i1 == i2 && i2 == i4 { i1 }
    else if i1 == i3 && i3 == i4 { i1 }
    else if i1 == i2 && i3 != i4 { i1 }
    else if i1 == i3 && i2 != i4 { i1 }
    else if i1 == i4 && i2 != i3 { i1 }
    else if i2 == i3 && i1 != i4 { i2 }
    else if i2 == i4 && i1 != i3 { i2 }
    else if i3 == i4 && i1 != i2 { i3 }
    else { rand.random_from_4(i1, i2, i3, i4) }
}

#[derive(PartialEq, Clone)]
pub enum ZoomType {
    MAJORITY,
    FUZZY,
}

#[derive(Clone)]
pub struct GenZoom<I> {
    source: Rc<GenLayer<I>>,
    seed: i64,
    zoom_type: ZoomType,
}
impl<I> GenZoom<I> {
    pub fn new(seed: i64, zoom_type: ZoomType, source: Rc<GenLayer<I>>
               ) -> Rc<GenZoom<I>> {
        Rc::new(GenZoom {
            source: source,
            seed: seed,
            zoom_type: zoom_type,
        })
    }
}
impl<I> GenLayer<I> for GenZoom<I> where I: PartialEq + Copy {
    fn gen(&self, seed: i64, pos: Pnt2<i32>, size: Vec2<u32>) -> Vec<I> {
        let mut lcg = LayerLCG::new(self.seed, seed);

        let source_x = pos.x >> 1;
        let source_y = pos.y >> 1;
        // +1 for sampling 2x2 area everywhere
        // +1 for the bit we lose wen shifting
        let source_w = (size.x >> 1) + 2; // 514
        let source_h = (size.y >> 1) + 2;
        let source_buf = self.source.gen(seed, 
                                         Pnt2::new(source_x, source_y), 
                                         Vec2::new(source_w, source_h));
        
        // We produce data for both cases of the lost byte,
        // reassemble correctly later on. Produce extra data.
        let sink_unaligned_w = (source_w - 1) << 1;
        let sink_unaligned_h = (source_h - 1) << 1;

        let unaligned_len = (sink_unaligned_w * sink_unaligned_h) as usize;
        let mut sink_unaligned_buf = Vec::with_capacity(unaligned_len);
        // Although this is unsafe, we will be writing to it right after, so it
        // should be fine.
        unsafe { sink_unaligned_buf.set_len(unaligned_len) };
        
        for sink_sample_y in 0..source_h-1 {

            let mut sample00 = 
                source_buf[(0+0 + ((sink_sample_y+0) * source_w)) as usize];
            let mut sample01 = 
                source_buf[(0+0 + ((sink_sample_y+1) * source_w)) as usize];

            for sink_sample_x in 0..source_w-1 {

                lcg.seed_pos((sink_sample_x as i32 + (source_x << 1)) as i64, 
                                  ((sink_sample_y as i32 + source_y) << 1) as i64);

                let sample10 = source_buf[
                    (sink_sample_x+1 + ((sink_sample_y+0) * source_w)) as usize];
                let sample11 = source_buf[
                    (sink_sample_x+1 + ((sink_sample_y+1) * source_w)) as usize];

                let base_x = sink_sample_x << 1;
                let base_y = sink_sample_y << 1;
                let base_idx = (sink_unaligned_w * base_y) + base_x;
                
                sink_unaligned_buf[base_idx as usize] = sample00;
                sink_unaligned_buf[(base_idx + 1) as usize] = 
                    lcg.random_from_2(sample00, sample10);
                sink_unaligned_buf[(base_idx + sink_unaligned_w) as usize] = 
                    lcg.random_from_2(sample00, sample01);
                if self.zoom_type == ZoomType::MAJORITY {
                    sink_unaligned_buf[(base_idx + sink_unaligned_w + 1) as usize] = 
                        frequent_or_random(&mut lcg, 
                                           sample00, sample10, 
                                           sample01, sample11);
                } else if self.zoom_type == ZoomType::FUZZY {
                    sink_unaligned_buf[(base_idx + sink_unaligned_w + 1) as usize] = 
                        lcg.random_from_4(sample00, sample10, 
                                          sample01, sample11);
                }

                sample00 = sample10;
                sample01 = sample11;
            }
        }

        let mut final_buf = Vec::with_capacity((size.x * size.y) as usize);
        // TODO: Optimize
        for row in 0..size.y {
            let source_start = (row as i32 + (pos.y & 1)) * sink_unaligned_w as i32 + (pos.x & 1);
            //let destStart = row * sink_w;
            let size = size.x;
            for col in  0..size {
                final_buf.push(sink_unaligned_buf[(source_start + col as i32) as usize])
            }
        }

        final_buf
    }
}

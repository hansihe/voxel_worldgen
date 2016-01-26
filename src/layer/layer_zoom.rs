use super::{ GenLayer, LayerLCG };

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

#[derive(PartialEq)]
pub enum ZoomType {
    MAJORITY,
    FUZZY,
}

pub struct GenZoom<I> {
    source: Box<GenLayer<I>>,
    lcg: LayerLCG,
    zoom_type: ZoomType,
}
impl<O> GenZoom<O> {
    pub fn new<O1>(seed: i64, zoom_type: ZoomType, source: Box<GenLayer<O1>>
               ) -> Box<GenZoom<O1>> {
        Box::new(GenZoom {
            source: source,
            lcg: LayerLCG::new(seed),
            zoom_type: zoom_type,
        })
    }
}
impl<O> GenLayer<O> for GenZoom<O> where O: PartialEq + Copy {
    fn seed_world(&mut self, seed: i64) {
        self.lcg.seed_world(seed);
        self.source.seed_world(seed);
    }
    fn gen(&mut self, sink_x: i32, sink_y: i32, sink_w: i32, sink_h: i32) -> Vec<O> {
        let source_x = sink_x >> 1;
        let source_y = sink_y >> 1;
        // +1 for sampling 2x2 area everywhere
        // +1 for the bit we lose wen shifting
        let source_w = (sink_w >> 1) + 2; // 514
        let source_h = (sink_h >> 1) + 2;
        let source_buf = self.source.gen(source_x, source_y, source_w, source_h);
        
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

                self.lcg.seed_pos((sink_sample_x + (source_x << 1)) as i64, 
                                  ((sink_sample_y + source_y) << 1) as i64);

                let sample10 = source_buf[
                    (sink_sample_x+1 + ((sink_sample_y+0) * source_w)) as usize];
                let sample11 = source_buf[
                    (sink_sample_x+1 + ((sink_sample_y+1) * source_w)) as usize];

                let base_x = sink_sample_x << 1;
                let base_y = sink_sample_y << 1;
                let base_idx = (sink_unaligned_w * base_y) + base_x;
                
                sink_unaligned_buf[base_idx as usize] = sample00;
                sink_unaligned_buf[(base_idx + 1) as usize] = 
                    self.lcg.random_from_2(sample00, sample10);
                sink_unaligned_buf[(base_idx + sink_unaligned_w) as usize] = 
                    self.lcg.random_from_2(sample00, sample01);
                if self.zoom_type == ZoomType::MAJORITY {
                    sink_unaligned_buf[(base_idx + sink_unaligned_w + 1) as usize] = 
                        frequent_or_random(&mut self.lcg, 
                                           sample00, sample10, 
                                           sample01, sample11);
                } else if self.zoom_type == ZoomType::FUZZY {
                    sink_unaligned_buf[(base_idx + sink_unaligned_w + 1) as usize] = 
                        self.lcg.random_from_4(sample00, sample10, 
                                               sample01, sample11);
                }

                sample00 = sample10;
                sample01 = sample11;
            }
        }

        let mut final_buf = Vec::with_capacity((sink_w * sink_h) as usize);
        // TODO: Optimize
        for row in 0..sink_h {
            let source_start = (row + (sink_y & 1)) * sink_unaligned_w + (sink_x & 1);
            //let destStart = row * sink_w;
            let size = sink_w;
            for col in  0..size {
                final_buf.push(sink_unaligned_buf[(source_start + col) as usize])
            }
        }

        final_buf
    }
}

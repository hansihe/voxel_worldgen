use ::nalgebra::{ Vec3, Pnt3, cast };
use std::ops::{ Index, IndexMut };
use ::rnd::OctavesSeed;

pub struct GenUnit3<N> {
    pub size: Vec3<u32>,
    pub data: Vec<N>,
}

impl<N: Sized> GenUnit3<N> {
    fn new(size: Vec3<u32>, default: N) -> GenUnit3<N> where N: Clone {
        let capacity: usize = cast(size.x * size.y * size.z);
        GenUnit3 {
            size: size,
            data: vec![default; capacity],
        }
    }
    fn gen_simplex(seed: &OctavesSeed, pos: Pnt3<f64>, size: Vec3<u32>,
                   scale: Vec3<f64>, mult: f64) -> GenUnit3<f64> {
        let scaled = pos.to_vec() * scale;

        let capacity: usize = cast(size.x * size.y * size.z);
        let mut data = Vec::with_capacity(capacity);

        for x_pos in 0..size.x {
            for z_pos in 0..size.z {
                for y_pos in 0..size.y {
                    let scaled_pos = (scaled + (pos.to_vec() * scale)) / 8192.0;
                    let val = seed.simplex3(scaled_pos.as_ref()) * mult;
                    data.push(val);
                }
            }
        }

        GenUnit3 {
            size: size,
            data: data,
        }
    }

    fn to_idx(&self, pos: Pnt3<u32>) -> usize {
        debug_assert!(pos.x < self.size.x);
        debug_assert!(pos.y < self.size.y);
        debug_assert!(pos.z < self.size.z);

        (pos.x + (pos.z*self.size.x) + (pos.y*self.size.x*self.size.z)) as usize
    }
    fn from_idx(&self, mut idx: usize) -> Pnt3<u32> {
        let x = idx % self.size.x as usize;
        idx /= self.size.x as usize;
        let z = idx % self.size.z as usize;
        idx /= self.size.z as usize;
        let y = idx;
        Pnt3::new(x as u32, y as u32, z as u32)
    }
}

impl<N> Index<usize> for GenUnit3<N> {
    type Output = N;
    fn index(&self, index: usize) -> &N { &self.data[index] }
}
impl<N> IndexMut<usize> for GenUnit3<N> {
    fn index_mut(&mut self, index: usize) -> &mut N { &mut self.data[index] }
}

impl<N> Index<Pnt3<u32>> for GenUnit3<N> {
    type Output = N;
    fn index(&self, index: Pnt3<u32>) -> &N {
        let idx = self.to_idx(index);
        &self.data[idx]
    }
}
impl<N> IndexMut<Pnt3<u32>> for GenUnit3<N> {
    fn index_mut(&mut self, index: Pnt3<u32>) -> &mut N {
        let idx = self.to_idx(index);
        &mut self.data[idx]
    }
}

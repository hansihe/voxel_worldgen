use ::nalgebra::{ Vec3, Pnt3, Vec2, Pnt2, cast };
use std::ops::{ Index, IndexMut };
use ::rnd::OctavesSeed;

// Make sure to use GenUnit3 for 3d, GenUnit2 for 2d.
// This would simplify making them separate structs in the 
// future if that is needed.
pub type GenUnit3<N> = GenUnit<N>;
pub type GenUnit2<N> = GenUnit<N>;

pub struct GenUnit<N> {
    pub size: Vec3<u32>,
    pub data: Vec<N>,
}

impl<N: Sized> GenUnit<N> {
    pub fn new3(size: Vec3<u32>, default: N) -> GenUnit<N> where N: Clone {
        let capacity: usize = cast(size.x * size.y * size.z);
        GenUnit {
            size: size,
            data: vec![default; capacity],
        }
    }
    pub fn new2(size: Vec2<u32>, default: N) -> GenUnit<N> where N: Clone {
        GenUnit::new3(Vec3::new(size.x, 1, size.y), default)
    }

    pub fn new2_from_vec(size: Vec2<u32>, data: Vec<N>) -> GenUnit<N> {
        let capacity: usize = cast(size.x * size.y);
        assert!(capacity == data.len());
        GenUnit {
            size: Vec3::new(size.x, 1, size.y),
            data: data,
        }
    }
    pub fn new3_from_vec(size: Vec3<u32>, data: Vec<N>) -> GenUnit<N> {
        let capacity: usize = cast(size.x * size.y * size.z);
        assert!(capacity == data.len());
        GenUnit {
            size: size,
            data: data,
        }
    }

    pub fn gen_simplex(seed: &OctavesSeed, pos: Pnt3<f64>, size: Vec3<u32>,
                   scale: Vec3<f64>, mult: f64) -> GenUnit<f64> {
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

        GenUnit {
            size: size,
            data: data,
        }
    }

    pub fn to_idx_vec3(&self, pos: Pnt3<u32>) -> usize {
        self.to_idx(pos.x, pos.y, pos.z)
    }
    pub fn to_idx_vec2(&self, pos: Pnt2<u32>) -> usize {
        self.to_idx(pos.x, 0, pos.y)
    }
    pub fn to_idx(&self, x: u32, y: u32, z: u32) -> usize {
        debug_assert!(x < self.size.x);
        debug_assert!(y < self.size.y);
        debug_assert!(z < self.size.z);

        (x + (z*self.size.x) + (y*self.size.x*self.size.z)) as usize
    }

    pub fn from_idx(&self, mut idx: usize) -> Pnt3<u32> {
        let x = idx % self.size.x as usize;
        idx /= self.size.x as usize;
        let z = idx % self.size.z as usize;
        idx /= self.size.z as usize;
        let y = idx;
        Pnt3::new(x as u32, y as u32, z as u32)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn size2(&self) -> Vec2<u32> {
        Vec2::new(self.size.x, self.size.z)
    }
}

impl<N> Index<usize> for GenUnit<N> {
    type Output = N;
    fn index(&self, index: usize) -> &N { &self.data[index] }
}
impl<N> IndexMut<usize> for GenUnit<N> {
    fn index_mut(&mut self, index: usize) -> &mut N { &mut self.data[index] }
}

impl<N> Index<Pnt3<u32>> for GenUnit<N> {
    type Output = N;
    fn index(&self, index: Pnt3<u32>) -> &N {
        let idx = self.to_idx_vec3(index);
        &self.data[idx]
    }
}
impl<N> IndexMut<Pnt3<u32>> for GenUnit<N> {
    fn index_mut(&mut self, index: Pnt3<u32>) -> &mut N {
        let idx = self.to_idx_vec3(index);
        &mut self.data[idx]
    }
}
impl<N> Index<Pnt2<u32>> for GenUnit<N> {
    type Output = N;
    fn index(&self, index: Pnt2<u32>) -> &N {
        let idx = self.to_idx_vec2(index);
        &self.data[idx]
    }
}
impl<N> IndexMut<Pnt2<u32>> for GenUnit<N> {
    fn index_mut(&mut self, index: Pnt2<u32>) -> &mut N {
        let idx = self.to_idx_vec2(index);
        &mut self.data[idx]
    }
}

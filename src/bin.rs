//extern crate image;
extern crate noise;
extern crate rand;
//extern crate time;
extern crate num;
extern crate nalgebra;

//use image::{ ImageBuffer };
use noise::{Seed, open_simplex2};

mod simplex_normalized;
use simplex_normalized::normalize_simplex;

mod layer;
use layer::GenL;
use layer::{ LayerLCG };
use layer::{ GenLayer, GenReduceOcean, GenIsland, GenZoom, ZoomType, GenSimpleFn };
use layer::{ GenSimplex, SimplexNoiseType };
use layer::{ GenSimpleFnMixer };

mod generators;

mod rnd;
use rnd::{ OctavesSeed, simplex3_octaves };
use rand::{ XorShiftRng, random, StdRng };

mod gen;

use nalgebra::{ Vec2, Pnt2 };

//mod geometry;

fn main() {
    /*let mut lcg = LayerLCG::new(12);
    lcg.seed_world(82);
    lcg.seed_pos(23, 92);

    for _ in 0..100 {
        println!("{:?}", lcg.next_int(10));
    }*/

    /*let mut src: Box<GenLayer> = GenIsland::new(1, 2);
    src = GenZoom::new(2, ZoomType::FUZZY, src);
    src = GenZoom::new(3, ZoomType::MAJORITY, src);
    let mut src1: Box<GenLayer> = GenIsland::new(2, 6);
    src = GenBlend::new(BlendMode::Or, src1, src);
    src = GenZoom::new(4, ZoomType::MAJORITY, src);
    src = GenReduceOcean::new(6, 2, src);
    let mut dst = GenZoom::new(5, ZoomType::MAJORITY, src);*/

    //divide(1000, 20_000_000);
    
//    let seed = Seed::new(1);
//    //let mut rng = XorShiftRng::new_unseeded();
//    //let mut rng: XorShiftRng = random();
//    let mut rng = StdRng::new().unwrap();
//    let octaves = OctavesSeed::new(&mut rng, 16);
//
//    let world_gen_state = generators::vanilla::WorldGeneratorState::new(&mut rng);
//    //let block_array = generators::vanilla::lerp_height_field(&world_gen_state, &[0; 81], &[72, 28], &[5, 5]);
//    
//    //let block_array = generators::vanilla::test_generate_chunk(&[20, 82]);
//    //println!("{:?}", block_array);
//    
//    //println!("{:?}, length: {:?}", height_field, height_field.len());
//    
//    let gen = generators::vanilla::biomes::land_mask();
//    let buf = gen.gen(10, Pnt2::new(-510, -510), Vec2::new(1026, 1026));
//
//    //println!("start");
//    //let start = time::precise_time_ns();
//    //let mut dst = test();
//    //let buf = dst.gen(10, 0, 0, 1024, 1024);
//    //let end = time::precise_time_ns();
//    //println!("end {:?} {:?}", buf[3432], end - start);
//
//    let img = ImageBuffer::from_fn(1024, 1024, |x, y| {
//        if buf[(x + y * 1026) as usize] {
//            image::Luma([255])
//        } else {
//            image::Luma([0])
//        }
//        //let num = buf[(x + y * 1024) as usize];
//        //let (height, var) = generators::vanilla::height_field::sample_biome_range(
//        //    &buf, &[1024, 1024], &[x, y]);
//        //image::Rgb([(num / 8) * 16, (num % 8) * 16, 0])
//        //image::Rgb([num*16, tum*16, 0])
//        /*let val = if x < 512 {
//            normalize_simplex(open_simplex2(&seed, &[x as f32 / 32.0, y as f32 / 32.0]))
//        } else {
//            open_simplex2(&seed, &[x as f32 / 32.0, y as f32 / 32.0])
//        };
//        let val_int = (((val + 1.0) / 2.0) * 255.0) as u8;
//        image::Luma([val_int])*/
//
//        //let i = simplex3_octaves(&octaves, &[x as f64 / 64.0, y as f64 / 64.0, 10.0]);
//        //image::Luma([((i + 1.0) * 100.0) as u8])
//        //image::Luma([((var + 100.0) * 100.0) as u8])
//        //let val = buf[(x + y * 1026) as usize];
//        //image::Luma([val * 100])
//    });
//
//    img.save("test.png").unwrap();


}

mod utils;

use array2d::Array2D;
use rand::Rng;
use wasm_bindgen::prelude::*;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct Unit {
    x: i32,
    y: i32,
    genome: Vec<i32>,
}

pub struct Population {
    units: Vec<Unit>,
    genome_size: i32,
    map: Array2D<i8>,
    map_size: usize,
    mut_rate: f64,
}

impl Population {
    fn new(population_size: i32, genome_size: i32, map_size: usize, mut_rate: f64) -> Population {
        let mut rng = rand::thread_rng();
        let mut units = Vec::new();
        for _ in 0..population_size {
            let mut genome: Vec<i32> = Vec::new();
            for _ in 0..genome_size {
                genome.push(rng.gen_range(0..=3));
            }
            units.push(Unit { x: 0, y: 0, genome })
        }
        let mut map = Array2D::filled_with(0, map_size, map_size);

        let mut i = 0;
        for u in units.iter_mut() {
            u.x = 0;
            u.y = i;

            map.set(u.x as usize, u.y as usize, 1);

            i += 1;
        }

        Population {
            units,
            genome_size,
            map,
            map_size,
            mut_rate,
        }
    }
    fn reset(&mut self, x: i32, y: i32) {
        for u in self.units.iter_mut() {
            u.x = x;
            u.y = y;
        }
    }

    fn step(&mut self, num: usize) {
        let mut standing = Vec::new();
        let mut prev_len = 0;
        for u in self.units.iter_mut() {
            let (vx, vy) = match u.genome[num] {
                0 => (-1, 0),
                1 => (0, -1),
                2 => (1, 0),
                3 => (0, 1),
                _ => panic!(),
            };

            let nx = u.x + vx;
            let ny = u.y + vy;

            if nx < 0 || nx as usize >= self.map_size || ny < 0 || ny as usize >= self.map_size {
                continue;
            }
            match self.map.get(nx as usize, ny as usize) {
                None => continue,
                Some(1) => standing.push(u),
                Some(0) => {
                    u.x = nx;
                    u.y = ny;
                }
                Some(_) => panic!(),
            }
        }
    }

    fn crossover(&self, parent_a: &Unit, parent_b: &Unit) -> Unit {
        let mut rng = rand::thread_rng();
        let mid = rng.gen_range(0..parent_a.genome.len());
        let mut genome = Vec::new();
        for i in 0..mid {
            genome.push(parent_a.genome[i]);
        }
        for i in mid..parent_b.genome.len() {
            genome.push(parent_b.genome[i]);
        }

        Unit { x: 0, y: 0, genome }
    }

    fn mutate(&self, unit: &mut Unit) {
        let mut rng = rand::thread_rng();

        for gene in unit.genome.iter_mut() {
            if rng.gen_range(0.0..=1.0) <= self.mut_rate {
                *gene = rng.gen_range(0..=3);
            }
        }
    }
}

#[wasm_bindgen]
pub fn find_path(
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
    genome_size: i32,
    map_size: usize,
    population_size: i32,
    num_of_evolutions: usize,
    mut_rate: f64,
    walls_x: Vec<usize>,
    walls_y: Vec<usize>,
) -> Vec<i32> {
    utils::set_panic_hook();
    // alert("Hello, pathfinder-wasm!");
    let destx = to_x;
    let detyy = to_y;
    let mut population = Population::new(population_size, genome_size, map_size, mut_rate);
    for i in 0..walls_x.len() {
        population.map.set(walls_x[i], walls_y[i], 1);
    }
    for _ in 0..num_of_evolutions {
        population.reset(from_x, from_y);
        for i in 0..population.genome_size {
            population.step(i as usize);
        }
        // Calculate cost for each unit
        let mut fits = Vec::new();
        let mut fits_sum = 0.0;
        for u in population.units.iter() {
            let fit = 100.0 / (((u.x - destx).pow(2) + (u.y - detyy).pow(2)) as f64 + 1.0).sqrt();
            fits.push((u, fit));

            fits_sum += fit;
        }
        // log!("fit: {}", fits_sum);
        // log!("x, y: {} {}", population.units[0].x, population.units[0].y);
        // Select new population using costs and crossover
        let mut rng = rand::thread_rng();
        let mut new_population = Vec::new();
        for _ in 0..population_size {
            // log!("{}", fits_sum);
            let mut r1 = rng.gen_range(0.0..fits_sum);
            let mut r2 = rng.gen_range(0.0..fits_sum);
            let mut p1 = None;
            let mut p2 = None;

            for p in fits.iter() {
                let (u, fit) = p;
                // log!("r1 {} {}", cost, r1);
                if *fit + 0.0001 >= r1 {
                    p1 = Some(*u);
                    break;
                }
                r1 -= *fit;
            }
            for p in fits.iter() {
                let (u, cost) = p;
                // log!("r2 {} {}", cost, r2);
                if *cost + 0.0001 >= r2 {
                    p2 = Some(*u);
                    break;
                }
                r2 -= *cost;
            }
            // log!("{} {}", population.units[0].x, population.units[0].y);
            match (p1, p2) {
                // (Some(sp1), Some(sp2)) => new_population.push(population.crossover(sp1, sp2)),
                (Some(sp1), Some(sp2)) => {
                    let mut new_unit = population.crossover(sp1, sp2);
                    population.mutate(&mut new_unit);
                    new_population.push(new_unit)
                }
                _ => panic!(),
            }
        }
        population.units = new_population;
    }
    population.units[0].genome.clone()
}

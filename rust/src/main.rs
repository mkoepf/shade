// TODO load config

// TODO ut in rust

// TODO file io

// TODO docker

use std::f64;

fn main() {
    let n = 256;
    let l = 2.0 * f64::consts::PI;

    let dx = l / (f64::from(n));

    let x : Vec<f64> = (1..n).map(|i| (f64::from(i)) * dx ).collect();

    let f : Vec<f64> = x.iter().map(|xi| xi.sin()).collect();


    for (cur_x, cur_f) in x.iter().zip(f) {
        println!("{};{} ", cur_x, cur_f);
    }


}

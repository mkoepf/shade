// TODO load config

// TODO ut in rust

// TODO file io

// TODO docker

// https://docs.rs/ode_solvers/0.3.0/ode_solvers/

use std::f64;

fn riemann_integral(f: &Vec<f64>) -> f64 {
    f.iter().sum()
}

fn main() {
    let n = 256;
    let l = 2.0 * f64::consts::PI;

    let dx = l / (f64::from(n));

    let x : Vec<f64> = (1..n).map(|i| (f64::from(i)) * dx ).collect();

    let f : Vec<f64> = x.iter().map(|xi| xi.sin()).collect();

    for (cur_x, cur_f) in x.iter().zip(f.iter()) {
        println!("{};{} ", cur_x, cur_f);
    }

    println!("Riemann integral of f: {}", riemann_integral(&f))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_riemann_integral() {
        let f = vec![0.0,0.0,0.0];
        assert_eq!(riemann_integral(&f), 0.0);
    }
}

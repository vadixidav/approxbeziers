extern crate clap;
use clap::{App, Arg};
use std::fmt;

use std::f64::consts::PI;

struct QBezier {
    points: [[f64; 2]; 3],
}

impl fmt::Display for QBezier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "[{{{}, {}}}, {{{}, {}}}, {{{}, {}}}]",
               self.points[0][0],
               self.points[0][1],
               self.points[1][0],
               self.points[1][1],
               self.points[2][0],
               self.points[2][1])
    }
}

fn main() {
    let matches = App::new("approxbeziers")
        .version("1.0")
        .author("Geordon Worley <vadixidav@gmail.com>")
        .about("Approximates shapes with quadratic bezier curves.")
        .arg(Arg::with_name("shape")
            .long("shape")
            .short("s")
            .takes_value(true)
            .possible_values(&["circle"])
            .default_value("circle")
            .help("The shape to approximate."))
        .arg(Arg::with_name("curves")
            .help("Sets the amount of curves to use.")
            .required(true)
            .index(1))
        .get_matches();

    let curves = match matches.value_of("curves") {
        Some(c) => {
            match c.parse::<usize>() {
                Ok(n) => n,
                Err(e) => {
                    println!("Error: Failed to parse curves: {}", e);
                    return;
                }
            }
        }
        None => unreachable!(),
    };

    match matches.value_of("shape").unwrap() {
        "circle" => {
            if curves < 2 {
                panic!("Must have at least 2 curves to make a circle.");
            }
            let a = 2.0 * PI / (2.0 * curves as f64);
            let outer_radius = 1.0 / (a.cos());

            let isolver = |i: usize| {
                if i % 2 == 0 {
                    [((2 * i) as f64 * a).sin(), -((2 * i) as f64 * a).cos()]
                } else {
                    [outer_radius * ((2 * i + 1) as f64 * a).sin(),
                     -outer_radius * ((2 * i + 1) as f64 * a).cos()]
                }
            };

            for curve in 0..curves {
                let base = curve * 2;

                println!("{}",
                         QBezier { points: [isolver(base), isolver(base + 1), isolver(base + 2)] });
            }
        }
        v => panic!("Error: \"{}\" is not a valid shape.", v),
    };
}

// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
//#![ allow( dead_code, unused_imports, unused_variables ) ]
#[macro_use] extern crate lazy_static;
extern crate mod_exp;
extern crate num_integer;
extern crate permutohedron;
extern crate regex;

use std::env;

macro_rules! regex_captures {
  ($re:tt, $s:expr) => {
    $re.captures($s).ok_or_else(|| format!("`{}` did not match `{}`", $s, $re.as_str()))
  };
}

macro_rules! capture_group {
  ($caps:expr, $group:expr) => { $caps.get($group).expect("valid capture group").as_str() };
}

#[allow(unused_macros)]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

// https://stackoverflow.com/a/56663823/113632
#[cfg(test)]
macro_rules! parameterized_test {
    ($name:ident, $args:pat, $body:tt) => {
        with_dollar_sign! {
            ($d:tt) => {
                macro_rules! $name {
                    ($d($d pname:ident: $d values:expr,)*) => {
                        mod $name {
                            use super::*;
                            $d(
                                #[test]
                                fn $d pname() {
                                    let $args = $d values;
                                    $body
                                }
                            )*
                        }}}}}}}

mod error;
mod euclid;
mod euclid3d;
mod intcode;
mod pathfinding;

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc19;
mod aoc20;
mod aoc22;
mod aoc23;
mod aoc24;

fn main() {
    println!(); // split build output from runtime output
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} DAY_OF_ADVENT", args[0]);
        return;
    }
    let day: u32 = args[1].parse().expect("Should be a natural number");
    match day {
        1 => aoc1::advent(),
        2 => aoc2::advent(),
        3 => aoc3::advent(),
        4 => aoc4::advent(),
        5 => aoc5::advent(),
        6 => aoc6::advent(),
        7 => aoc7::advent(),
        8 => aoc8::advent(),
        9 => aoc9::advent(),
        10 => aoc10::advent(),
        11 => aoc11::advent(),
        12 => aoc12::advent(),
        13 => aoc13::advent(),
        14 => aoc14::advent(),
        15 => aoc15::advent(),
        16 => aoc16::advent(),
        17 => aoc17::advent(),
        19 => aoc19::advent(),
        20 => aoc20::advent(),
        22 => aoc22::advent(),
        23 => aoc23::advent(),
        24 => aoc24::advent(),
        x => {
            eprintln!("Day {} hasn't happened yet.", x);
            ::std::process::exit(1);
        },

    }
}

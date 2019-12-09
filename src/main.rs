// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
//#![ allow( dead_code, unused_imports, unused_variables ) ]
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate permutohedron;

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
mod intcode;

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;

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
        x => {
            eprintln!("Day {} hasn't happened yet.", x);
            ::std::process::exit(1);
        },

    }
}

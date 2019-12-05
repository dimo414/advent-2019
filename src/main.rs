// https://github.com/rust-lang/cargo/issues/3591#issuecomment-475701083
//#![ allow( dead_code, unused_imports, unused_variables ) ]

use std::env;

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

mod intcode;

mod aoc1;
mod aoc2;

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
        x => {
            eprintln!("Day {} hasn't happened yet.", x);
            ::std::process::exit(1);
        },

    }
}

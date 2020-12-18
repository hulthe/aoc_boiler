extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::fs;
use std::io;
use syn::Ident;

/// A very dirty proc-macro for automatically generating a boilerplate cli solver for AoC
#[proc_macro]
pub fn generate_days(_tokens: TokenStream) -> TokenStream {
    let days = get_aoc_dirs().expect("failed to scan for aoc dirs");

    let mut modules = quote! {};
    let mut opts = quote! {};
    let mut async_run_calls = quote! {};
    let mut run_calls = quote! {};
    let mut benches = quote! {};

    for day in days {
        let day_num: u8 = day.trim_start_matches("day").parse().unwrap();
        let day = Ident::new(&day, Span::call_site());
        let day_pascal = day.clone(); // TODO

        modules = quote! {
            #modules
            mod #day;
        };

        opts = quote! {
            #opts

            /// Compute the solutions for day 03
            #day_pascal {
                #[structopt(subcommand)]
                part: Option<Part>,
            },
        };

        async_run_calls = quote! {
            #async_run_calls
            run_day_async(YEAR, #day_num, #day::part1, #day::part2, &mp);
        };

        run_calls = quote! {
            #run_calls
            Opt::#day_pascal { part } => run_day(YEAR, #day_num, #day::part1, #day::part2, part)?,
        };

        benches = quote! {
            #benches

            mod #day {
                use crate::{YEAR, #day};
                use test::{black_box, Bencher};

                #[bench]
                fn parse(b: &mut Bencher) {
                    let input = aoc_helpers::get_input(YEAR, #day_num).unwrap();
                    b.iter(|| #day::parse(black_box(&input)));
                }

                #[bench]
                fn part1(b: &mut Bencher) {
                    let input = aoc_helpers::get_input(YEAR, #day_num).unwrap();
                    b.iter(|| #day::part1(black_box(&input)));
                }

                #[bench]
                fn part2(b: &mut Bencher) {
                    let input = aoc_helpers::get_input(YEAR, #day_num).unwrap();
                    b.iter(|| #day::part2(black_box(&input)));
                }
            }
        };
    }

    let out = quote! {
        #modules

        use structopt::StructOpt;
        use aoc_helpers::{get_input};
        use aoc_helpers::helpers::{Part, run_day, run_day_async};
        use std::error::Error;
        use std::fmt::Display;
        use std::thread::{self, JoinHandle};
        use std::time::Duration;
        use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

        #[derive(StructOpt, Debug)]
        #[structopt(name = "basic")]
        enum Opt {
            /// Compute all solutions
            All,

            #opts
        }

        fn run_all_async() -> Result<(), Box<dyn Error>> {
            let mp = MultiProgress::new();

            #async_run_calls

            mp.set_move_cursor(true);
            mp.join()?;

            Ok(())
        }

        fn main() -> Result<(), Box<dyn Error>> {
            let opt = Opt::from_args();

            match opt {
                Opt::All => run_all_async()?,
                #run_calls
            }

            Ok(())
        }

        #[cfg(test)]
        mod benches {
            #benches
        }
    };

    out.into()
}

fn get_aoc_dirs() -> io::Result<Vec<String>> {
    let mut dirs = vec![];

    for entry in fs::read_dir("src")? {
        let entry = entry?;

        if !entry.path().is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_string_lossy();

        if let Some(num) = name.strip_prefix("day") {
            if num.chars().all(|c| char::is_ascii_digit(&c)) {
                dirs.push(name.to_string())
            }
        }
    }

    dirs.sort();

    Ok(dirs)
}

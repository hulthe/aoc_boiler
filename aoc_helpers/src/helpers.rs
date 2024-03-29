use crate::input::get_input;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::error::Error;
use std::fmt::Display;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

#[derive(Parser, Debug)]
pub enum Part {
    /// Run only part 1 of the day
    One,

    /// Run only part 2 of the day
    Two,
}

pub fn run_day<P1, O1, P2, O2>(
    year: u32,
    day: u8,
    p1: P1,
    p2: P2,
    part: Option<Part>,
) -> Result<(), Box<dyn Error>>
where
    P1: Fn(&str) -> O1,
    O1: Display,
    P2: Fn(&str) -> O2,
    O2: Display,
{
    let input = get_input(year, day)?;

    if let None | Some(Part::One) = part {
        let out = p1(&input).to_string();
        if out.contains('\n') {
            println!("part 1:\n{out}");
        } else {
            println!("part 1: {out}");
        }
    }
    if let None | Some(Part::Two) = part {
        let out = p2(&input).to_string();
        if out.contains('\n') {
            println!("part 2:\n{out}");
        } else {
            println!("part 2: {out}");
        }
    }

    Ok(())
}

pub fn run_day_async<P1, O1, P2, O2>(
    year: u32,
    day: u8,
    p1: P1,
    p2: P2,
    mp: &MultiProgress,
) -> JoinHandle<()>
where
    P1: Fn(&str) -> O1 + Send + 'static,
    O1: Display,
    P2: Fn(&str) -> O2 + Send + 'static,
    O2: Display,
{
    fn msg_is_slim(msg: &str) -> bool {
        msg.len() <= 24 && !msg.contains('\n')
    }

    fn hide_long(s: &str) -> &str {
        if msg_is_slim(s) {
            s
        } else {
            "(...)"
        }
    }

    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        //.tick_chars("|/-\\ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    let pb = mp.add(ProgressBar::new_spinner());
    pb.set_style(spinner_style);
    pb.set_prefix(format!("day{:02}", day));
    pb.enable_steady_tick(75);
    let pb = Arc::new(pb);

    thread::spawn(move || {
        let run = || -> Result<(), Box<dyn Error>> {
            pb.set_message("Fetching Data...");
            let input: String = get_input(year, day)?;

            pb.set_message("Calculating part 1...");
            let solution_1 = p1(&input).to_string();

            pb.set_message("Calculating part 2...");
            let solution_2 = p2(&input).to_string();

            pb.finish_with_message(format!(
                "part 1: {:24}   part 2: {}",
                hide_long(&solution_1),
                hide_long(&solution_2)
            ));
            Ok(())
        };

        if let Err(e) = run() {
            pb.finish_with_message(format!("Error: {}", e));
        }
    })
}

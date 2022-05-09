mod game;

use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};

use clap::Parser;

use crate::game::Position;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    log: String,
}

use vampirc_uci::{self as uci, UciMessage};

#[derive(Default, Debug)]
struct Options {
    hash: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut log = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&args.log)
        .unwrap();

    let mut options = Options::default();

    let mut position = Position::default();

    let stdin = io::stdin();

    // UCI specification: https://backscattering.de/chess/uci/

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                for msg in uci::parse_with_unknown(&line) {
                    writeln!(log, "{:?}", msg).unwrap();

                    match msg {
                        UciMessage::Uci => {
                            println!(
                                "id name Khan \n\
                                 id author the Khan developers (see AUTHORS file)\n\
                                 option name Hash type spin default 1 min 1 max 128\n\
                                 uciok"
                            );
                        }
                        UciMessage::SetOption { name, value } => match name.as_str() {
                            "Hash" => {
                                options.hash =
                                    match value.as_ref().and_then(|x| x.parse::<usize>().ok()) {
                                        Some(x) => Some(x),
                                        None => {
                                            writeln!(
                                                log,
                                                "Warning: Failed to parse hash value: {:?}",
                                                value
                                            );
                                            continue;
                                        }
                                    }
                            }
                            _ => {}
                        },
                        UciMessage::IsReady => {
                            println!("readyok")
                        }
                        UciMessage::Quit => return,
                        UciMessage::Position {
                            startpos,
                            fen,
                            moves,
                        } => {
                            if startpos {
                                position = Position::starting()
                            }
                            writeln!(log, "this is position: {:?}", position).unwrap();
                        }
                        UciMessage::UciNewGame => {
                            writeln!(log, "options: {:?}", options).unwrap();
                        }
                        UciMessage::Go {
                            time_control,
                            search_control,
                        } => {
                            println!("bestmove d2d4");
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                println!("info string Error occured: {:?}", err);
                return;
            }
        }
    }
}

mod game;

use std::io::{self, BufRead};

use clap::Parser;
use game::{Promotion, Square};

use crate::game::Position;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {}

use vampirc_uci::{self as uci, UciMessage};

#[derive(Default, Debug)]
struct Options {
    hash: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut options = Options::default();

    let mut position = Position::default();

    let stdin = io::stdin();

    // UCI specification: https://backscattering.de/chess/uci/

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                for msg in uci::parse_with_unknown(&line) {
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
                            for m in moves {
                                let from: Square = m.from.try_into().unwrap();
                                let to: Square = m.to.try_into().unwrap();
                                let promotion: Option<Promotion> =
                                    m.promotion.map(|x| x.try_into().unwrap());
                                position.ply(from, to, promotion).unwrap();
                            }
                        }
                        UciMessage::UciNewGame => {
                            println!("info string {:?}", options);
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

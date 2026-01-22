mod board;
mod clifford;
mod game;
mod graph;
mod logger;
mod position;
mod search;
mod symmetry;

use std::io::{self, BufRead, Write};

use clap::Parser;
use log::LevelFilter;

use crate::board::{PieceType, Square};
use crate::position::GeometricPosition;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    log_level: Option<String>,
}

use vampirc_uci::{self as uci, UciMessage};

#[derive(Default, Debug)]
struct Options {
    hash: Option<usize>,
}

fn main() {
    logger::init(LevelFilter::Trace);

    let _args = Args::parse();

    let mut position = GeometricPosition::starting();
    let mut options = Options::default();

    let stdin = io::stdin();

    // UCI specification: https://backscattering.de/chess/uci/

    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                for msg in uci::parse_with_unknown(&line) {
                    match msg {
                        UciMessage::Uci => {
                            println!("id name Khan");
                            println!("id author the Khan developers (see AUTHORS file)");
                            println!("option name Hash type spin default 1 min 1 max 128");
                            println!("uciok");
                            let _ = io::stdout().flush();
                        }
                        UciMessage::SetOption { name, value } => {
                            match name.as_str() {
                                "Hash" => {
                                    options.hash =
                                        value.as_ref().and_then(|x| x.parse::<usize>().ok());
                                }
                                _ => {}
                            }
                        }
                        UciMessage::IsReady => {
                            println!("readyok");
                            let _ = io::stdout().flush();
                        }
                        UciMessage::UciNewGame => {
                            position = GeometricPosition::starting();
                            log::info!("New game started, options: {:?}", options);
                        }
                        UciMessage::Position { startpos, fen: _, moves: halfmoves } => {
                            if startpos {
                                position = GeometricPosition::starting();
                            }

                            for m in halfmoves {
                                let from = Square::from_algebraic(&m.from.to_string()).unwrap();
                                let to = Square::from_algebraic(&m.to.to_string()).unwrap();
                                let promotion = m.promotion.map(|p| {
                                    use vampirc_uci::UciPiece;
                                    match p {
                                        UciPiece::Knight => PieceType::Knight,
                                        UciPiece::Bishop => PieceType::Bishop,
                                        UciPiece::Rook => PieceType::Rook,
                                        UciPiece::Queen => PieceType::Queen,
                                        _ => PieceType::Queen,
                                    }
                                });

                                position.make_move(from, to, promotion);
                            }
                        }
                        UciMessage::Go { .. } => {
                            // Use alpha-beta search to find best move
                            if let Some(best_move) = search::search(&mut position) {
                                println!("bestmove {}", best_move.to_uci());
                            } else {
                                log::info!("No legal moves available");
                                println!("bestmove 0000");
                            }
                            io::stdout().flush().unwrap();
                        }
                        UciMessage::Quit => {
                            log::info!("Goodbye!");
                            return;
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                log::error!("{:?}", err);
                return;
            }
        }
    }
}

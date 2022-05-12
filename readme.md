# khan

A UCI chess engine experiment in Rust

It uses [`vampric_uci`] for parsing messages from a chess interfaces. I've been testing with [CuteChess] and [Scid vs. PC](http://scidvspc.sourceforge.net/) (open to suggestions for a good chess interface).

AI is WIP. Currently it plays d4 then loses by illegal move :D

[`vampric_uci`]: https://github.com/vampirc/vampirc-uci
[CuteChess]: https://github.com/cutechess/cutechess
[Scid vs. PC]: http://scidvspc.sourceforge.net/
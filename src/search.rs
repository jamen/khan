use crate::board::{Color, Piece, PieceType, Square};
use crate::position::{GeometricPosition, Move};

/// Search parameters
const MAX_DEPTH: i32 = 5;
const MAX_QUIESCE_DEPTH: i32 = 6;

/// Lightweight position for fast search (no geometric structures)
#[derive(Clone)]
struct SearchPosition {
    pieces: Vec<Piece>,
    to_move: Color,
}

impl SearchPosition {
    fn from_geometric(pos: &GeometricPosition) -> Self {
        Self {
            pieces: pos.pieces.clone(),
            to_move: pos.to_move,
        }
    }

    fn make_move(&mut self, from: Square, to: Square, promotion: Option<PieceType>) {
        // Handle castling
        if let Some(piece) = self.pieces.iter().find(|p| p.square == from) {
            if piece.piece_type == PieceType::King {
                let file_diff = (to.file as i8 - from.file as i8).abs();
                if file_diff == 2 {
                    let (rook_from, rook_to) = if to.file > from.file {
                        (Square::new(7, from.rank), Square::new(5, from.rank))
                    } else {
                        (Square::new(0, from.rank), Square::new(3, from.rank))
                    };
                    if let Some(rook_idx) = self.pieces.iter().position(|p| p.square == rook_from) {
                        self.pieces[rook_idx].square = rook_to;
                    }
                }
            }
        }

        // Remove captured piece
        self.pieces.retain(|p| p.square != to);

        // Move piece
        if let Some(piece_idx) = self.pieces.iter().position(|p| p.square == from) {
            self.pieces[piece_idx].square = to;
            if let Some(promo) = promotion {
                self.pieces[piece_idx].piece_type = promo;
            }
        }

        self.to_move = self.to_move.opposite();
    }

    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in &self.pieces {
            if piece.color != self.to_move {
                continue;
            }
            match piece.piece_type {
                PieceType::Pawn => self.gen_pawn_moves(piece, &mut moves),
                PieceType::Knight => self.gen_knight_moves(piece, &mut moves),
                PieceType::Bishop => self.gen_bishop_moves(piece, &mut moves),
                PieceType::Rook => self.gen_rook_moves(piece, &mut moves),
                PieceType::Queen => self.gen_queen_moves(piece, &mut moves),
                PieceType::King => self.gen_king_moves(piece, &mut moves),
            }
        }
        moves
    }

    fn gen_pawn_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        let dir: i8 = if piece.color == Color::White { 1 } else { -1 };
        let start_rank = if piece.color == Color::White { 1 } else { 6 };
        let promo_rank = if piece.color == Color::White { 7 } else { 0 };

        // Single push
        let nr = sq.rank as i8 + dir;
        if nr >= 0 && nr < 8 {
            let to = Square::new(sq.file, nr as u8);
            if !self.is_occupied(to) {
                if to.rank == promo_rank {
                    for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves.push(Move::with_promotion(sq, to, promo));
                    }
                } else {
                    moves.push(Move::new(sq, to));
                    if sq.rank == start_rank {
                        let to2 = Square::new(sq.file, (sq.rank as i8 + 2 * dir) as u8);
                        if !self.is_occupied(to2) {
                            moves.push(Move::new(sq, to2));
                        }
                    }
                }
            }
        }

        // Captures
        for df in [-1i8, 1i8] {
            let nf = sq.file as i8 + df;
            let nr = sq.rank as i8 + dir;
            if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                let to = Square::new(nf as u8, nr as u8);
                if self.is_enemy(to, piece.color) {
                    if to.rank == promo_rank {
                        for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                            moves.push(Move::with_promotion(sq, to, promo));
                        }
                    } else {
                        moves.push(Move::new(sq, to));
                    }
                }
            }
        }
    }

    fn gen_knight_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        for (df, dr) in [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)] {
            let nf = sq.file as i8 + df;
            let nr = sq.rank as i8 + dr;
            if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                let to = Square::new(nf as u8, nr as u8);
                if !self.is_friendly(to, piece.color) {
                    moves.push(Move::new(sq, to));
                }
            }
        }
    }

    fn gen_bishop_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.gen_sliding(piece, moves, &[(1,1),(1,-1),(-1,1),(-1,-1)]);
    }

    fn gen_rook_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.gen_sliding(piece, moves, &[(0,1),(0,-1),(1,0),(-1,0)]);
    }

    fn gen_queen_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.gen_sliding(piece, moves, &[(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)]);
    }

    fn gen_king_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        for df in -1..=1i8 {
            for dr in -1..=1i8 {
                if df == 0 && dr == 0 { continue; }
                let nf = sq.file as i8 + df;
                let nr = sq.rank as i8 + dr;
                if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                    let to = Square::new(nf as u8, nr as u8);
                    if !self.is_friendly(to, piece.color) {
                        moves.push(Move::new(sq, to));
                    }
                }
            }
        }
    }

    fn gen_sliding(&self, piece: &Piece, moves: &mut Vec<Move>, dirs: &[(i8,i8)]) {
        let sq = piece.square;
        for &(df, dr) in dirs {
            let mut f = sq.file as i8 + df;
            let mut r = sq.rank as i8 + dr;
            while f >= 0 && f < 8 && r >= 0 && r < 8 {
                let to = Square::new(f as u8, r as u8);
                if self.is_friendly(to, piece.color) { break; }
                moves.push(Move::new(sq, to));
                if self.is_enemy(to, piece.color) { break; }
                f += df;
                r += dr;
            }
        }
    }

    fn is_occupied(&self, sq: Square) -> bool {
        self.pieces.iter().any(|p| p.square == sq)
    }

    fn is_friendly(&self, sq: Square, color: Color) -> bool {
        self.pieces.iter().any(|p| p.square == sq && p.color == color)
    }

    fn is_enemy(&self, sq: Square, color: Color) -> bool {
        self.pieces.iter().any(|p| p.square == sq && p.color != color)
    }

    fn find_king(&self, color: Color) -> Option<Square> {
        self.pieces.iter().find(|p| p.piece_type == PieceType::King && p.color == color).map(|p| p.square)
    }

    fn is_square_attacked_by(&self, sq: Square, attacker: Color) -> bool {
        for piece in &self.pieces {
            if piece.color != attacker { continue; }
            match piece.piece_type {
                PieceType::Pawn => {
                    let dir: i8 = if attacker == Color::White { 1 } else { -1 };
                    for df in [-1i8, 1i8] {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dir;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                            return true;
                        }
                    }
                }
                PieceType::Knight => {
                    for (df, dr) in [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)] {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dr;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                            return true;
                        }
                    }
                }
                PieceType::King => {
                    for df in -1..=1i8 {
                        for dr in -1..=1i8 {
                            if df == 0 && dr == 0 { continue; }
                            let af = piece.square.file as i8 + df;
                            let ar = piece.square.rank as i8 + dr;
                            if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                                return true;
                            }
                        }
                    }
                }
                PieceType::Rook => {
                    if self.slider_attacks(sq, piece.square, &[(0,1),(0,-1),(1,0),(-1,0)]) { return true; }
                }
                PieceType::Bishop => {
                    if self.slider_attacks(sq, piece.square, &[(1,1),(1,-1),(-1,1),(-1,-1)]) { return true; }
                }
                PieceType::Queen => {
                    if self.slider_attacks(sq, piece.square, &[(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)]) { return true; }
                }
            }
        }
        false
    }

    fn slider_attacks(&self, target: Square, from: Square, dirs: &[(i8,i8)]) -> bool {
        for &(df, dr) in dirs {
            let mut f = from.file as i8 + df;
            let mut r = from.rank as i8 + dr;
            while f >= 0 && f < 8 && r >= 0 && r < 8 {
                if f as u8 == target.file && r as u8 == target.rank { return true; }
                if self.pieces.iter().any(|p| p.square.file == f as u8 && p.square.rank == r as u8) { break; }
                f += df;
                r += dr;
            }
        }
        false
    }

    fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_sq) = self.find_king(color) {
            self.is_square_attacked_by(king_sq, color.opposite())
        } else {
            false
        }
    }

    fn generate_legal_moves(&self) -> Vec<Move> {
        let pseudo = self.generate_moves();
        let mut legal = Vec::new();
        for mv in pseudo {
            let mut test = self.clone();
            test.make_move(mv.from, mv.to, mv.promotion);
            if !test.is_in_check(self.to_move) {
                legal.push(mv);
            }
        }
        legal
    }

    /// Generate only capture moves (for quiescence search)
    fn generate_captures(&self) -> Vec<Move> {
        let mut captures = Vec::new();
        for mv in self.generate_moves() {
            // Check if this is a capture (piece on target square)
            if self.pieces.iter().any(|p| p.square == mv.to && p.color != self.to_move) {
                captures.push(mv);
            }
            // Also include promotions (even non-captures)
            if mv.promotion.is_some() {
                if !captures.contains(&mv) {
                    captures.push(mv);
                }
            }
        }
        captures
    }

    /// Generate legal capture moves
    fn generate_legal_captures(&self) -> Vec<Move> {
        let captures = self.generate_captures();
        let mut legal = Vec::new();
        for mv in captures {
            let mut test = self.clone();
            test.make_move(mv.from, mv.to, mv.promotion);
            if !test.is_in_check(self.to_move) {
                legal.push(mv);
            }
        }
        legal
    }

    /// Count attackers on a square
    fn count_attackers(&self, sq: Square, attacker_color: Color) -> i32 {
        let mut count = 0;
        for piece in &self.pieces {
            if piece.color != attacker_color { continue; }
            let attacks = match piece.piece_type {
                PieceType::Pawn => {
                    let dir: i8 = if attacker_color == Color::White { 1 } else { -1 };
                    let mut atks = false;
                    for df in [-1i8, 1i8] {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dir;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                            atks = true;
                        }
                    }
                    atks
                }
                PieceType::Knight => {
                    let mut atks = false;
                    for (df, dr) in [(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)] {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dr;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                            atks = true;
                        }
                    }
                    atks
                }
                PieceType::King => {
                    let mut atks = false;
                    for df in -1..=1i8 {
                        for dr in -1..=1i8 {
                            if df == 0 && dr == 0 { continue; }
                            let af = piece.square.file as i8 + df;
                            let ar = piece.square.rank as i8 + dr;
                            if af >= 0 && af < 8 && ar >= 0 && ar < 8 && sq.file == af as u8 && sq.rank == ar as u8 {
                                atks = true;
                            }
                        }
                    }
                    atks
                }
                PieceType::Rook => self.slider_attacks(sq, piece.square, &[(0,1),(0,-1),(1,0),(-1,0)]),
                PieceType::Bishop => self.slider_attacks(sq, piece.square, &[(1,1),(1,-1),(-1,1),(-1,-1)]),
                PieceType::Queen => self.slider_attacks(sq, piece.square, &[(0,1),(0,-1),(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1)]),
            };
            if attacks { count += 1; }
        }
        count
    }
}
const INFINITY: f32 = 100000.0;

/// Piece values for MVV-LVA move ordering
fn piece_value(pt: PieceType) -> i32 {
    match pt {
        PieceType::Pawn => 100,
        PieceType::Knight => 320,
        PieceType::Bishop => 330,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 20000,
    }
}

/// Piece-square tables for positional evaluation (from White's perspective)
/// Encourage pieces to go to good squares
const PAWN_TABLE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5,  5, 10, 25, 25, 10,  5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [5, 10, 10,-20,-20, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0],
];

const KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

const BISHOP_TABLE: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

const ROOK_TABLE: [[i32; 8]; 8] = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [0,  0,  0,  5,  5,  0,  0,  0],
];

const KING_TABLE: [[i32; 8]; 8] = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [20, 20,  0,  0,  0,  0, 20, 20],
    [20, 30, 10,  0,  0, 10, 30, 20],
];

/// Get piece-square value for a piece at a given square
fn piece_square_value(piece_type: PieceType, square: Square, color: Color) -> i32 {
    let (file, rank) = (square.file as usize, square.rank as usize);
    // For black, flip the rank to mirror the table
    let rank = if color == Color::White { rank } else { 7 - rank };

    match piece_type {
        PieceType::Pawn => PAWN_TABLE[7 - rank][file],
        PieceType::Knight => KNIGHT_TABLE[7 - rank][file],
        PieceType::Bishop => BISHOP_TABLE[7 - rank][file],
        PieceType::Rook => ROOK_TABLE[7 - rank][file],
        PieceType::Queen => 0, // Queen doesn't need special placement
        PieceType::King => KING_TABLE[7 - rank][file],
    }
}

/// Evaluate position from the perspective of the side to move
fn evaluate_search(pos: &SearchPosition) -> f32 {
    let mut score: i32 = 0;

    let mut white_king_sq = None;
    let mut black_king_sq = None;

    for piece in &pos.pieces {
        let material = piece_value(piece.piece_type);
        let positional = piece_square_value(piece.piece_type, piece.square, piece.color);
        let value = material + positional;

        if piece.color == Color::White {
            score += value;
            if piece.piece_type == PieceType::King {
                white_king_sq = Some(piece.square);
            }
        } else {
            score -= value;
            if piece.piece_type == PieceType::King {
                black_king_sq = Some(piece.square);
            }
        }
    }

    // King safety: penalize if king is attacked by multiple pieces
    if let Some(wk) = white_king_sq {
        let attackers = pos.count_attackers(wk, Color::Black);
        score -= attackers * 50; // Penalty for each attacker on white king
    }
    if let Some(bk) = black_king_sq {
        let attackers = pos.count_attackers(bk, Color::White);
        score += attackers * 50; // Bonus for attacking black king
    }

    let eval = score as f32 / 100.0;

    if pos.to_move == Color::White {
        eval
    } else {
        -eval
    }
}

/// Public evaluate for GeometricPosition
pub fn evaluate(pos: &GeometricPosition) -> f32 {
    let search_pos = SearchPosition::from_geometric(pos);
    evaluate_search(&search_pos)
}

/// Score a move for ordering (higher = search first)
fn score_move_search(pos: &SearchPosition, mv: &Move) -> i32 {
    let mut score = 0;

    // Capture bonus (MVV-LVA)
    if let Some(captured) = pos.pieces.iter().find(|p| p.square == mv.to) {
        let victim_value = piece_value(captured.piece_type);
        let attacker_value = pos.pieces.iter()
            .find(|p| p.square == mv.from)
            .map(|p| piece_value(p.piece_type))
            .unwrap_or(0);
        score += 10000 + victim_value - attacker_value / 100;
    }

    // Promotion bonus
    if let Some(promo) = mv.promotion {
        score += piece_value(promo);
    }

    // Center control bonus
    if let Some(piece) = pos.pieces.iter().find(|p| p.square == mv.from) {
        if piece.piece_type == PieceType::Pawn || piece.piece_type == PieceType::Knight {
            let center_dist = ((mv.to.file as i32 - 3).abs() + (mv.to.rank as i32 - 3).abs()) as i32;
            score += 10 - center_dist;
        }
    }

    score
}

/// Order moves for better alpha-beta pruning
fn order_moves_search(pos: &SearchPosition, moves: &mut [Move]) {
    moves.sort_by(|a, b| score_move_search(pos, b).cmp(&score_move_search(pos, a)));
}

/// Quiescence search - search captures until position is quiet
fn quiescence(pos: &SearchPosition, mut alpha: f32, beta: f32, depth: i32) -> f32 {
    // Stand-pat: evaluate current position
    let stand_pat = evaluate_search(pos);

    if depth <= 0 {
        return stand_pat;
    }

    if stand_pat >= beta {
        return beta;
    }

    if stand_pat > alpha {
        alpha = stand_pat;
    }

    // Search captures only
    let mut captures = pos.generate_legal_captures();
    if captures.is_empty() {
        return stand_pat;
    }

    order_moves_search(pos, &mut captures);

    for mv in captures {
        let mut new_pos = pos.clone();
        new_pos.make_move(mv.from, mv.to, mv.promotion);
        let score = -quiescence(&new_pos, -beta, -alpha, depth - 1);

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

/// Alpha-beta search using lightweight SearchPosition
fn alpha_beta(pos: &SearchPosition, depth: i32, mut alpha: f32, mut beta: f32) -> f32 {
    if depth == 0 {
        // Use quiescence search instead of static eval
        return quiescence(pos, alpha, beta, MAX_QUIESCE_DEPTH);
    }

    let mut moves = pos.generate_legal_moves();

    if moves.is_empty() {
        if pos.is_in_check(pos.to_move) {
            // Checkmate
            return -INFINITY + (MAX_DEPTH - depth) as f32;
        } else {
            // Stalemate
            return 0.0;
        }
    }

    order_moves_search(pos, &mut moves);

    let mut best_eval = -INFINITY;
    for mv in moves {
        let mut new_pos = pos.clone();
        new_pos.make_move(mv.from, mv.to, mv.promotion);
        let eval = -alpha_beta(&new_pos, depth - 1, -beta, -alpha);
        best_eval = best_eval.max(eval);
        alpha = alpha.max(eval);
        if alpha >= beta {
            break;
        }
    }
    best_eval
}

/// Find the best move using alpha-beta search
pub fn find_best_move(pos: &GeometricPosition, depth: i32) -> Option<Move> {
    let search_pos = SearchPosition::from_geometric(pos);
    let mut moves = search_pos.generate_legal_moves();

    if moves.is_empty() {
        return None;
    }

    order_moves_search(&search_pos, &mut moves);

    let mut best_move = moves[0];
    let mut best_eval = -INFINITY;

    for mv in &moves {
        let mut new_pos = search_pos.clone();
        new_pos.make_move(mv.from, mv.to, mv.promotion);
        let eval = -alpha_beta(&new_pos, depth - 1, -INFINITY, INFINITY);

        if eval > best_eval {
            best_eval = eval;
            best_move = *mv;
        }
    }

    log::info!("Best move: {} with eval {:.2}", best_move.to_uci(), best_eval);
    Some(best_move)
}

/// Search with iterative deepening and time limit
pub fn search(pos: &GeometricPosition) -> Option<Move> {
    // For now, just search at fixed depth
    find_best_move(pos, MAX_DEPTH)
}

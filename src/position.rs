use crate::board::{Color, Piece, PieceType, Square};
use crate::clifford::Multivector;
use crate::graph::CliffordAdjacencyMatrix;
use crate::symmetry::D4Element;

/// A chess move
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to, promotion: None }
    }

    pub fn with_promotion(from: Square, to: Square, promotion: PieceType) -> Self {
        Self { from, to, promotion: Some(promotion) }
    }

    /// Convert to UCI notation (e.g., "e2e4", "e7e8q")
    pub fn to_uci(&self) -> String {
        let mut s = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());
        if let Some(promo) = self.promotion {
            let c = match promo {
                PieceType::Queen => 'q',
                PieceType::Rook => 'r',
                PieceType::Bishop => 'b',
                PieceType::Knight => 'n',
                _ => 'q',
            };
            s.push(c);
        }
        s
    }
}

/// Complete chess position with geometric algebra representation
#[derive(Clone)]
pub struct GeometricPosition {
    pub pieces: Vec<Piece>,
    pub to_move: Color,
    pub position_multivector: Multivector,
    pub white_attack_graph: CliffordAdjacencyMatrix,
    pub black_attack_graph: CliffordAdjacencyMatrix,
    pub canonical_symmetry: Option<D4Element>,
}

impl GeometricPosition {
    /// Create from piece list
    pub fn from_pieces(pieces: Vec<Piece>, to_move: Color) -> Self {
        let mut pos = Self {
            pieces,
            to_move,
            position_multivector: Multivector::new(),
            white_attack_graph: CliffordAdjacencyMatrix::new(64),
            black_attack_graph: CliffordAdjacencyMatrix::new(64),
            canonical_symmetry: None,
        };
        pos.rebuild();
        pos
    }

    /// Create starting position
    pub fn starting() -> Self {
        let mut pieces = Vec::new();

        // White pieces - back rank
        pieces.push(Piece::new(PieceType::Rook, Color::White, Square::new(0, 0)));
        pieces.push(Piece::new(PieceType::Knight, Color::White, Square::new(1, 0)));
        pieces.push(Piece::new(PieceType::Bishop, Color::White, Square::new(2, 0)));
        pieces.push(Piece::new(PieceType::Queen, Color::White, Square::new(3, 0)));
        pieces.push(Piece::new(PieceType::King, Color::White, Square::new(4, 0)));
        pieces.push(Piece::new(PieceType::Bishop, Color::White, Square::new(5, 0)));
        pieces.push(Piece::new(PieceType::Knight, Color::White, Square::new(6, 0)));
        pieces.push(Piece::new(PieceType::Rook, Color::White, Square::new(7, 0)));

        // White pawns
        for file in 0..8 {
            pieces.push(Piece::new(PieceType::Pawn, Color::White, Square::new(file, 1)));
        }

        // Black pieces - back rank
        pieces.push(Piece::new(PieceType::Rook, Color::Black, Square::new(0, 7)));
        pieces.push(Piece::new(PieceType::Knight, Color::Black, Square::new(1, 7)));
        pieces.push(Piece::new(PieceType::Bishop, Color::Black, Square::new(2, 7)));
        pieces.push(Piece::new(PieceType::Queen, Color::Black, Square::new(3, 7)));
        pieces.push(Piece::new(PieceType::King, Color::Black, Square::new(4, 7)));
        pieces.push(Piece::new(PieceType::Bishop, Color::Black, Square::new(5, 7)));
        pieces.push(Piece::new(PieceType::Knight, Color::Black, Square::new(6, 7)));
        pieces.push(Piece::new(PieceType::Rook, Color::Black, Square::new(7, 7)));

        // Black pawns
        for file in 0..8 {
            pieces.push(Piece::new(PieceType::Pawn, Color::Black, Square::new(file, 6)));
        }

        Self::from_pieces(pieces, Color::White)
    }

    /// Rebuild derived structures after position change
    pub fn rebuild(&mut self) {
        // Rebuild position multivector
        self.position_multivector = Multivector::new();
        for piece in &self.pieces {
            let piece_mv = piece.to_multivector();
            self.position_multivector = self.position_multivector.add(&piece_mv);
        }

        // Rebuild attack graphs
        self.white_attack_graph = CliffordAdjacencyMatrix::new(64);
        self.black_attack_graph = CliffordAdjacencyMatrix::new(64);

        for piece in &self.pieces {
            let attacks = self.get_piece_attacks(piece);
            let graph = match piece.color {
                Color::White => &mut self.white_attack_graph,
                Color::Black => &mut self.black_attack_graph,
            };

            for target in attacks {
                graph.set_attack(piece.square.index(), target.index());
            }
        }
    }

    /// Get squares attacked by a piece
    fn get_piece_attacks(&self, piece: &Piece) -> Vec<Square> {
        let mut attacks = Vec::new();
        let sq = piece.square;

        match piece.piece_type {
            PieceType::Knight => {
                let offsets: [(i8, i8); 8] = [
                    (1, 2), (2, 1), (2, -1), (1, -2),
                    (-1, -2), (-2, -1), (-2, 1), (-1, 2),
                ];
                for (df, dr) in offsets {
                    let nf = sq.file as i8 + df;
                    let nr = sq.rank as i8 + dr;
                    if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                        attacks.push(Square::new(nf as u8, nr as u8));
                    }
                }
            }
            PieceType::King => {
                for df in -1..=1 {
                    for dr in -1..=1 {
                        if df == 0 && dr == 0 {
                            continue;
                        }
                        let nf = sq.file as i8 + df;
                        let nr = sq.rank as i8 + dr;
                        if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                            attacks.push(Square::new(nf as u8, nr as u8));
                        }
                    }
                }
            }
            PieceType::Pawn => {
                let dir = match piece.color {
                    Color::White => 1,
                    Color::Black => -1,
                };
                for df in [-1, 1] {
                    let nf = sq.file as i8 + df;
                    let nr = sq.rank as i8 + dir;
                    if nf >= 0 && nf < 8 && nr >= 0 && nr < 8 {
                        attacks.push(Square::new(nf as u8, nr as u8));
                    }
                }
            }
            PieceType::Rook => {
                self.add_sliding_attacks(&mut attacks, sq, &[(0, 1), (0, -1), (1, 0), (-1, 0)]);
            }
            PieceType::Bishop => {
                self.add_sliding_attacks(&mut attacks, sq, &[(1, 1), (1, -1), (-1, 1), (-1, -1)]);
            }
            PieceType::Queen => {
                self.add_sliding_attacks(&mut attacks, sq, &[
                    (0, 1), (0, -1), (1, 0), (-1, 0),
                    (1, 1), (1, -1), (-1, 1), (-1, -1),
                ]);
            }
        }

        attacks
    }

    /// Add sliding piece attacks (rook, bishop, queen)
    fn add_sliding_attacks(&self, attacks: &mut Vec<Square>, from: Square, directions: &[(i8, i8)]) {
        for &(df, dr) in directions {
            let mut f = from.file as i8 + df;
            let mut r = from.rank as i8 + dr;

            while f >= 0 && f < 8 && r >= 0 && r < 8 {
                let target = Square::new(f as u8, r as u8);
                attacks.push(target);

                // Stop if blocked by a piece
                if self.pieces.iter().any(|p| p.square == target) {
                    break;
                }

                f += df;
                r += dr;
            }
        }
    }

    /// Make a move
    pub fn make_move(&mut self, from: Square, to: Square, promotion: Option<PieceType>) {
        // Check for castling (king moves 2 squares)
        if let Some(piece) = self.pieces.iter().find(|p| p.square == from) {
            if piece.piece_type == PieceType::King {
                let file_diff = (to.file as i8 - from.file as i8).abs();
                if file_diff == 2 {
                    // This is castling - also move the rook
                    let (rook_from, rook_to) = if to.file > from.file {
                        // Kingside castling
                        (Square::new(7, from.rank), Square::new(5, from.rank))
                    } else {
                        // Queenside castling
                        (Square::new(0, from.rank), Square::new(3, from.rank))
                    };
                    // Move the rook
                    if let Some(rook_idx) = self.pieces.iter().position(|p| p.square == rook_from) {
                        self.pieces[rook_idx].square = rook_to;
                    }
                }
            }
        }

        // Remove captured piece first (if any)
        self.pieces.retain(|p| p.square != to);

        // Find and move piece (must find after retain since indices may have changed)
        if let Some(piece_idx) = self.pieces.iter().position(|p| p.square == from) {
            // Move piece
            self.pieces[piece_idx].square = to;

            // Handle promotion
            if let Some(promo) = promotion {
                self.pieces[piece_idx].piece_type = promo;
            }

            // Switch side
            self.to_move = self.to_move.opposite();

            // Rebuild derived structures
            self.rebuild();
        }
    }

    /// Generate all pseudo-legal moves for the side to move
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for piece in &self.pieces {
            if piece.color != self.to_move {
                continue;
            }

            match piece.piece_type {
                PieceType::Pawn => self.generate_pawn_moves(piece, &mut moves),
                PieceType::Knight => self.generate_knight_moves(piece, &mut moves),
                PieceType::Bishop => self.generate_bishop_moves(piece, &mut moves),
                PieceType::Rook => self.generate_rook_moves(piece, &mut moves),
                PieceType::Queen => self.generate_queen_moves(piece, &mut moves),
                PieceType::King => self.generate_king_moves(piece, &mut moves),
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        let dir: i8 = match piece.color {
            Color::White => 1,
            Color::Black => -1,
        };
        let start_rank = match piece.color {
            Color::White => 1,
            Color::Black => 6,
        };
        let promo_rank = match piece.color {
            Color::White => 7,
            Color::Black => 0,
        };

        // Single push
        let nr = sq.rank as i8 + dir;
        if nr >= 0 && nr < 8 {
            let to = Square::new(sq.file, nr as u8);
            if !self.is_occupied(to) {
                if to.rank == promo_rank {
                    // Promotion
                    for promo in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                        moves.push(Move::with_promotion(sq, to, promo));
                    }
                } else {
                    moves.push(Move::new(sq, to));

                    // Double push from starting rank
                    if sq.rank == start_rank {
                        let nr2 = sq.rank as i8 + 2 * dir;
                        let to2 = Square::new(sq.file, nr2 as u8);
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

    fn generate_knight_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        let offsets: [(i8, i8); 8] = [
            (1, 2), (2, 1), (2, -1), (1, -2),
            (-1, -2), (-2, -1), (-2, 1), (-1, 2),
        ];
        for (df, dr) in offsets {
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

    fn generate_bishop_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(piece, moves, &[(1, 1), (1, -1), (-1, 1), (-1, -1)]);
    }

    fn generate_rook_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(piece, moves, &[(0, 1), (0, -1), (1, 0), (-1, 0)]);
    }

    fn generate_queen_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        self.generate_sliding_moves(piece, moves, &[
            (0, 1), (0, -1), (1, 0), (-1, 0),
            (1, 1), (1, -1), (-1, 1), (-1, -1),
        ]);
    }

    fn generate_king_moves(&self, piece: &Piece, moves: &mut Vec<Move>) {
        let sq = piece.square;
        for df in -1..=1i8 {
            for dr in -1..=1i8 {
                if df == 0 && dr == 0 {
                    continue;
                }
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

    fn generate_sliding_moves(&self, piece: &Piece, moves: &mut Vec<Move>, directions: &[(i8, i8)]) {
        let sq = piece.square;
        for &(df, dr) in directions {
            let mut f = sq.file as i8 + df;
            let mut r = sq.rank as i8 + dr;

            while f >= 0 && f < 8 && r >= 0 && r < 8 {
                let to = Square::new(f as u8, r as u8);
                if self.is_friendly(to, piece.color) {
                    break; // Blocked by own piece
                }
                moves.push(Move::new(sq, to));
                if self.is_enemy(to, piece.color) {
                    break; // Can capture but not go further
                }
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

    /// Find the king of a given color
    fn find_king(&self, color: Color) -> Option<Square> {
        self.pieces
            .iter()
            .find(|p| p.piece_type == PieceType::King && p.color == color)
            .map(|p| p.square)
    }

    /// Check if a square is attacked by pieces of a given color
    fn is_square_attacked_by(&self, sq: Square, attacker_color: Color) -> bool {
        for piece in &self.pieces {
            if piece.color != attacker_color {
                continue;
            }

            match piece.piece_type {
                PieceType::Pawn => {
                    let dir: i8 = match attacker_color {
                        Color::White => 1,
                        Color::Black => -1,
                    };
                    // Pawns attack diagonally
                    for df in [-1i8, 1i8] {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dir;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 {
                            if sq.file == af as u8 && sq.rank == ar as u8 {
                                return true;
                            }
                        }
                    }
                }
                PieceType::Knight => {
                    let offsets: [(i8, i8); 8] = [
                        (1, 2), (2, 1), (2, -1), (1, -2),
                        (-1, -2), (-2, -1), (-2, 1), (-1, 2),
                    ];
                    for (df, dr) in offsets {
                        let af = piece.square.file as i8 + df;
                        let ar = piece.square.rank as i8 + dr;
                        if af >= 0 && af < 8 && ar >= 0 && ar < 8 {
                            if sq.file == af as u8 && sq.rank == ar as u8 {
                                return true;
                            }
                        }
                    }
                }
                PieceType::King => {
                    for df in -1..=1i8 {
                        for dr in -1..=1i8 {
                            if df == 0 && dr == 0 {
                                continue;
                            }
                            let af = piece.square.file as i8 + df;
                            let ar = piece.square.rank as i8 + dr;
                            if af >= 0 && af < 8 && ar >= 0 && ar < 8 {
                                if sq.file == af as u8 && sq.rank == ar as u8 {
                                    return true;
                                }
                            }
                        }
                    }
                }
                PieceType::Rook => {
                    if self.is_attacked_by_slider(sq, piece.square, &[(0, 1), (0, -1), (1, 0), (-1, 0)]) {
                        return true;
                    }
                }
                PieceType::Bishop => {
                    if self.is_attacked_by_slider(sq, piece.square, &[(1, 1), (1, -1), (-1, 1), (-1, -1)]) {
                        return true;
                    }
                }
                PieceType::Queen => {
                    if self.is_attacked_by_slider(sq, piece.square, &[
                        (0, 1), (0, -1), (1, 0), (-1, 0),
                        (1, 1), (1, -1), (-1, 1), (-1, -1),
                    ]) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Check if a slider piece attacks a target square
    fn is_attacked_by_slider(&self, target: Square, from: Square, directions: &[(i8, i8)]) -> bool {
        for &(df, dr) in directions {
            let mut f = from.file as i8 + df;
            let mut r = from.rank as i8 + dr;

            while f >= 0 && f < 8 && r >= 0 && r < 8 {
                if f as u8 == target.file && r as u8 == target.rank {
                    return true;
                }
                // Check if blocked by any piece
                if self.pieces.iter().any(|p| p.square.file == f as u8 && p.square.rank == r as u8) {
                    break;
                }
                f += df;
                r += dr;
            }
        }
        false
    }

    /// Check if the given color's king is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        if let Some(king_sq) = self.find_king(color) {
            self.is_square_attacked_by(king_sq, color.opposite())
        } else {
            false
        }
    }

    /// Generate only legal moves (filters out moves that leave king in check)
    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let pseudo_moves = self.generate_moves();
        let mut legal_moves = Vec::new();

        for mv in pseudo_moves {
            // Make the move on a clone
            let mut test_pos = self.clone();
            test_pos.make_move_no_rebuild(mv.from, mv.to, mv.promotion);

            // Check if our king is in check after the move
            if !test_pos.is_in_check(self.to_move) {
                legal_moves.push(mv);
            }
        }

        legal_moves
    }

    /// Make a move without rebuilding (for testing legality)
    fn make_move_no_rebuild(&mut self, from: Square, to: Square, promotion: Option<PieceType>) {
        // Check for castling (king moves 2 squares)
        if let Some(piece) = self.pieces.iter().find(|p| p.square == from) {
            if piece.piece_type == PieceType::King {
                let file_diff = (to.file as i8 - from.file as i8).abs();
                if file_diff == 2 {
                    // This is castling - also move the rook
                    let (rook_from, rook_to) = if to.file > from.file {
                        // Kingside castling
                        (Square::new(7, from.rank), Square::new(5, from.rank))
                    } else {
                        // Queenside castling
                        (Square::new(0, from.rank), Square::new(3, from.rank))
                    };
                    // Move the rook
                    if let Some(rook_idx) = self.pieces.iter().position(|p| p.square == rook_from) {
                        self.pieces[rook_idx].square = rook_to;
                    }
                }
            }
        }

        // Remove captured piece first (if any)
        self.pieces.retain(|p| p.square != to);

        // Find and move piece (must find after retain since indices may have changed)
        if let Some(piece_idx) = self.pieces.iter().position(|p| p.square == from) {
            // Move piece
            self.pieces[piece_idx].square = to;

            // Handle promotion
            if let Some(promo) = promotion {
                self.pieces[piece_idx].piece_type = promo;
            }

            // Switch side
            self.to_move = self.to_move.opposite();
        }
    }

    /// Count k-cycles in attack graph
    pub fn count_cycles(&self, color: Color, k: usize) -> f32 {
        let graph = match color {
            Color::White => &self.white_attack_graph,
            Color::Black => &self.black_attack_graph,
        };
        let powered = graph.power(k);
        powered.trace() / (k as f32)
    }

    /// Extract geometric features for evaluation
    pub fn extract_features(&self) -> Vec<f32> {
        let mut features = Vec::new();

        // Material balance
        let mut material = 0.0;
        for piece in &self.pieces {
            let value = piece.piece_type.value() * piece.color.scalar();
            material += value;
        }
        features.push(material);

        // Cycle counts (attack graph features)
        features.push(self.count_cycles(Color::White, 2));
        features.push(self.count_cycles(Color::Black, 2));
        features.push(self.count_cycles(Color::White, 3));
        features.push(self.count_cycles(Color::Black, 3));

        // Grade projections of position multivector
        for grade in 0..5 {
            let projected = self.position_multivector.grade(grade);
            features.push(projected.norm_squared());
        }

        features
    }

    /// Simple evaluation using geometric features
    pub fn evaluate(&self) -> f32 {
        let features = self.extract_features();

        // Simple linear combination (placeholder for ML model)
        let weights = [1.0, 0.1, -0.1, 0.05, -0.05, 0.01, 0.01, 0.01, 0.01, 0.01];
        let mut score = 0.0;
        for (feat, weight) in features.iter().zip(weights.iter()) {
            score += feat * weight;
        }

        score
    }
}

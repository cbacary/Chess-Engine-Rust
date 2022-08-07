use chess::{BitBoard, Board, ChessMove, MoveGen, MoveList, SquareAndBitBoard, EMPTY};
use chess::{NUM_PROMOTION_PIECES, PROMOTION_PIECES};

pub struct MoveIterator {
    moves: MoveList,
    index: usize,
    mask_index: usize,
    promotion_index: usize,
    iterator_mask: BitBoard,
    masks: Vec<BitBoard>,
}

impl MoveIterator {
    #[inline(always)]
    pub fn new_legal(board: &Board, masks: Vec<BitBoard>) -> MoveIterator {
        MoveIterator {
            moves: MoveGen::enumerate_moves(board),
            index: 0,
            mask_index: 0,
            masks: if masks.len() != 0 { masks } else { vec![EMPTY] },
            iterator_mask: !EMPTY,
            promotion_index: 0,
        }
    }

    /// Optionally pass in the first_move to check in the list of moves
    /// This MUST BE CALLED for iterator to work.
    pub fn set_first_mask(&mut self, first_move: Option<ChessMove>) {
        if let Some(m) = first_move {
            let source_sq = m.get_source();

            // We are basically going to set the first_move as the first_move
            // using an iterator mask

            println!("HERHE");

            // First convert the chess move to a mask
            let dest = BitBoard::from_square(m.get_dest());
            self.iterator_mask = dest;
            self.masks.insert(0, dest);

            let new_square_and_bb = SquareAndBitBoard::new(
                source_sq,
                dest,
                if m.get_promotion() == None {
                    false
                } else {
                    true
                },
            );

            for i in 0..self.moves.len() {
                if *self.moves[i].get_square() == source_sq {
                    // Because we do not want to check the same move twice
                    // we remove the move from the move list for this square
                    self.moves[i].xor(dest);

                    // After removing the move from the move list add the move
                    // back in at the beginning of move list
                    self.moves.insert(0, new_square_and_bb);
                    break;
                }
            }
        } else {
            self.set_iterator_mask(self.masks[0]);
        }
    }

    /// Function should really only be called by the next()
    /// iterator trait
    fn set_iterator_mask(&mut self, mask: BitBoard) {
        self.iterator_mask = mask;
        self.index = 0;

        // the iterator portion of this struct relies on the invariant that
        // the bitboards at the beginning of the moves[] array are the only
        // ones used.  As a result, we must partition the list such that the
        // assumption is true.

        // first, find the first non-used moves index, and store that in i
        let mut i = 0;
        while i < self.moves.len() && self.moves[i].get_bitboard() & self.iterator_mask != EMPTY {
            i += 1;
        }

        // next, find each element past i where the moves are used, and store
        // that in i.  Then, increment i to point to a new unused slot.
        for j in (i + 1)..self.moves.len() {
            if self.moves[j].get_bitboard() & self.iterator_mask != EMPTY {
                let backup = self.moves[i];
                self.moves[i] = self.moves[j];
                self.moves[j] = backup;
                i += 1;
            }
        }
    }
}

impl Iterator for MoveIterator {
    type Item = ChessMove;

    /// Find the next chess move.
    fn next(&mut self) -> Option<ChessMove> {
        if self.index >= self.moves.len()
            || self.moves[self.index].get_bitboard() & self.iterator_mask == EMPTY
        {
            // are we done?
            self.mask_index += 1;
            if self.mask_index >= self.masks.len() {
                None
            } else {
                self.set_iterator_mask(self.masks[self.mask_index]);
                self.next()
            }
        } else if *self.moves[self.index].get_promotion() {
            let moves = &mut self.moves[self.index];

            let dest = (moves.get_bitboard() & self.iterator_mask).to_square();

            // deal with potential promotions for this pawn
            let result = ChessMove::new(
                *moves.get_square(),
                dest,
                Some(PROMOTION_PIECES[self.promotion_index]),
            );
            self.promotion_index += 1;
            if self.promotion_index >= NUM_PROMOTION_PIECES {
                moves.xor(BitBoard::from_square(dest));
                //moves.bitboard ^= BitBoard::from_square(dest);
                self.promotion_index = 0;
                if moves.get_bitboard() & self.iterator_mask == EMPTY {
                    self.index += 1;
                }
            }
            Some(result)
        } else {
            // not a promotion move, so its a 'normal' move as far as this function is concerned
            let moves = &mut self.moves[self.index];
            // This is equal to saying the following
            // let dest = (moves.bitboard ^ BitBoard::from_square(move.square)).to_square();
            let dest = (moves.get_bitboard() & self.iterator_mask).to_square();

            moves.xor(BitBoard::from_square(dest));
            //moves.bitboard ^= BitBoard::from_square(dest);
            if moves.get_bitboard() & self.iterator_mask == EMPTY {
                self.index += 1;
            }
            Some(ChessMove::new(*moves.get_square(), dest, None))
        }
    }
}

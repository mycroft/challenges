#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            None
        } else {
            Some(ChessPosition{rank: rank, file: file})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { pos: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let diff = (self.pos.rank - other.pos.rank).abs() == (self.pos.file - other.pos.file).abs();

        self.pos.rank == other.pos.rank 
            || self.pos.file == other.pos.file
            || diff
    }
}

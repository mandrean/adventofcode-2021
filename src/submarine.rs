pub struct Position {
    pub depth: usize,
    pub horizontal: usize,
}

pub struct Submarine {
    pub position: Position,
    pub aim: usize,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine {
            position: Position {
                depth: 0,
                horizontal: 0,
            },
            aim: 0,
        }
    }
}

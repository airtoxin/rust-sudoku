const SIZE: usize = 9;

pub struct GameBoard {
    pub cells: [[u8; SIZE]; SIZE],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            cells: [[0; SIZE]; SIZE],
        }
    }

    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }
}

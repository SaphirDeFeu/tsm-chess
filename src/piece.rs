pub struct Piece {
    pub info: u8, // in the format 0b00001111 where the first 1 is the colour (0=black,1=white) and the 3 other ones are piece type
    pub text: char,
}

impl Piece {
    pub fn setText(&mut self) {
        let mut _text: char = ' ';
        match self.info & 0b00000111 {
            0 => _text = 'p',
            1 => _text = 'n',
            2 => _text = 'b',
            3 => _text = 'r',
            4 => _text = 'q',
            5 => _text = 'k',
            _ => _text = ' ',
        }
        if (self.info & 0b00001000) == 8 {
            let temp: Vec<_> = _text.to_uppercase().collect();
            _text = temp[0];
        }
        self.text = _text;
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "PIECE[info:{0:b};text:{1}]", &self.info, &self.text)
    }
}

pub fn init() {
    print!("piece.rs,");
}
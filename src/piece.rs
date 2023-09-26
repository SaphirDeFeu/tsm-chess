/**
 * * EXPLANATIONS
 * 'Piece' struct has two fields :
 *      - info | an unsigned byte ; only uses the last 4 bits
 *      - text | if unsure, provide a default ' ' value, and use the this.set_text() method
 * It also has three methods :
 *      - set_text() | sets the field 'text' using whatever the field 'info' has stored
 *      - get_type() | returns whatever type the piece is (last 3 bits of 'info' field)
 *      - get_color() | returns whatever color the piece is (4th from last bit of 'info' field) -- TRUE is WHITE ; FALSE is BLACK
 */

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Piece {
    pub info: u8, // in the format XXXXCTTT where the C is color (0=black,1=white) and T is piece type
    pub position: u8, // in the form XfRRRFFF where X we don't care, F is the file (0-7 a-h) and R is the rank (0-7 1-8) and f is 'null' flag
    pub text: char,
}

#[allow(dead_code)]
impl Piece {
    pub fn set_text(&mut self) {
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
            let temp: Vec<char> = _text.to_uppercase().collect();
            _text = temp[0];
        }
        self.text = _text;
    }

    pub fn get_type(&self) -> u8 {
        return &self.info & 0b00000111; // refer to "set_text()" function for piece type values
    }

    pub fn get_color(&self) -> bool {
        let mut _colorbeforeshift: u8 = &self.info & 0b00001000;
        return (_colorbeforeshift >> 3) != 0; // given that it's a single bit, then false=black and true=white
    }

    pub fn new(_info: Option<u8>, _position: Option<u8>) -> Piece {
        Piece {
            info: _info.unwrap_or(0b111),
            position: _position.unwrap_or(0),
            text: ' ',
        }
    }

    pub fn from_text(text: &str) -> u8 {
        let mut _color: u8 = 0b00000000;
        let _type: u8;

        if text.to_uppercase() == text { // if text is uppercase
            _color = 0b00001000;
        }

        match text {
            "p" | "P" => _type = 0,
            "n" | "N" => _type = 1,
            "b" | "B" => _type = 2,
            "r" | "R" => _type = 3,
            "q" | "Q" => _type = 4,
            "k" | "K" => _type = 5,
            _ => _type = 0b111,
        }

        return _type | _color;
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

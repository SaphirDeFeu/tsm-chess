use crate::piece;

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
pub struct ParsedFEN {
    pub board: [piece::Piece; 64],
    pub color: bool, // BLACK is FALSE and WHITE is TRUE
    pub castling_rights: u8, // under the form XXXXWWBB where X we don't care, W is white castle (0=queen,1=king,2=both), B is black castle (0=queen,1=king,2=both)
    pub en_passant: u8, // under the form XfFFFRRR where X we don't care, F is the file (0-7 a-h), R is the rank (0-7 1-8), and f is a flag to check if it's none
    pub halfmove: u8,
    pub fullmove: u16,
}

pub fn parse_fen(fen: &str) -> ParsedFEN {
    let mut splitfen: Vec<&str> = fen.split("").collect();
    let some_x = "";
    splitfen.retain(|&x| x != some_x);
    let mut char: &str;
    let mut _board: [piece::Piece; 64] = [piece::Piece {
        info: 0b111,
        position: 0b1000000,
        text: ' ',
    }; 64];
    let mut currentstage: u8 = 0;
    let mut _castle: u8 = 0;
    let mut _color: bool = false;
    let mut _passant: u8 = 64;
    let mut _half: u8 = 0;
    let mut _full: u16 = 0;
    let mut temp: String;

    let mut currentrank: u8 = 0;
    let mut currentfile: u8 = 0;

    for i in 0..splitfen.len() {
        char = splitfen[i];
        if char == " " {
            currentstage += 1;
            continue;
        }

        match currentstage {
            0 => {
                match char.parse::<u8>() {
                    Ok(value) => { // it's a number, so we need to start a clock
                        for _j in 0..value {
                            _board[((currentrank << 3) + currentfile) as usize] = piece::Piece {
                                info: 0b111,
                                position: ((currentrank << 3) + currentfile),
                                text: ' ',
                            };
                            currentfile += 1;
                        }
                        continue;
                    }
                    Err(_err) => {
                        // do nothing
                    }
                }

                // So it's not a number.
                if char == "/" {
                    currentrank += 1;
                    currentfile = 0;
                    continue;
                } else {
                    _board[((currentrank << 3) + currentfile) as usize] = piece::Piece {
                        info: piece::Piece::from_text(char),
                        position: ((currentrank << 3) + currentfile),
                        text: char.chars().next().unwrap(),
                    }
                }

                currentfile += 1;
            }
            1 => {
                if char == "w" {
                    _color = true;
                }
            }
            2 => {
                if (char == "Q") && (_castle & 0b1100) == 0b100 {
                    _castle += 4;
                }
                if (char == "q") && (_castle & 0b11) == 1 {
                    _castle += 1;
                }
                match char {
                    "K" => _castle += 4,
                    "k" => _castle += 1,
                    "Q" | "q" => continue,
                    "-" => continue,
                    _ => eprintln!("couldn't recognize castle rights part of fen string"),
                }
            }
            3 => {
                if (splitfen[i + 1] != " ") && (_passant == 64) { // if the letter after this is not a space, then we do :
                    temp = char.to_owned() + splitfen[i + 1];
                    _passant = from_string_to_pos(&temp);
                }
            }
            4 => {
                match char.parse::<u8>() {
                    Ok(value) => _half  = _half * 10 + value,
                    Err(err) => {
                        eprintln!("ERR PARSING STRING TO U8: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            5 => {
                match char.parse::<u16>() {
                    Ok(value) => _full  = _full * 10 + value,
                    Err(err) => {
                        eprintln!("ERR PARSING STRING TO U16: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                eprintln!("/!\\ PARSING FEN ERROR: FEN STAGE VALUE WENT OUT OF BOUNDS");
                std::process::exit(1);
            }
        }
    }
    return ParsedFEN {
        board: _board,
        color: _color,
        castling_rights: _castle,
        en_passant: _passant,
        halfmove: _half,
        fullmove: _full,
    }
}

pub fn convert_pos_to_string(pos: u8) -> String {
    if (pos & 0b01000000) == 64 {
        return String::from("-");
    }
    let nfile: u8 = pos & 0b00000111;
    let rank: u8 = (pos & 0b00111000) >> 3;
    let file: char;
    let frank: u8 = rank + 1;
    match nfile {
        1 => file = 'b',
        2 => file = 'c',
        3 => file = 'd',
        4 => file = 'e',
        5 => file = 'f',
        6 => file = 'g',
        7 => file = 'h',
        _ => file = 'a',
    }
    let sfile: String = String::from(file);
    let srank: String = frank.to_string();
    return String::from(sfile + &srank);
}

pub fn from_string_to_pos(pos: &str) -> u8 {
    if pos == "-" {
        return 0b01000000;
    }
    let mut eachchar: Vec<&str> = pos.split("").collect();
    let some_x = "";
    eachchar.retain(|&x| x != some_x);
    let file: u8;
    let rank: u8 = eachchar[1].parse().unwrap_or(8) - 1;
    match eachchar[0] {
        "a" => file = 0,
        "b" => file = 1,
        "c" => file = 2,
        "d" => file = 3,
        "e" => file = 4,
        "f" => file = 5,
        "g" => file = 6,
        "h" => file = 7,
        _ => file = 7,
    }
    return (rank << 3) + file;
}

pub fn init() {
    print!("position.rs,");
}

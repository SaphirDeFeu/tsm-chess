use crate::piece;

pub fn display(_board: [piece::Piece; 64], fullfen: &str) {
    println!("┌───┬───┬───┬───┬───┬───┬───┬───┐");
    for i in 0..64 {
        if i % 8 == 0 && i != 0 { // we have reached a new rank
            println!("│");
            println!("├───┼───┼───┼───┼───┼───┼───┼───┤	");
        }
        print!("│ {} ", _board[i].text);
    }
    println!("│\n└───┴───┴───┴───┴───┴───┴───┴───┘\n");
    println!("FEN: {}", fullfen);
}

pub fn init() {
    print!("d.rs,");
}
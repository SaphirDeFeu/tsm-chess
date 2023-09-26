mod piece;
mod position;
mod d;
mod uci;

fn main() {
    print!("TSMChess developed by TSMStudios | loaded modules (");
    piece::init();
    position::init();
    d::init();
    uci::init();
    println!(")");
    let mut s_cmd: String = String::new();
    let mut currentfen: String = String::from(position::STARTING_FEN);
    let mut currentparsedfen: position::ParsedFEN = position::parse_fen(&currentfen);
    let mut ready: &str = "";
    loop {
        std::io::stdin()
            .read_line(&mut s_cmd)
            .expect("FAILED TO READ LINE");
        let commandparts: Vec<&str> = s_cmd.split_whitespace().collect();
        let command: &str = commandparts[0];
        let args: &[&str] = &commandparts[1..];
        match command {
            "exit" | "quit" => std::process::exit(0),
            "position" => {
                match args[0] {
                    "startpos" => {
                        currentfen = String::from(position::STARTING_FEN);
                        currentparsedfen = position::parse_fen(&currentfen);
                    }
                    "fen" => {
                        let str_slice: &[&str] = &args[1..7];
                        let concatenated: String = str_slice.join(" ");
                        currentfen = concatenated;
                        currentparsedfen = position::parse_fen(&currentfen);
                    }
                    _ => println!("{}", command),
                }
            }
            "test" => { // testing functionalities (only available during snapshots or development commits)
                //println!("No testing functionalities available for this snapshot");
                if args[0] == "fen" {
                    currentparsedfen = position::parse_fen(position::STARTING_FEN);
                    println!("{:#?}", currentparsedfen);
                } else if args[0] == "pos" {
                    dbg!(position::convert_pos_to_string(0b00101011));
                    println!("{:b}", position::from_string_to_pos("c5"));
                }
            }
            "d" => d::display(currentparsedfen.board, &currentfen),
            "uci" => {
                uci::start();
            }
            "ucinewgame" => {
                ready = uci::newgame();
            }
            "isready" => {
                println!("{}", ready);
            }
            _ => {
                println!("Unknown command '{}'. Use help or ? for more information.", command);
            }
        }
        s_cmd = String::new();
    }
}

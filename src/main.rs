pub mod piece;

fn main() {
    print!("TSMChess developed by TSMStudios | loaded modules (");
    piece::init();
    println!(")");
    let mut s_cmd: String = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut s_cmd)
            .expect("FAILED TO READ LINE");
        let commandparts: Vec<&str> = s_cmd.split_whitespace().collect();
        let command: &str = commandparts[0];
        let args: &[&str] = &commandparts[1..];
        match command {
            "exit" | "quit" => std::process::exit(0),
            "test" => { // testing functionalities (only available during snapshots or development commits)
                if args[0] == "newpiece" {
                    let mut newpiece: piece::Piece = piece::Piece {
                        info: 0b00001011,
                        text: ' '
                    };
                    newpiece.setText();
                    println!("{}", newpiece);
                }
            }
            _ => {
                println!("Unknown command '{}'. Use help or ? for more information.", command);
            }
        }
        s_cmd = String::new();
    }
}

fn main() {
    print!("TSMChess developed by TSMStudios | loaded modules (");
    println!(")");
    let mut s_cmd: String = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut s_cmd)
            .expect("FAILED TO READ LINE");
        println!("{}", s_cmd);
        s_cmd = String::new();
    }
}

mod cpu;
mod window;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("É necessário informar a rom. Exemplo: cargo run roms/INVADERS");
        return;
    }

    window::execute(&args);
}
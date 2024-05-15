mod args;
mod errors;
mod player;
mod system;
mod utils;
mod waybar;
mod weather;

fn main() {
    match args::parse() {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
}

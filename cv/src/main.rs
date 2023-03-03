use std::env;


pub mod config;
pub mod gstate;
pub mod chrono;

fn main() {
    let config = config::Config::new(env::args()).unwrap();
    let mut gstate = gstate::GState::new(config).unwrap();
    println!("Civilization X\n");
    loop {
        gstate.run();
    }
}

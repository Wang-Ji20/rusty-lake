use std::env;

pub mod chrono;
pub mod config;
pub mod gstate;
pub mod terrain;

fn main() {
    let config = config::Config::new(env::args())
        .expect("usage ./cv: <verbose level> <day change duration> <land number>\n");
    let mut gstate = gstate::GState::new(config).unwrap();
    println!("Civilization X\n");
    loop {
        gstate.run();
    }
}

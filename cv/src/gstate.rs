/*!
 * Maintain global state for the simulator..
 * [`Chrono`], City, Pop..
 * G represents
 */
use std::thread::sleep;

use crate::{chrono::Chrono, config::Config, terrain::TerrainManager};

pub struct GState {
    config: Config,
    date: Chrono,
    terrain: TerrainManager,
}

impl GState {
    pub fn new(config: Config) -> Option<GState> {
        let tm = TerrainManager::new(&config)?;
        let c = Chrono::new(0, 1, 1)?;
        Some(GState {
            config: config,
            date: c,
            terrain: tm,
        })
    }

    pub fn run(&mut self) {
        print!("{}\n\n{}\n", self.date, self.terrain);
        self.date.proceed();
        sleep(self.config.sleep_duration);
    }
}

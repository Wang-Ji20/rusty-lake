/*!
 * Maintain global state for the simulator..
 * [`Chrono`], City, Pop..
 * G represents
 */
use std::thread::sleep;

use crate::{config::Config, chrono::Chrono};

pub struct GState {
  config: Config,
  date: Chrono,
}

impl GState {
    pub fn new(config: Config) -> Option<GState> {
      Some(GState { config: config, date: Chrono::new(0, 1, 1).unwrap() })
    }
    
    pub fn run(&mut self) {
      self.date.proceed();
      sleep(self.config.sleep_duration);
      println!("{}", self.date);
    }
}
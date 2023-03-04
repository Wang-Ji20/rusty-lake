use std::{env, time::Duration};

pub struct Config {
    pub verbose_level: i32,
    pub sleep_duration: Duration,
    pub land_num: u32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Option<Config> {
        args.next();
        let verbose_level: i32 = args.next()?.parse().ok()?;
        let sleep_ms: u64 = args.next()?.parse().ok()?;
        let sleep_duration = Duration::from_millis(sleep_ms);
        let land_num: u32 = args.next()?.parse().ok()?;
        Some(Config {
            verbose_level,
            sleep_duration,
            land_num,
        })
    }
}

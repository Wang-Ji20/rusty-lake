/*!
 * Chrono module provides DateTime system for this project.
 * We ignore reap year, assuming 365 days/year
 * 
 */

use std::fmt::Display;

pub struct Chrono {
  pub year: u64,
  pub month: u8,
  pub day: u8,
}

const MONTH_DAY_LIMIT: [u8; 13] = [
  0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
];

impl Chrono {
    pub fn new(year: u64, month: u8, day: u8) -> Option<Chrono> {
      if month > 12 || day > MONTH_DAY_LIMIT[month as usize] {
        None
      } else {
        Some(Chrono {
          year, month, day,
        })
      }
    }

    /**
     * proceed a new day
     * 
     * ```
     * let date1 = Chrono::new(2020, 1, 31);
     * date1.proceed();
     * 
     * assert_eq!(2, date1.month);
     * 
     * ```
     */
    pub fn proceed(&mut self) {
      self.day += 1;
      if self.day > MONTH_DAY_LIMIT[self.month as usize] {
        self.day = 1;
        self.month += 1;
      }
      if self.month > 12 {
        self.year += 1;
        self.month = 1;
      }
    }
    
}


impl Display for Chrono{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.year, self.month, self.day)
    }
}
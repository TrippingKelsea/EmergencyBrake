/*
 *  Kelsea Blackwell (c) 2023
 *  See LICENSE for licensing information.
 */

use std::collections::VecDeque;
use std::process;
use tracing::error;

trait EmergencyBrake {
    fn sample(&mut self, sample: bool);
    fn trigger(&self) -> bool;
}

#[derive(Clone, Debug, Default)]
pub struct EBrake {
    data: VecDeque<bool>,
    failures: usize,
    samples: usize,
    successes: usize,
    tolerance: usize,
}


impl EmergencyBrake for EBrake {
    fn sample(&mut self, sample: bool) {
        if self.data.len() == self.samples {
            match self.data.pop_front() {
                Some(true) => self.successes -= 1,
                Some(false) => self.failures -= 1,
                None => {},
            }
        }
        self.data.push_back(sample);
    }

    fn trigger(&self) -> bool {
        if self.data.len() < self.samples {
            return false;
        }

        if self.failures > self.tolerance {
            error!("Emergency brake triggered!");
            process::abort();
        }

        false
    }
}


#[cfg(test)]
mod test;
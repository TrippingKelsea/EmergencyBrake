/*
 *  Kelsea Blackwell (c) 2023
 *  See LICENSE for licensing information.
 */

#![deny(missing_docs)]

use std::collections::VecDeque;
use std::process;
use tracing::error;

trait EmergencyBrake {
    /// Insert a sample into the emergency brake.
    /// This will pop the oldest sample if the queue is full.
    /// `true` indicates a success, `false` indicates a failure.
    fn sample(&mut self, sample: bool);

    /// Returns false if the emergency brake has not been triggered.
    /// If the emergency brake has been triggered, the process will be aborted.
    fn trigger(&self) -> bool;
}


/// The emergency brake is a circular queue of boolean samples with a defined size and tolerance.
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

impl EBrake {
    /// Creates a new Emergency Brake with the given number of samples and tolerance.
    pub fn new(samples: usize, tolerance: usize) -> Self {
        EBrake {
            data: VecDeque::with_capacity(samples),
            failures: 0,
            samples: samples,
            successes: 0,
            tolerance: tolerance,
        }
    }
}


#[cfg(test)]
mod test;
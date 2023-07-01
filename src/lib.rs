//! eBrake creates a moving sample window of the last N samples. If the number of
//! failures in the sample window exceeds the threshold, the process or service
//! will be terminated. The sample window is a circular buffer, so the oldest
//! sample will be replaced by the newest sample.
//! 
//! # Examples
//! 
//! This will use the sample and trigger functions separately.
//! ```
//! use emergency_brake::*;
//! let sample_window_size = 25;
//! let failure_threshold = 3;
//! let mut ebrake = EBrake::new(sample_window_size, failure_threshold);
//! for _ in 0..sample_window_size {
//!    ebrake.add_sample(true);
//! }
//! assert_eq!(ebrake.trigger(), false);
//! ```
//! 
//! This will use the trigger_on_sample function.
//! ```
//! use emergency_brake::*;
//! let sample_window_size = 25;
//! let failure_threshold = 3;
//! let mut ebrake = EBrake::new(sample_window_size, failure_threshold);
//! for _ in 0..sample_window_size {
//!   ebrake.trigger_on_sample(true);
//! }
//! assert_eq!(ebrake.trigger(), false);
//! ```
//! 
//! 
//! Kelsea Blackwell (c) 2023
//! See LICENSE for licensing information.

#![deny(missing_docs)]

#[cfg(feature = "service_checker")]
use async_trait::async_trait;


use std::collections::VecDeque;
use std::process;

#[cfg(feature = "service_checker")]
use reqwest;

#[cfg(feature = "service_checker")]
use tokio;

use tracing::error;





/// The EmergencyBrake trait is the interface for the emergency brake.
pub trait EmergencyBrake {
    /// Insert a sample into the emergency brake.
    /// This will pop the oldest sample if the queue is full.
    /// `true` indicates a success, `false` indicates a failure.
    fn add_sample(&mut self, sample: bool);

    /// Returns false if the emergency brake has not been triggered.
    /// If the emergency brake has been triggered, the process will be aborted.
    fn trigger(&self) -> bool;

    /// Insert a sample and check if the emergency brake should be triggered.
    fn trigger_on_sample(&mut self, sample: bool) -> bool {
        self.add_sample(sample);
        self.trigger()
    }
}


/// The ServiceCheck trait is the interface for checking or monitoring a service.
#[cfg_attr(docsrs, doc(cfg(feature = "service_checker")))]
#[cfg(feature = "service_checker")]
#[async_trait]
pub trait ServiceChecker {
    /// Check if the service is running. This takes a URI as a parameter, and
    /// performs a basic HTTP GET request to the URI. If the request is successful,
    /// it will return true and assume the service is running, false otherwise.
    async fn check_service_endpoint(&self, uri: &str) -> bool;

    /// Similar to check_service_endpoint, but will check the service at a given
    /// interval. This will spawn a background thread and consume the current
    /// instance of the EBrake. If the service stops responding, the EBrake will
    /// be triggered and the process will be aborted.
    async fn watch_service_endpoint(mut self, uri: &'static str, interval: usize);
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
    fn add_sample(&mut self, sample: bool) {
        if self.data.len() == self.samples {
            match self.data.pop_front() {
                Some(true) => self.successes -= 1,
                Some(false) => self.failures -= 1,
                None => {},
            }
        }
        
        match sample {
            true => self.successes += 1,
            false => self.failures += 1,
        }

        self.data.push_back(sample);
    }

    fn trigger(&self) -> bool {
        if self.data.len() < self.tolerance {
            return false;
        }

        if self.failures > self.tolerance {
            error!("Emergency brake triggered!");
            process::abort();
        }

        false
    }

    fn trigger_on_sample(&mut self, sample: bool) -> bool {
        self.add_sample(sample);
        self.trigger()
    }
}



#[cfg(feature = "service_checker")]
#[async_trait]
impl ServiceChecker for EBrake {
    async fn check_service_endpoint(&self, uri: &str) -> bool {
        let client = reqwest::Client::new();
        let response = client.get(uri).send().await;
        match response {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    async fn watch_service_endpoint(mut self, uri: &'static str, interval: usize) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval as u64));
            loop {
                interval.tick().await;
                let result = self.check_service_endpoint(uri).await;
                self.trigger_on_sample(result);
            }
        });
    }
}


impl EBrake {
    /// Creates a new Emergency Brake with the given number of samples and tolerance.
    /// ```
    /// use emergency_brake::EBrake;
    /// let ebrake = EBrake::new(10, 3);
    /// ```
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


/// Test module for the Emergency Brake.
#[cfg(test)]
mod test;


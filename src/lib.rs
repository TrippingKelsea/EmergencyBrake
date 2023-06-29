/*
 *  Kelsea Blackwell (c) 2021
 *  See LICENSE for licensing information.
 */

use std::collections::VecDeque;


pub struct EBrake<T> {
    data: VecDeque<T>,
    failures: usize,
    samples: usize,
    successes: usize,
    tolerance: usize,
}



#[cfg(test)]
mod test;
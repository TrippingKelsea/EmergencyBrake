/*
 *  Kelsea Blackwell (c) 2023
 *  See LICENSE for licensing information.
 */

use super::*;

#[test]
/// Test that the emergency brake can be created with default values.
fn it_should_create_with_defaults() {
    let ebrake = EBrake::default();
    assert_eq!(ebrake.data.len(), 0);
    assert_eq!(ebrake.failures, 0);
    assert_eq!(ebrake.samples, 0);
    assert_eq!(ebrake.successes, 0);
    assert_eq!(ebrake.tolerance, 0);
}

#[test]
/// Test that the emergency brake returns false when not triggered.
fn it_should_return_false_when_not_triggered() {
}
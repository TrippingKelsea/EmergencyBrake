Emergency Brake
===

`emergency_brake` is a simple and easy to use process or service monitor that will immediately
terminate the execution of a process or service on a critical dependency failure.

# Usage

eBrake creates a moving sample window of the last N samples. If the number of
failures in the sample window exceeds the threshold, the process or service
will be terminated. The sample window is a circular buffer, so the oldest
sample will be replaced by the newest sample.

```
use emergency_brake::*;

fn main() {
    let sample_window_size = 25;
    let threshold = 3;
    let mut ebrake = EBrake::new(sample_window_size, threshold);
    loop:
        // Check service status
        let service_status: bool = check_service_status('service.foo.com');
        // Add the sample to the sample window and trigger if necessary
        ebrake.add_sample(service_status);
        ebrake.trigger();
        // Do something critical
        ...
}
```

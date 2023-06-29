eBrake
===

eBrake is a simple and easy to use process or service monitor that will immediately
terminate the execution of a process or service on a critical dependency failure.

# Usage

eBrake creates a moving sample window of the last N samples. If the number of
failures in the sample window exceeds the threshold, the process or service
will be terminated. The sample window is a circular buffer, so the oldest
sample will be replaced by the newest sample.



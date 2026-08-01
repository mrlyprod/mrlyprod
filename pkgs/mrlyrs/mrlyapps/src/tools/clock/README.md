# clock

A digital clock in hours, minutes and seconds, zero-padded to 09:05:03 style, on a 24-hour face that wraps at midnight. It keeps itself honest: the time comes from the system heartbeat, one tick per beat, and a clock that has never been set admits it with --:--:-- rather than a guess.

## Dials

- The clock ticks on its own; there is nothing to wind or set.
- Hours run 0 to 23, and the face wraps cleanly at 86400 seconds.
- With the shared *font* set to mrly, the readout is drawn as glyphs.
- The last known time is saved, so a reopened clock starts from what it knew.

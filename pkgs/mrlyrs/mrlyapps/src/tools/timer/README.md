# timer

A countdown and a stopwatch sharing one face. The countdown takes any span up to 24 hours and rings exactly once when the deadline passes - a single "time is up" and no more. The stopwatch counts up, banks its time across pauses, and collects laps. While either is running the app checks itself on every system heartbeat; when nothing runs, it goes quiet.

## Using

- Start a countdown in whole seconds, up to 86400, or set a duration in hours and minutes, up to 1440 minutes in all.
- **pause** holds the time still; **resume** picks up exactly where it left off.
- In stopwatch mode, **lap** stamps the elapsed time with a blip. Laps need the watch running.
- Switching modes, or **clear**, wipes everything.
- The whole state survives a reload - a paused stopwatch reopens still paused.
